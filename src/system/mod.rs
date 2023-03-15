use crate::prelude::*;

mod fstab;
mod locale;
mod networking;
mod portage;
mod sysutils;
mod update;

pub fn configure(c: Config) -> (Config, Vec<std::thread::JoinHandle<()>>) {
    fstab::configure(&c);

    let msj = portage::configure();

    locale::configure(&c);

    let upj = update::configure();

    sysutils::install(&c);

    sysutils::configure(&c);

    networking::configure(&c);

    (c, vec![upj, msj])
}
