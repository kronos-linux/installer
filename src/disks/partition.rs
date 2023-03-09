use crate::prelude::*;

pub fn base(bootd: &str, rootd: &str) -> (String, String, String) {
    if bootd == rootd {
        partition_rb(rootd);
    } else {
        partition_b(bootd);
        partition_r(rootd);
    }

    ("".into(), "".into(), "".into())
}

fn partition_rb(target: &str) {
    info!("Partitioning {} as the boot and root drive", target);

    // Set string to pass into fdisk
    let sin = "g\nn\n\n2048\n+1M\nt\n4\nn\n\n\n+256M\nt\n\n1\nn\n\n\n\nw\n";

    let fd_part = ShellCommand::new("fdisk").pipe_string(sin).args([&target]);
    shrun(&fd_part);
}

fn partition_b(target: &str) {
    info!("Partitioning {} as the boot drive", target);

    // Set string to pass into fdisk
    let sin = "g\nn\n\n2048\n+1M\nt\n4\nn\n\n\n+256M\nt\n\n1\nw\n";

    let fd_part = ShellCommand::new("fdisk").pipe_string(sin).args([&target]);
    shrun(&fd_part);
}

fn partition_r(target: &str) {
    info!("Partitioning {} as the root drive", target);

    // Set string to pass into fdisk
    let sin = "g\nn\n\n\n\nw\n";

    let fd_part = ShellCommand::new("fdisk").pipe_string(sin).args([&target]);
    shrun(&fd_part);
}
