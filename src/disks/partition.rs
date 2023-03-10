use crate::prelude::*;

pub fn base(bootd: &str, rootd: &str) -> (String, String, String) {
    if bootd == rootd {
        partition_rb(rootd);
    } else {
        partition_b(bootd);
        partition_r(rootd);
    }

    let (bb, esp) = read_boot(bootd);
    let rd = read_root(rootd);

    debug!("\nBIOS boot: {}\nESP: {}\nRoot partition: {}", bb, esp, rd);
    (bb, esp, rd)
}

fn read_boot(target: &str) -> (String, String) {
    let bootd_result = shrun(&ShellCommand::new("fdisk").args(["-l", target]));

    // Set the BIOS boot and ESP targets
    let grep_bb = shrun(
        &ShellCommand::new("grep")
            .pipe_string(&bootd_result)
            .args(["BIOS boot"]),
    );
    let grep_esp = shrun(
        &ShellCommand::new("grep")
            .pipe_string(&bootd_result)
            .args(["EFI System"]),
    );

    let bb = Vec::from_iter(grep_bb.split_whitespace())[0].to_string();
    let esp = Vec::from_iter(grep_esp.split_whitespace())[0].to_string();

    (bb, esp)
}

fn read_root(target: &str) -> String {
    let rootd_result = shrun(&ShellCommand::new("fdisk").args(["-l", target]));
    let grep = shrun(
        &ShellCommand::new("grep")
            .pipe_string(&rootd_result)
            .args(["Linux filesystem"]),
    );

    let root_part = Vec::from_iter(grep.split_whitespace())[0].to_string();

    root_part
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
