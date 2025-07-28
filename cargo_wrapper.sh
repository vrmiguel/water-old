#!/bin/bash
export RUSTUP_HOME="/workspace/.rustup"
export CARGO_HOME="/workspace/.cargo"
exec rustup run stable cargo "$@"