# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Features

- Fast parsing and compilation of WebAssembly Text Format (.wat) files
- Lightweight implementation with minimal dependencies
- Built with Rust for memory safety and performance

## Installation

```bash
cargo install --git https://github.com/vrmiguel/water-old
```

## Usage

```bash
water input.wat -o output.wasm
```

## Development Status

⚠️ **Early Work in Progress** - This project is in active development and not ready for production use.

## Building from Source

```bash
git clone https://github.com/vrmiguel/water-old
cd water-old
cargo build --release
```

## License

This project is licensed under the terms specified in the LICENSE file.

