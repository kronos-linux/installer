use crate::prelude::*;

pub fn format(target: &str) {
    info!("Formatting {} as ext4", target);
    shrun(&ShellCommand::new("mkfs.ext4").args(["-F", target]));
}
