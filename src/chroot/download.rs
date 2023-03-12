use crate::prelude::*;

use std::{
    env::{current_dir, set_current_dir},
    path::Path,
};

pub fn get_s3_string(gui: bool) -> String {
    info!("Locating latest stage3 archive");
    let rex = if !gui {
        "stage3-amd64-openrc-[0-9]*T[0-9]*Z.tar.xz"
    } else {
        "stage3-amd64-desktop-openrc-[0-9]*T[0-9]*Z.tar.xz"
    };

    let a = ["https://distfiles.gentoo.org/releases/amd64/autobuilds/current-stage3-amd64-desktop-openrc", "-O", "-"];

    let wget = shrun(&ShellCommand::new("wget").args(a));
    let grep = shrun(
        &ShellCommand::new("grep")
            .args(["-om1", rex])
            .pipe_string(wget),
    );
    debug!("Latest stage3 archive located");
    Vec::from_iter(grep.lines())[0].replace("\n", "")
}

pub fn stage3(t: &str, tasc: &str) {
    info!("Downloading latest stage3 archive");
    let cd = current_dir().expect("Failed to get running directory");
    set_current_dir(Path::new("/mnt/gentoo")).expect("Failed directory change /mnt/gentoo");

    shrun(&ShellCommand::new("wget").args(["https://distfiles.gentoo.org/releases/amd64/autobuilds/current-stage3-amd64-desktop-openrc/".to_owned()+&t]));
    shrun(&ShellCommand::new("wget").args(["https://distfiles.gentoo.org/releases/amd64/autobuilds/current-stage3-amd64-desktop-openrc/".to_owned()+&tasc]));
    debug!("Latest stage3 archive downloaded");

    let ls = shrun(&ShellCommand::new("ls").args(["-lah"]));
    debug!("/mnt/gentoo contents:\n{}", ls);

    // Corrupt the stage3 archive to see how the installer behaves
    // shrun(&ShellCommand::new("truncate").args(["-s", "5M", t]));

    s3_check(t, tasc);
    shrun(&ShellCommand::new("rm").args([tasc]));

    s3_extract(t);
    shrun(&ShellCommand::new("rm").args([t]));

    let ls = shrun(&ShellCommand::new("ls").args(["-lah"]));
    debug!("/mnt/gentoo contents:\n{}", ls);

    set_current_dir(cd).expect("Failed directory change to running directory");
}

fn s3_extract(t: &str) {
    info!("Extracting stage3 archive");
    shrun(&ShellCommand::new("tar").args(["xpf", t, "--xattrs-include='*.*'", "--numeric-owner"]));
}

fn s3_check(t: &str, tasc: &str) {
    info!("Checking stage3 archive integrity and authenticity");

    debug!("Importing GPG keys");
    shrun(&ShellCommand::new("wget").args([
        "-O",
        "keys.gpg",
        "https://qa-reports.gentoo.org/output/service-keys.gpg",
    ]));

    shrun(&ShellCommand::new("gpg").args(["--import", "keys.gpg"]));
    shrun(&ShellCommand::new("rm").args(["keys.gpg"]));
    debug!("GPG keys imported");

    shrun(&ShellCommand::new("gpg").args(["--verify", tasc, t]));

    debug!("Stage3 archive check complete")
}
