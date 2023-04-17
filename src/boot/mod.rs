use crate::prelude::*;

mod bootloader;
mod initramfs;
mod kernel;

pub fn configure(c: Config) -> Config {
    initramfs::configure();
    kernel::configure();
    bootloader::configure(&c);

    c
}
