#!/bin/sh -e

rm -rf .build/debug && mkdir -p .build/debug

cargo test
cargo build

cp target/debug/installer .build/debug/installer
cp default/debug.toml .build/debug/config.toml

printf "#!/bin/sh\nLOG_LEVEL=\"debug\" ./kronos_installer config.toml\n" \
    >> .build/debug/test_install.sh
chmod +x .build/debug/test_install.sh
