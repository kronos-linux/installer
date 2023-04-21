use crate::prelude::*;

static SEP: &str = " ";

pub fn configure(c: &Config) {
    info!("Configuring kernel command line");
    let kcmd = build_kcmd(c);

    emerge_grub();

    shrun(&ShellCommand::new("sed").args([
        "-i",
        "-e",
        &format!(
            "s|#GRUB_CMDLINE_LINUX_DEFAULT=\".*\"|GRUB_CMDLINE_LINUX_DEFAULT=\"{}\"|g",
            kcmd
        ),
        "/etc/default/grub",
    ]));

    install_grub(c);
}

fn emerge_grub() {
    info!("Emerging GRUB");
    shrun(&ShellCommand::new("emerge").args(["-vnq", "sys-boot/grub"]));
}

fn install_grub(c: &Config) {
    let bios_dev: String = get_value(c, "disk.boot");
    shrun(&ShellCommand::new("grub-install").args(["--target=i386-pc", &bios_dev]));
    shrun(&ShellCommand::new("grub-install").args([
        "--target=x86_64-efi",
        "--efi-directory=/boot",
        "--removable",
    ]));

    shrun(&ShellCommand::new("grub-mkconfig").args(["-o", "/boot/grub/grub.cfg"]));
}

fn build_kcmd(c: &Config) -> String {
    let kcmd_args = String::from("console=tty1");

    let pvd = shrun(&ShellCommand::new("pvdisplay"));
    let pvuuid = shrun(&ShellCommand::new("grep").pipe_string(pvd).args(["PV UUID"]));
    let pvuuid = shrun(
        &ShellCommand::new("sed")
            .pipe_string(pvuuid)
            .args(["-e", "s|  PV UUID.* ||g"]),
    )
    .replace("\n", "");

    let kcmd_args = kcmd_args + SEP + "irfs.root_pv_uuid=UUID=" + &pvuuid;

    let kcmd_args = if get_value(c, "disk.encryption.enable") {
        let rp: String = get_value(c, "disk.root_partition");
        let blkid = shrun(&ShellCommand::new("blkid").args(["-s", "UUID", "-o", "value", &rp]))
            .replace("\n", "");
        kcmd_args + SEP + "irfs.crypt_uuid=UUID=" + &blkid
    } else {
        kcmd_args
    };

    let kcmd_args = if get_value::<String>(c, "filesystem.type") == "btrfs"
        && get_value(c, "filesystem.btrfs.root.subvolume")
    {
        let svid: String = get_value(c, "filesystem.btrfs.root_subvolid");
        kcmd_args + SEP + "irfs.btrfs_subvol_id=" + &svid
    } else {
        kcmd_args
    };

    let kcmd_args = if get_value(c, "disk.discard.enable") {
        kcmd_args + SEP + "irfs.root_dev_discard=true"
    } else {
        kcmd_args
    };

    kcmd_args
}
