#!/bin/bash

# Set up environment
export PATH="/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
export PATH="/workspace/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin:$PATH"

echo "Checking Rust compilation..."

# Try basic compilation
echo "Running rustc --version..."
rustc --version

echo "Running cargo check..."
cargo check

echo "Running cargo clippy with strict warnings..."
cargo clippy -- -D warnings

echo "Running cargo fmt check..."
cargo +nightly fmt -- --check

echo "Running cargo build..."
cargo build

echo "Running cargo test..."
cargo test

echo "Build check complete!"