use std::{
    env::{current_dir, set_current_dir},
    {fs::File, io::Write},
};

use crate::prelude::*;

pub fn configure(c: &Config) {
    info!("Adding users");

    let fs: String = get_value(c, "filesystem.type");
    let btrfs = fs == "btrfs";
    let z_enable = btrfs && get_value(c, "filesystem.btrfs.compression.enable");
    let z_algo: String = get_value(c, "filesystem.btrfs.compression.algo");
    let z_level: u8 = get_value(c, "filesystem.btrfs.compression.level");
    let z = (z_enable, z_algo, z_level);
    let user_subvolume: bool = btrfs && get_value(c, "filesystem.btrfs.user.subvolume");
    let user_snapshot: bool = user_subvolume && get_value(c, "filesystem.btrfs.user.snapshot");
    let admins: Vec<String> = get_value(c, "user.admins");
    let users: Vec<String> = get_value(c, "user.normal");
    let gui: bool = get_value(c, "gui.enable");

    let admin_groups = if !gui {
        "users,wheel"
    } else {
        "users,wheel,audio,video"
    };
    let user_groups = if !gui { "users" } else { "users,audio,video" };

    if !user_subvolume && !user_snapshot {
        vanilla(&admins, &users, &admin_groups, &user_groups);
        return;
    }

    subvolume_user(
        user_snapshot,
        &admins,
        &users,
        &admin_groups,
        &user_groups,
        z,
    );

    debug!("Users added");
}

fn subvolume_user(
    usnap: bool,
    admins: &[String],
    users: &[String],
    ag: &str,
    ug: &str,
    z: (bool, String, u8),
) {
    shrun(&ShellCommand::new("mkdir").args(["-p", "/tmp/btrfs_root"]));
    shrun(&ShellCommand::new("mount").args([
        "-o",
        "subvolid=5",
        "/dev/mapper/vg0-root",
        "/tmp/btrfs_root",
    ]));
    let cwd = current_dir().expect("Failed to read current directory");
    set_current_dir("/tmp/btrfs_root").expect("Failed to cd to temproot directory");
    create_subvols([admins, users].concat());
    set_current_dir(cwd).expect("Failed to return to root directory");

    edit_fstab([admins, users].concat(), z);
    shrun(&ShellCommand::new("umount").args(["/tmp/btrfs_root"]));
    shrun(&ShellCommand::new("mount").args(["-a"]));

    for admin in admins {
        shrun(&ShellCommand::new("useradd").args([
            "-d",
            &format!("/home/{}", admin),
            "-G",
            ag,
            admin,
        ]));
        shrun(&ShellCommand::new("passwd").args(["-de", admin]));
    }

    for user in users {
        shrun(&ShellCommand::new("useradd").args([
            "-d",
            &format!("/home/{}", user),
            "-G",
            ug,
            user,
        ]));
        shrun(&ShellCommand::new("passwd").args(["-de", user]));
    }

    for user in [admins, users].concat() {
        let files = shrun(&ShellCommand::new("find").args(["/etc/skel", "-name", ".[bp]*"]));
        for file in files.lines() {
            shrun(&ShellCommand::new("cp").args([file, &format!("/home/{}", user)]));
        }
    }

    shrun(&ShellCommand::new("rm").args(["-rf", "/tmp/btrfs_root"]));

    if usnap {
        user_snapshots([admins, users].concat());
    } else {
        for user in [admins, users].concat() {
            let homedir = format!("/home/{}", user);
            shrun(&ShellCommand::new("chown").args([
                "-R",
                &format!("{}:{}", user, user),
                &homedir,
            ]));
            shrun(&ShellCommand::new("chmod").args(["750", &homedir]));
        }
    }
}

fn user_snapshots(users: Vec<String>) {
    for user in users {
        let confname = format!("{}_home", user);
        let homedir = format!("/home/{}", user);
        let snapdir = homedir.to_string() + "/.snapshots";
        shrun(&ShellCommand::new("snapper").args(["-c", &confname, "create-config", &homedir]));
        crate::system::sysutils::snapper_conf(&confname);
        shrun(&ShellCommand::new("sed").args([
            "-i",
            "-e",
            &format!("s|ALLOW_USERS=\"\"|ALLOW_USERS=\"{}\"|g", user),
            &format!("/etc/snapper/configs/{}", &confname),
        ]));
        shrun(&ShellCommand::new("chown").args(["-R", &format!("{}:{}", user, user), &homedir]));
        shrun(&ShellCommand::new("chown").args(["-R", &format!("{}:{}", "root", user), &snapdir]));
        shrun(&ShellCommand::new("chmod").args(["750", &homedir]));
        shrun(&ShellCommand::new("chmod").args(["750", &snapdir]));
    }
}

fn edit_fstab(users: Vec<String>, z: (bool, String, u8)) {
    for user in users {
        let mut fstab = File::options()
            .write(true)
            .append(true)
            .open("/etc/fstab")
            .expect("Failed to open fstab");

        let subvol_list =
            shrun(&ShellCommand::new("btrfs").args(["subvolume", "list", "/tmp/btrfs_root"]));
        let toplevel = shrun(
            &ShellCommand::new("grep")
                .pipe_string(subvol_list)
                .args(["top level 5"]),
        );
        let subvol = shrun(
            &ShellCommand::new("grep")
                .pipe_string(toplevel)
                .args([&user]),
        );
        let subvolid: u32 = shrun(
            &ShellCommand::new("awk")
                .pipe_string(subvol)
                .args(["{print $2}"]),
        )
        .replace("\n", "")
        .parse()
        .expect("Failed to parse subvolid");
        let (z_enable, z_algo, z_level) = z.clone();
        let mountopts = if z_enable {
            format!(
                "subvolid={},compress={}:{},defaults,noatime",
                subvolid, z_algo, z_level
            )
        } else {
            format!("subvolid={},defaults,noatime", subvolid)
        };

        writeln!(
            fstab,
            "/dev/mapper/vg0-root\t/home/{}\tbtrfs\t{}\t0 0",
            user, mountopts
        )
        .expect("Failed to write to fstab");
    }
}

fn create_subvols(users: Vec<String>) {
    for user in users {
        shrun(&ShellCommand::new("btrfs").args(["subvolume", "create", &user]));
        shrun(&ShellCommand::new("mkdir").args(["-p", &format!("/home/{}", &user)]));
    }
}

fn vanilla(admins: &[String], users: &[String], ag: &str, ug: &str) {
    for admin in admins {
        let homedir = format!("/home/{}", admin);
        shrun(&ShellCommand::new("useradd").args(["-m", "-G", ag, admin]));
        shrun(&ShellCommand::new("passwd").args(["-de", admin]));
        shrun(&ShellCommand::new("chmod").args(["750", &homedir]));
    }

    for user in users {
        let homedir = format!("/home/{}", user);
        shrun(&ShellCommand::new("useradd").args(["-m", "-G", ug, user]));
        shrun(&ShellCommand::new("passwd").args(["-de", user]));
        shrun(&ShellCommand::new("chmod").args(["750", &homedir]));
    }
}
