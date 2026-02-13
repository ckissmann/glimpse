#!/bin/sh

# Cargo Zigbuild - Build for All Platforms
# POSIX-compatible version (works with /bin/sh)
# Builds your Rust binary for macOS, Linux, and Windows

cd "$(dirname "$0")" || exit
cd ".." || exit

cargo install cargo-zigbuild

cd scripts || exit

sh ./compile-all.sh