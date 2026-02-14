#!/bin/sh

# Cargo Zigbuild - Build for All Platforms
# POSIX-compatible version (works with /bin/sh)
# Builds your Rust binary for macOS, Linux, and Windows

cd "$(dirname "$0")" || exit
cd ".."

docker buildx build --platform linux/amd64,linux/arm64 -f Dockerfile.deb -o dist .