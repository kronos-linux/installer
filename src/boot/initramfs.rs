use crate::prelude::*;

pub fn configure() {
    info!("Emerging initramfs");
    shrun(&ShellCommand::new("tetrahedron").args(["-c"]));
    debug!("Finished initramfs creation");
}
