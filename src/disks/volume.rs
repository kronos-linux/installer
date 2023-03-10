use crate::prelude::*;

pub fn root_container(c: Config) -> Config {
    let root_container: String = get_value(&c, "disk.root_container");
    info!("Creating volumes on root container {}", root_container);

    shrun(&ShellCommand::new("mkfs.vfat").args([&root_container]));
    shrun(&ShellCommand::new("pvcreate").args(["-f", &root_container]));
    shrun(&ShellCommand::new("vgcreate").args(["vg0", &root_container]));

    let free = ShellCommand::new("free").args(["-gt", "--si"]);
    let awk = ShellCommand::new("awk").args(["{print $2}"]);

    let mt = shrun(&awk.pipe_stdout(free).expect(""));
    let mt: u16 = match mt.lines().collect::<Vec<&str>>()[1].parse() {
        Ok(v) => v,
        Err(e) => Error::Generic(e.to_string()).handle(),
    };

    debug!("Memory detected: {}G", mt);
    let c = add_value(c, "system.memtotal", mt);

    if get_value(&c, "disk.swap.enable") {
        let (vol, swap) = create_swapped_lvm(get_value(&c, "system.memtotal"));
        let c = add_value(c, "disk.swap_volume", swap);
        let c = add_value(c, "disk.root_volume", vol);

        shrun(&ShellCommand::new("vgchange").args(["--available", "y"]));
        return c;
    } else {
        let vol = create_lvm();
        let c = add_value(c, "disk.root_volume", vol);

        shrun(&ShellCommand::new("vgchange").args(["--available", "y"]));
        return c;
    }
}

fn create_lvm() -> String {
    info!("Creating root volume");
    let arg = ["-y", "--extents", "100%FREE", "--name", "root", "vg0"];
    shrun(&ShellCommand::new("lvcreate").args(arg));
    "/dev/mapper/vg0-root".into()
}

fn create_swapped_lvm(mem: u16) -> (String, String) {
    let swap_size: u16 = if mem < 3 {
        6
    } else if mem > 8 {
        16
    } else {
        2 * mem
    };

    info!("Creating swap volume");
    shrun(&ShellCommand::new("lvcreate").args([
        "-y",
        "--size",
        &format!("{}G", swap_size),
        "--name",
        "swap",
        "vg0",
    ]));

    info!("Creating root volume");
    let arg = ["-y", "--extents", "100%FREE", "--name", "root", "vg0"];
    shrun(&ShellCommand::new("lvcreate").args(arg));
    ("/dev/mapper/vg0-root".into(), "/dev/mapper/vg0-swap".into())
}
