use crate::prelude::*;

mod fstab;
mod locale;
mod mirrors;
mod networking;
mod portage;
mod sysutils;
mod update;

pub fn configure(c: Config) -> Config {
    fstab::configure(&c);

    portage::configure();

    locale::configure();

    mirrors::configure();

    sysutils::install();

    sysutils::configure();

    networking::configure();

    update::configure();

    c
}
