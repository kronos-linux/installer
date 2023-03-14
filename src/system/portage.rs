use crate::prelude::*;

use std::{fs::OpenOptions, io::Write, thread};

pub fn configure() -> thread::JoinHandle<()> {
    info!("Configuring portage");
    sync_repo();

    info!("Updating make.conf");
    shrun(&ShellCommand::new("emerge").args([
        "-vnq",
        "app-portage/cpuid2cpuflags",
        "app-portage/mirrorselect",
    ]));
    restructure_portage_dir();
    cpuflags();
    thread::spawn(|| mirrors())
}

fn sync_repo() {
    shrun(&ShellCommand::new("emerge-webrsync"));

    match ShellCommand::new("emerge").args(["--sync", "-q"]).run() {
        Ok(_) => debug!("Synced repository"),
        Err(_) => warn!("Failed to sync repository; Ignoring..."),
    }
}

fn mirrors() {
    let mirrors = shrun(&ShellCommand::new("mirrorselect").args(["-D", "-b25", "-s3", "-o"]))
        .replace("\n", "");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/portage/make.conf")
        .expect("Could not open make.conf");
    write!(file, "{}\n", mirrors).expect("Could not write make.conf");

    let content = shrun(&ShellCommand::new("cat").args(["/etc/portage/make.conf"]));
    debug!("Updated make.conf:\n{}", content);
}

fn cpuflags() {
    let binding = shrun(&ShellCommand::new("cpuid2cpuflags"));
    let cpuid2cpuflags: Vec<&str> = binding.split(": ").collect();
    let cpuflags = format!("CPU_FLAGS_X86=\"{}\"", cpuid2cpuflags[1].replace("\n", ""));

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/portage/make.conf")
        .expect("Could not open make.conf");
    write!(file, "{}\n", cpuflags).expect("Could not write make.conf");

    let content = shrun(&ShellCommand::new("cat").args(["/etc/portage/make.conf"]));
    debug!("Updated make.conf:\n{}", content);
}

fn restructure_portage_dir() {
    shrun(&ShellCommand::new("rm").args(["-rf", "/etc/portage/package.license"]));
    shrun(&ShellCommand::new("rm").args(["-rf", "/etc/portage/package.accept_keywords"]));
    shrun(&ShellCommand::new("rm").args(["-rf", "/etc/portage/package.use"]));

    shrun(&ShellCommand::new("touch").args(["/etc/portage/package.license"]));
    shrun(&ShellCommand::new("touch").args(["/etc/portage/package.accept_keywords"]));
    shrun(&ShellCommand::new("touch").args(["/etc/portage/package.use"]));
}
