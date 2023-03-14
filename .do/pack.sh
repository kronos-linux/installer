#!/bin/sh -e

rm -rf .build/release && mkdir -p .build/release

cargo test --quiet 2>/dev/null
cargo build --release

cp target/release/installer .build/release
upx --best --lzma .build/release/installer
cp default/config.toml .build/release/config.toml
