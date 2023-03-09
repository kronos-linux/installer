use crate::prelude::*;

mod error;
mod prelude;
mod utils;

pub fn exit_success() {
    let xstr1 = "Installer exited successfully.";
    let xstr2 = "Please reboot into KronOS";

    info!("\n{}\n{}", xstr1, xstr2);
}

fn main() -> Result<()> {
    utils::setup_logger();

    Ok(exit_success())
}
