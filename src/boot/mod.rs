use crate::prelude::*;

mod bootloader;
mod initramfs;
mod kernel;

pub fn configure(c: Config) -> Config {
    initramfs::configure(&c);
    kernel::configure(&c);
    bootloader::configure(&c);

    c
}
