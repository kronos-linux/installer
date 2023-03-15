use crate::prelude::*;
use std::thread;

pub fn configure() -> thread::JoinHandle<()> {
    thread::spawn(|| update_world())
}

fn update_world() {
    info!("Updating @world");
    shrun(&ShellCommand::new("emerge").args(["-vnquDN", "--with-bdeps=y", "@world"]));
    debug!("@world updated")
}
