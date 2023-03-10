use crate::prelude::*;

mod encrypt;
mod partition;

pub fn configure(c: Config) -> Config {
    info!("Configuring the disks");

    let bootd: String = get_value(&c, "disk.boot");
    let rootd: String = get_value(&c, "disk.root");
    let (bb, esp, rp) = partition::base(&bootd, &rootd);
    let c = add_value(c, "disk.bb", bb);
    let c = add_value(c, "disk.esp", esp);
    let c = add_value(c, "disk.root_partition", rp);

    let c = encrypt::root_partition(c);

    c
}
