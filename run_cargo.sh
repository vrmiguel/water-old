#!/bin/bash
export PATH="/workspace/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
cd /workspace/project
exec "$@"