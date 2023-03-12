use crate::prelude::*;

mod btrfs;
mod ext4;

pub fn configure(c: Config) -> Config {
    info!("Configuring the filesystem");

    let root_volume: String = get_value(&c, "disk.root_volume");
    let fs: String = get_value(&c, "filesystem.type");

    shrun(&ShellCommand::new("mkdir").args(["-p", "/mnt/gentoo"]));

    if fs == "ext4" {
        ext4::format(&root_volume);
        ext4::mount(&root_volume);
        return c;
    } else if fs == "btrfs" {
        let c = btrfs::format(&root_volume, c);
        btrfs::mount(&root_volume, &c);
        return c;
    } else {
        Error::Config("Improper filesystem supplied in config".into()).handle()
    }
}
