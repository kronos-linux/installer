use crate::prelude::*;

pub fn format(target: &str) {
    info!("Formatting {} as ext4", target);
    shrun(&ShellCommand::new("mkfs.ext4").args(["-F", target]));
}

pub fn mount(target: &str) {
    info!("Mounting root filesystem");
    shrun(&ShellCommand::new("mount").args([target, "/mnt/gentoo"]));
}
