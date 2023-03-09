use crate::prelude::*;

mod error;
mod prelude;
mod utils;

mod boot;
mod chroot;
mod disks;
mod system;

pub fn exit_success() {
    let xstr1 = "Installer exited successfully.";
    let xstr2 = "Please reboot into KronOS";

    info!("\n{}\n{}", xstr1, xstr2);
}

pub fn run_as_root() {
    if std::env::var("USER").expect("Failed to read envvar USER") != "root" {
        let estr1 = "Installer must be run as root";
        Error::Usage(estr1.into()).handle()
    }
}

fn main() -> Result<()> {
    info!("KronOS Installer");

    // Logger
    utils::setup_logger();

    // Privilege
    run_as_root();

    // Config
    let t = utils::config_target();
    let c = utils::generate_config(&t);
    report_config(&c);

    // Given a set of targets and configuration options, the disk configuration
    // is going to partition, format and mount disks such that the space onto
    // which kronos will be installed will be mounted to /mnt/gentoo, the ESP
    // will be mounted on /mnt/gentoo/boot and the configuration will have the
    // information in it to handle encryption, btrfs and lvm if they are
    // necessary.
    let c = disks::configure(c);

    // Given the architechture and what gui will be installed, locate, verify
    // and extract the stage3 archive into the root volume.
    let c = chroot::configure(c);

    // Configure the system. Including gui installation
    let c = system::configure(c);

    // Install the kernel, initramfs and bootloader. Configure them accordingly
    let c = boot::configure(c);

    // Exit
    report_config(&c);
    Ok(exit_success())
}

fn report_config(c: &Config) {
    debug!("{:?}", c);
}

mod tests;
