use crate::prelude::*;

pub fn format(target: &str, c: Config) -> Config {
    info!("Formatting {} as btrfs", target);
    let subvolume: bool = get_value(&c, "filesystem.btrfs.root.subvolume");
    let compress: bool = get_value(&c, "filesystem.btrfs.compression.enable");

    shrun(&ShellCommand::new("mkfs.btrfs").args(["-qf", target]));

    if subvolume {
        info!("Creating root subvolume");
    }

    if compress {
        info!("Setting up rootfs comrpession");
    }

    c
}
