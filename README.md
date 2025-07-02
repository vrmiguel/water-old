# water (WebAssembly TExt foRmat compiler)

`water` is a tiny and performant WebAssembly Text Format compiler designed to translate WebAssembly text format (WAT) to WebAssembly binary format (WASM).

## Project Overview

Water is built to be:

- **Fast**: Optimized for performance with minimal overhead
- **Lightweight**: Small footprint with minimal dependencies
- **Accurate**: Aims for full compliance with the WebAssembly specification
- **Developer-friendly**: Clear error messages and intuitive API

## Features

- WAT to WASM compilation
- Validation of WebAssembly modules
- Support for WebAssembly 1.0 specification
- Efficient binary encoding

## Getting Started

### Requirements

- Rust toolchain (cargo, rustc)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/vrmiguel/water.git
cd water

# Build in release mode
cargo build --release
```

### Usage

```bash
# Compile a WAT file to WASM
water input.wat -o output.wasm

# Or pipe WAT content directly
cat input.wat | water > output.wasm
```

## Documentation

For more information on the WebAssembly Text Format:
- [WebAssembly Specification](https://webassembly.github.io/spec/core/)
- [Understanding the WAT format](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.