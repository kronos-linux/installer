use crate::prelude::*;
use std::{fs::File, io::Write};

pub fn configure(c: &Config) {
    info!("Configuring utilities");
    shrun(&ShellCommand::new("eselect").args(["vi", "set", "nvim"]));
    shrun(&ShellCommand::new("eselect").args(["editor", "set", "vi"]));

    info!("Configuring services");

    let gui = get_value(c, "gui.enable");
    let snapper_root: bool = get_value::<String>(c, "filesystem.type") == "btrfs"
        && get_value(c, "filesystem.btrfs.root.subvolume")
        && get_value(c, "filesystem.btrfs.root.snapshot");

    shrun(&ShellCommand::new("rc-update").args(["add", "sysklogd", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "cronie", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "chronyd", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "sshd", "default"]));

    if snapper_root {
        snapper_setup();
        snapper_conf("root");
    }

    if gui {
        shrun(&ShellCommand::new("rc-update").args(["add", "NetworkManager", "default"]));
    } else {
        shrun(&ShellCommand::new("rc-update").args(["add", "dhcpcd", "default"]));
    }

    doas_conf();
    zram_conf();
    if get_value(c, "disk.discard.enable") {
        trim_conf();
    }
    cgroups_conf();
}

pub fn install(c: &Config) {
    info!("Installing utilities");
    let gui = get_value(c, "gui.enable");
    let snapper_root: bool = get_value::<String>(c, "filesystem.type") == "btrfs"
        && get_value(c, "filesystem.btrfs.root.subvolume")
        && get_value(c, "filesystem.btrfs.root.snapshot");

    let utilities = get_utils(gui);
    let args = [vec![String::from("-vnq")].as_slice(), utilities.as_slice()].concat();
    let args = if snapper_root {
        [args, vec![String::from("app-backup/snapper")]].concat()
    } else {
        args
    };

    let htop_lm = "sys-process/htop lm-sensors\n";
    let dash = "app-alternatives/sh -bash dash\n";
    let lvm = "sys-fs/lvm2 lvm\n";

    write!(
        File::create("/etc/portage/package.use").expect("Failed to open paockage.use"),
        "{}{}{}",
        htop_lm,
        dash,
        lvm
    )
    .expect("Failed to write package.use");

    shrun(&ShellCommand::new("emerge").args(args));

    add_kronological();
    tetrahedron_install();
}

fn trim_conf() {
    let trim = include_str!("resources/trim.stop.sh");

    writeln!(
        File::create("/etc/local.d/trim.stop").expect("Failed to create trim.stop"),
        "{}",
        trim
    )
    .expect("Failed to write to zram.stop");

    shrun(&ShellCommand::new("chmod").args(["744", "/etc/local.d/trim.stop"]));
}

fn cgroups_conf() {
    shrun(&ShellCommand::new("sed").args([
        "-i",
        "-e",
        "s|^#rc_cgroup_mode=\".*\"|rc_cgroup_mode=\"unified\"|g",
        "-e",
        "s|^#rc_cgroup_controllers=\".*\"|rc_cgroup_controllers=\"cpuset cpu io memory hugetlb pids\"|g",
        "/etc/rc.conf",
    ]));
}

fn zram_conf() {
    let start = include_str!("resources/zram.start.sh");
    let stop = include_str!("resources/zram.stop.sh");

    writeln!(
        File::create("/etc/local.d/zram.start").expect("Failed to create zram.start"),
        "{}",
        start
    )
    .expect("Failed to write to zram.start");

    writeln!(
        File::create("/etc/local.d/zram.stop").expect("Failed to create zram.stop"),
        "{}",
        stop
    )
    .expect("Failed to write to zram.stop");

    shrun(&ShellCommand::new("chmod").args(["744", "/etc/local.d/zram.start"]));
    shrun(&ShellCommand::new("chmod").args(["744", "/etc/local.d/zram.stop"]));
}

fn doas_conf() {
    writeln!(
        File::create("/etc/doas.conf").expect("Failed to open doas.conf"),
        "permit :wheel"
    )
    .expect("Failed to write to doas.conf");

    writeln!(
        File::options()
            .write(true)
            .append(true)
            .open("/etc/skel/.bashrc")
            .expect("Failed to open skel bashrc"),
        "\ncomplete -cf doas"
    )
    .expect("Failed to write to skel bashrc");
}

fn snapper_setup() {
    match ShellCommand::new("rc-service")
        .args(["dbus", "start"])
        .run()
    {
        _ => (),
    }
    shrun(&ShellCommand::new("rc-update").args(["add", "dbus", "default"]));
    shrun(&ShellCommand::new("snapper").args(["-c", "root", "create-config", "/"]));
}

pub fn snapper_conf(tconf: &str) {
    shrun(&ShellCommand::new("sed").args([
        "-i",
        "-e",
        "s|TIMELINE_LIMIT_HOURLY=.*|TIMELINE_LIMIT_HOURLY=\"24\"|g",
        "-e",
        "s|TIMELINE_LIMIT_DAILY=.*|TIMELINE_LIMIT_DAILY=\"14\"|g",
        "-e",
        "s|TIMELINE_LIMIT_WEEKLY=.*|TIMELINE_LIMIT_WEEKLY=\"4\"|g",
        "-e",
        "s|TIMELINE_LIMIT_MONTHLY=.*|TIMELINE_LIMIT_MONTHLY=\"2\"|g",
        "-e",
        "s|TIMELINE_LIMIT_YEARLY=.*|TIMELINE_LIMIT_YEARLY=\"0\"|g",
        &format!("/etc/snapper/configs/{}", tconf),
    ]));
}

fn tetrahedron_install() {
    shrun(&ShellCommand::new("emerge").args(["-vnq", "dev-lang/rust-bin", "sys-apps/tetrahedron"]));
}

fn add_kronological() {
    info!("Adding Kronological repository");

    shrun(&ShellCommand::new("eselect").args([
        "repository",
        "add",
        "kronological",
        "git",
        "https://git.temp.hyprlab.net/KRONOS/kronological.git",
    ]));

    shrun(&ShellCommand::new("emaint").args(["-r", "kronological", "sync"]));
}

fn select_netd<'a>(gui: bool) -> &'a str {
    if gui {
        "net-misc/networkmanager"
    } else {
        "net-misc/dhcpcd"
    }
}

fn get_utils(gui: bool) -> Vec<String> {
    let networkd = select_netd(gui);

    let utils = [
        networkd,
        "www-client/links",
        "sys-process/nmon",
        "app-shells/dash",
        "app-shells/bash-completion",
        "sys-process/htop",
        "net-analyzer/bmon",
        "app-editors/neovim",
        "sys-fs/ncdu",
        "app-misc/tmux",
        "app-eselect/eselect-vi",
        "app-editors/nano",
        "app-alternatives/sh",
        "app-misc/neofetch",
        "app-crypt/gnupg",
        "app-admin/sysklogd",
        "sys-process/cronie",
        "sys-apps/mlocate",
        "net-misc/chrony",
        "sys-fs/e2fsprogs",
        "sys-fs/dosfstools",
        "sys-fs/btrfs-progs",
        "sys-fs/ntfs3g",
        "app-admin/doas",
        "app-eselect/eselect-repository",
        "app-portage/gentoolkit",
        "dev-vcs/git",
        "sys-fs/lvm2",
        "sys-fs/cryptsetup",
    ];

    utils.iter().map(|x| x.to_string()).collect()
}
