use std::{fs::File, io::Write};

use crate::prelude::*;

pub fn configure(c: &Config) {
    info!("Configuring fstab");

    let boot_target: String = get_value(&c, "disk.esp");
    let swap = get_value(&c, "disk.swap.enable");

    let mut fstab = File::create("/etc/fstab").expect("Failed to open fstab");

    add_boot_line(&boot_target, &mut fstab);

    if get_value::<String>(c, "filesystem.type") == "btrfs" {
        let mopts: String = get_value(c, "filesystem.mountopts");
        let mopts = String::from(",") + &mopts;
        add_root_line(&mopts, "btrfs", &mut fstab);
    } else {
        add_root_line("", "ext4", &mut fstab);
    }

    if swap {
        add_swap_line(&mut fstab);
    }

    write!(fstab, "\n").expect("Could not write to fstab");

    debug!(
        "Configured fstab:\n{}",
        shrun(&ShellCommand::new("cat").args(["/etc/fstab"]))
    );
}

fn add_boot_line(target: &str, f: &mut File) {
    let sep = String::from("\t");
    let uuid = shrun(&ShellCommand::new("blkid").args(["-s", "UUID", "-o", "value", target]))
        .replace("\n", "");
    let fuuid = String::from("UUID=") + &uuid;
    let opts = "noauto,noatime";

    let bl = fuuid + &sep + "/boot" + &sep + "vfat" + &sep + opts + &sep + "0 2";

    write!(f, "{}\n", bl).expect("Could not write to fstab");
}

fn add_root_line(mopts: &str, fs: &str, f: &mut File) {
    let sep = String::from("\t");
    let rp = String::from("/dev/mapper/vg0-root");

    let opts = String::from("defaults,noatime") + mopts;
    let check = if fs != "btrfs" { "0 1" } else { "0 0" };

    let rl = rp + &sep + "/" + &sep + fs + &sep + &opts + &sep + check;

    write!(f, "{}\n", rl).expect("Could not write to fstab");
}

fn add_swap_line(f: &mut File) {
    let sep = String::from("\t");
    let sp = String::from("/dev/mapper/vg0-swap");

    let opts = String::from("sw");
    let check = "0 0";

    let sl = sp + &sep + "none" + &sep + "swap" + &sep + &opts + &sep + check;

    write!(f, "{}\n", sl).expect("Could not write to fstab");
}
