# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Features

- Fast compilation of WebAssembly Text Format (WAT) files
- Minimal memory footprint
- Built with Rust for safety and performance
- Uses the `nom` parser combinator library for efficient parsing

## Building

To build the project, you need Rust installed. Then run:

```bash
cargo build --release
```

## Usage

```bash
cargo run -- input.wat
```

## Project Structure

- `src/ast.rs` - Abstract Syntax Tree definitions
- `src/parser.rs` - WAT format parser implementation
- `src/emitter.rs` - WebAssembly bytecode emitter
- `src/opcode.rs` - WebAssembly opcode definitions
- `src/leb128.rs` - LEB128 encoding utilities

## License

This project is licensed under the terms specified in the LICENSE file.

