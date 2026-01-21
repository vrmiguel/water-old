# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler, designed to bring efficient WAT-to-WASM compilation to developers who prioritize performance and minimal resource usage.

## Overview

`water` is a compiler designed to transform WebAssembly Text Format (WAT) into binary WebAssembly code. It focuses on providing fast compilation with a minimal footprint, making it suitable for embedded systems and performance-critical environments. The project combines Rust's safety guarantees with optimized compilation strategies to deliver a reliable and efficient toolchain.

## Motivation

WebAssembly represents a transformative technology for portable, high-performance code execution across platforms. However, many WAT compilers come with significant overhead or dependencies. `water` addresses this gap by providing a lean, fast alternative that maintains full compatibility with the WebAssembly specification while minimizing resource consumption.

## Features

- **Lightweight**: Minimal dependencies and small binary size for easy distribution and embedding
- **Fast Compilation**: Optimized for quick compilation times without sacrificing correctness
- **WAT Support**: Comprehensive support for WebAssembly Text Format specifications
- **Cross-platform**: Runs on multiple operating systems and architectures
- **Reliable**: Built in Rust with strong type safety and memory guarantees
- **Embeddable**: Can be integrated into larger projects as a library

## Project Structure

The project is organized to maintain clarity and modularity:
- **src/**: Core compiler implementation
- **tests/**: Comprehensive test suite for validation
- **examples/**: Sample WAT files demonstrating compiler usage

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Installation

To get started with `water`, clone this repository and build it:

```bash
git clone <repository-url>
cd water
cargo build --release
```

The compiled binary will be available at `target/release/water`.

### Usage

#### Command Line

Compile a WAT file to WebAssembly binary:

```bash
water input.wat -o output.wasm
```

#### As a Library

You can also use `water` as a dependency in your Rust projects:

```toml
[dependencies]
water = { path = "./water" }
```

## Examples

### Basic WAT Module

```wat
(module
  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)
  (export "add" (func $add)))
```

Compile this with:
```bash
water example.wat -o example.wasm
```

## Testing

Run the test suite to ensure everything is working correctly:

```bash
cargo test
```

For linting and formatting checks:
```bash
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Performance

`water` is optimized for both compilation speed and binary size:
- Minimal memory footprint during compilation
- Direct WAT-to-WASM translation without unnecessary intermediate representations
- Suitable for resource-constrained environments

## Contributing

Contributions are welcome! Whether you find a bug, have a feature request, or want to improve the documentation, please feel free to open an issue or submit a pull request. We appreciate all forms of contributions, from code improvements to documentation enhancements.

### Development Guidelines

1. Fork the repository
2. Create a feature branch for your changes
3. Ensure all tests pass and code is properly formatted
4. Submit a pull request with a clear description of your changes

## License

See the LICENSE file for more information.

