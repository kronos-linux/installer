use crate::prelude::*;

pub fn configure() {
    info!("Configuring portage");
    sync_repo();
}

fn sync_repo() {
    shrun(&ShellCommand::new("emerge-webrsync"));

    match ShellCommand::new("emerge").args(["--sync", "-q"]).run() {
        Ok(_) => debug!("Synced repository"),
        Err(_) => warn!("Failed to sync repository; Ignoring..."),
    }
}
