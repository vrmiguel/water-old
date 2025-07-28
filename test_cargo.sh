#!/bin/bash

# Set up the environment for Rust tools
export CARGO_HOME="/workspace/.cargo"
export RUSTUP_HOME="/workspace/.rustup"

# Try to locate and use cargo from the stable toolchain
CARGO_BIN="/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo"

if [ -x "$CARGO_BIN" ]; then
    echo "Using cargo from: $CARGO_BIN"
    
    echo "=== Checking cargo fmt ==="
    "$CARGO_BIN" fmt -- --check
    FMT_EXIT=$?
    
    echo "=== Running cargo clippy ==="
    "$CARGO_BIN" clippy -- -D warnings
    CLIPPY_EXIT=$?
    
    echo "=== Running cargo build ==="
    "$CARGO_BIN" build
    BUILD_EXIT=$?
    
    echo "=== Running cargo test ==="
    "$CARGO_BIN" test
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
else
    echo "Cargo binary not found or not executable at: $CARGO_BIN"
    exit 1
fi