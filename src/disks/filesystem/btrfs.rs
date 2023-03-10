use crate::prelude::*;

pub fn format(target: &str, c: Config) -> Config {
    info!("Formatting {} as btrfs", target);
    let subvolume: bool = get_value(&c, "filesystem.btrfs.root.subvolume");
    let compress: bool = get_value(&c, "filesystem.btrfs.compression.enable");

    shrun(&ShellCommand::new("mkfs.btrfs").args(["-qf", target]));

    let mut c = c;

    if subvolume {
        info!("Creating root subvolume");

        shrun(&ShellCommand::new("mkdir").args(["-p", "/tmp/rootfs"]));
        shrun(&ShellCommand::new("mount").args([target, "/tmp/rootfs"]));
        shrun(&ShellCommand::new("btrfs").args(["subvolume", "create", "/tmp/rootfs/root"]));
        let root_subvolid_info =
            shrun(&ShellCommand::new("btrfs").args(["subvolume", "list", "/tmp/rootfs/root"]));
        let root_subvolid_info: Vec<&str> = root_subvolid_info.split_whitespace().collect();
        let root_subvolid = root_subvolid_info[1];
        debug!("Root subvolume id: {}", root_subvolid);
        c = add_value(
            c,
            "filesystem.btrfs.root_subvolid",
            root_subvolid
                .parse::<i64>()
                .expect("Failed subvolid parsing"),
        );
        shrun(&ShellCommand::new("umount").args(["/tmp/rootfs"]));
    }

    let mountopts = if subvolume {
        format!(
            "subvolid={}",
            get_value::<String>(&c, "filesystem.btrfs.root_subvolid")
        )
    } else {
        "".into()
    };
    let mountopts = if compress {
        let algo: String = get_value(&c, "filesystem.btrfs.compression.algo");
        let level: String = get_value(&c, "filesystem.btrfs.compression.level");
        mountopts.clone()
            + if mountopts != "" { "," } else { "" }
            + &format!("compress={}:{}", algo, level)
    } else {
        mountopts
    };
    debug!("Btrfs mountopts: {}", mountopts);
    let c = add_value(c, "filesystem.mountopts", mountopts);

    c
}

pub fn mount(target: &str, c: &Config) {
    info!("Mounting root filesystem");
    let mountopts: String = get_value(c, "filesystem.mountopts");
    shrun(&ShellCommand::new("mount").args(["-o", &mountopts, target, "/mnt/gentoo"]));
}
