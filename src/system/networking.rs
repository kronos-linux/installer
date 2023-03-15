use crate::prelude::*;

use std::{fs::OpenOptions, io::Write};

pub fn configure(c: &Config) {
    let mut hosts = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/hosts")
        .expect("Failed to open hosts file");

    let hostname: String = get_value(c, "hostname");

    let ipv4 = format!("127.0.0.1       {}            localhost\n", hostname);
    let ipv6 = format!("::1             {}            localhost\n", hostname);
    write!(hosts, "{}", ipv4).expect("Failed to write to hosts file");
    write!(hosts, "{}", ipv6).expect("Failed to write to hosts file");
}
