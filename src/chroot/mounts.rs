use crate::prelude::*;

pub fn relevant() {
    info!("Remounting /dev /sys /proc /run");
    shrun(&ShellCommand::new("mount").args(["--types", "proc", "/proc", "/mnt/gentoo/proc"]));
    shrun(&ShellCommand::new("mount").args(["--rbind", "/sys", "/mnt/gentoo/sys"]));
    shrun(&ShellCommand::new("mount").args(["--make-rslave", "/mnt/gentoo/sys"]));
    shrun(&ShellCommand::new("mount").args(["--rbind", "/dev", "/mnt/gentoo/dev"]));
    shrun(&ShellCommand::new("mount").args(["--make-rslave", "/mnt/gentoo/dev"]));
    shrun(&ShellCommand::new("mount").args(["--bind", "/run", "/mnt/gentoo/run"]));
    shrun(&ShellCommand::new("mount").args(["--make-slave", "/mnt/gentoo/run"]));
}
