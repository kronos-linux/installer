use crate::prelude::*;
use std::{fs::File, io::Write};

pub fn configure(c: &Config) {
    info!("Configuring utilities");
    shrun(&ShellCommand::new("eselect").args(["vi", "set", "nvim"]));
    shrun(&ShellCommand::new("eselect").args(["editor", "set", "vi"]));

    info!("Configuring services");

    let gui = get_value(c, "gui.enable");
    shrun(&ShellCommand::new("rc-update").args(["add", "sysklogd", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "cronie", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "chronyd", "default"]));
    shrun(&ShellCommand::new("rc-update").args(["add", "sshd", "default"]));

    if gui {
        shrun(&ShellCommand::new("rc-update").args(["add", "NetworkManager", "default"]));
    } else {
        shrun(&ShellCommand::new("rc-update").args(["add", "dhcpcd", "default"]));
    }
}

pub fn install(c: &Config) {
    info!("Installing utilities");
    let gui = get_value(c, "gui.enable");

    let utilities = get_utils(gui);
    let args = [vec![String::from("-vnq")].as_slice(), utilities.as_slice()].concat();

    let mut p_use = File::create("/etc/portage/package.use").expect("Failed to open paockage.use");
    let htop_lm = "sys-process/htop lm-sensors\n";
    let dash = "app-alternatives/sh -bash dash\n";
    let lvm = "sys-fs/lvm2 lvm\n";

    write!(p_use, "{}{}{}", htop_lm, dash, lvm).expect("Failed to write package.use");

    shrun(&ShellCommand::new("emerge").args(args));

    add_kronological();
    tetrahedron_install();
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

fn get_utils(gui: bool) -> Vec<String> {
    let networkd = if gui {
        "net-misc/networkmanager"
    } else {
        "net-misc/dhcpcd"
    };

    let utils = [
        networkd,
        "www-client/links",
        "sys-process/nmon",
        "app-shells/dash",
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
        "app-portage/layman",
        "app-eselect/eselect-repository",
        "app-portage/gentoolkit",
        "dev-vcs/git",
        "sys-fs/lvm2",
        "sys-fs/cryptsetup",
    ];

    utils.iter().map(|x| x.to_string()).collect()
}
