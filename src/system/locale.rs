use crate::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub fn configure(c: &Config) {
    info!("Updating locales and timezone");
    let tz: String = get_value(c, "localisation.timezone");
    let lc: Vec<String> = get_value(c, "localisation.locales");

    timezone(&tz);
    locales(&lc);

    shrun(&ShellCommand::new("eselect").args(["locale", "set", "C.utf8"]));
}

fn timezone(zone: &str) {
    let mut file = File::create("/etc/timezone").expect("Could not open /etc/timezone");
    write!(file, "{}\n", zone).expect("Could not write timezone");

    shrun(&ShellCommand::new("emerge").args(["--config", "sys-libs/timezone-data"]));
    debug!("Timezone updated")
}

fn locales(l: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/locale.gen")
        .expect("Could not open locale.gen");
    for loc in l {
        write!(file, "{}\n", loc).expect("Failed to update locale.gen");
    }
    shrun(&ShellCommand::new("locale-gen"));
    debug!("Locales generated");
}
