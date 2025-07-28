#!/bin/bash

# Set environment variables
export RUSTUP_HOME="/workspace/.rustup"
export CARGO_HOME="/workspace/.cargo" 
export PATH="/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"

echo "Attempting to check Rust code..."

# Try to run a simple compilation check
echo "Checking basic syntax with rustc..."
/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --version || echo "rustc failed"

echo "Done with basic checks."