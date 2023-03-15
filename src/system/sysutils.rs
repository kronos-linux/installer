use crate::prelude::*;

pub fn configure() {
    info!("Configuring utilities");
    shrun(&ShellCommand::new("eselect").args(["vi", "set", "nvim"]));
    shrun(&ShellCommand::new("eselect").args(["editor", "set", "vi"]));
}

pub fn install(c: &Config) {
    info!("Installing utilities");
    let gui = get_value(c, "gui.enable");

    let utilities = get_utils(gui);
    let args = [vec![String::from("-vnq")].as_slice(), utilities.as_slice()].concat();

    shrun(&ShellCommand::new("emerge").args(args));
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
    ];

    utils.iter().map(|x| x.to_string()).collect()
}
