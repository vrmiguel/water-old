#!/bin/bash

# Set up the environment for Rust tools
export CARGO_HOME="/workspace/.cargo"
export RUSTUP_HOME="/workspace/.rustup"

# Use rustup to run commands
echo "Using rustup to run cargo commands"

echo "=== Checking cargo fmt ==="
rustup run nightly cargo fmt -- --check
FMT_EXIT=$?

echo "=== Running cargo clippy ==="
rustup run stable cargo clippy -- -D warnings
CLIPPY_EXIT=$?

echo "=== Running cargo build ==="
rustup run stable cargo build
BUILD_EXIT=$?

echo "=== Running cargo test ==="
rustup run stable cargo test
TEST_EXIT=$?
    
echo "Format check exit code: $FMT_EXIT"
echo "Clippy exit code: $CLIPPY_EXIT"
echo "Build exit code: $BUILD_EXIT"
echo "Test exit code: $TEST_EXIT"

if [ $FMT_EXIT -eq 0 ] && [ $CLIPPY_EXIT -eq 0 ] && [ $BUILD_EXIT -eq 0 ] && [ $TEST_EXIT -eq 0 ]; then
    echo "All checks passed!"
    exit 0
else
    echo "Some checks failed!"
    exit 1
fi