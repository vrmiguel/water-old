# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Overview

`water` is a compiler designed to transform WebAssembly Text Format (WAT) into binary WebAssembly code. It focuses on providing fast compilation with a minimal footprint, making it suitable for embedded systems and performance-critical environments.

## Features

- **Lightweight**: Minimal dependencies and small binary size
- **Fast Compilation**: Optimized for quick compilation times
- **WAT Support**: Comprehensive support for WebAssembly Text Format
- **Cross-platform**: Runs on multiple operating systems

## Getting Started

### Installation

To get started with `water`, clone this repository and build it:

```bash
cargo build --release
```

### Usage

Compile a WAT file to WebAssembly binary:

```bash
water input.wat -o output.wasm
```

## Contributing

Contributions are welcome! Whether you find a bug, have a feature request, or want to improve the documentation, please feel free to open an issue or submit a pull request.

## License

See the LICENSE file for more information.

