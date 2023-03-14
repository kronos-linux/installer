use crate::prelude::*;

mod fstab;
mod locale;
mod networking;
mod portage;
mod sysutils;
mod update;

pub fn configure(c: Config) -> (Config, std::thread::JoinHandle<()>) {
    fstab::configure(&c);

    let msj = portage::configure();

    locale::configure(&c);

    sysutils::install(&c);

    sysutils::configure();

    networking::configure();

    update::configure();

    (c, msj)
}
