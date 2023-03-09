use crate::prelude::*;

mod error;
mod prelude;
mod utils;

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
    // Logger
    utils::setup_logger();

    // Privilege
    run_as_root();

    // Config
    let t = utils::config_target();
    let installation_config = utils::generate_config(&t);
    report_config(&installation_config);

    // Exit
    Ok(exit_success())
}

fn report_config(c: &Config) {
    debug!("{:?}", c);
}

mod tests;
