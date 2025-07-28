#!/bin/bash
export PATH="/workspace/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin:$PATH"
cd /workspace/project
cargo +nightly fmt -- --check