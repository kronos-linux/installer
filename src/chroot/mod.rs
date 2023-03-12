use crate::prelude::*;

mod download;

pub fn configure(c: Config) -> Config {
    prepare(&c);

    c
}

fn prepare(c: &Config) {
    match std::env::set_current_dir("/mnt/gentoo") {
        Err(_) => Error::Generic("Failed to change directory".into()).handle(),
        _ => (),
    }
    let gui_enable = get_value(c, "gui.enabled");

    let target = download::get_s3_string(gui_enable);
    let target_asc = target.clone() + ".asc";
    debug!("Stage3 target: {}", target);
    debug!("Target asc: {}", target_asc);

    download::stage3(&target, &target_asc);
    // make_conf();
    // ebuild_repo();
    // dns_info();
    //
    // relevant_mounts();
}
