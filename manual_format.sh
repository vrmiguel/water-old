#!/bin/bash
cd /workspace/project

# Try to run rustfmt on individual files
for file in $(find src -name "*.rs"); do
    echo "Formatting $file"
    /workspace/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustfmt --check "$file" 2>&1 || {
        echo "Format issues in $file, applying fixes..."
        /workspace/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustfmt "$file"
    }
done