#!/bin/bash
cd /workspace/project
export PATH="/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
export CARGO_HOME="/workspace/.cargo"
export RUSTUP_HOME="/workspace/.rustup"

# Run clippy through rustup to handle permissions
/nix/store/rnf8wb305f8508hnkjgi0i3vy2522b5w-rustup-1.28.2/bin/rustup run stable cargo clippy -- -D warnings