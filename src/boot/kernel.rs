use std::{
    env::{current_dir, set_current_dir},
    fs::File,
    io::Write,
};

use crate::prelude::*;

pub fn configure() {
    info!("Emerging kernel sources");
    portage_config();

    shrun(&ShellCommand::new("emerge").args([
        "-vq",
        "sys-kernel/linux-firmware",
        "=sys-kernel/gentoo-sources-6.1.19",
    ]));

    info!("Compiling kernel");
    kompile();
}

fn kompile() {
    let konfig = include_str!("resource/konfig");
    let cwd = current_dir().expect("Failed to read current directory");
    set_current_dir("/usr/src/linux").expect("Failed to cd to linux directory");

    shrun(&ShellCommand::new("make").args(["clean"]));
    shrun(&ShellCommand::new("make").args(["mrproper"]));

    writeln!(
        File::create(".config").expect("Failed to open package.use"),
        "{}",
        konfig
    )
    .expect("Failed to write to package.use");

    let nproc: i16 = shrun(&ShellCommand::new("nproc"))
        .replace("\n", "")
        .parse()
        .expect("Failed to parse nproc");
    shrun(&ShellCommand::new("make").args([format!("-j{}", nproc)]));

    shrun(&ShellCommand::new("make").args(["install"]));
    shrun(&ShellCommand::new("make").args(["modules_install"]));

    set_current_dir(cwd).expect("Failed to return to root directory");
}

fn portage_config() {
    writeln!(
        File::options()
            .append(true)
            .open("/etc/portage/package.use")
            .expect("Failed to open package.use"),
        "sys-kernel/gentoo-sources symlink"
    )
    .expect("Failed to write to package.use");

    writeln!(
        File::options()
            .append(true)
            .open("/etc/portage/package.license")
            .expect("Failed to open package.license"),
        "sys-kernel/linux-firmware linux-fw-redistributable no-source-code"
    )
    .expect("Failed to write to package.license");
}
