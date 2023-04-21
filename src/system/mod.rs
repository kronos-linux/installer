use crate::prelude::*;

mod fstab;
mod locale;
mod networking;
mod portage;
mod sysutils;
mod update;
mod users;

pub fn configure(c: Config) -> Config {
    fstab::configure(&c);

    let msj = portage::configure();

    locale::configure(&c);

    let upj = update::configure();

    sysutils::install(&c);

    sysutils::configure(&c);

    networking::configure(&c);

    root_passwd();

    wait(vec![upj, msj]);

    users::configure(&c);

    c
}

fn wait(joins: Vec<std::thread::JoinHandle<()>>) {
    for j in joins {
        match j.join() {
            Ok(_) => (),
            Err(_) => warn!("Failed to join mirrorselect thread"),
        };
    }
}

fn root_passwd() {
    shrun(&ShellCommand::new("passwd").args(["-de", "root"]));
}
