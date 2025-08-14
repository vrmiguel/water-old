# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Features

- Fast compilation of WebAssembly Text Format (.wat) files
- Minimal resource usage
- Built with Rust for performance and safety
- Uses nom parser combinator library for efficient parsing

## Building

To build the project:

```bash
cargo build --release
```

## Usage

Run the compiler:

```bash
cargo run -- input.wat
```

Or with the built binary:

```bash
./target/release/water input.wat
```

## Project Structure

- `src/main.rs` - Entry point of the application
- `src/lib.rs` - Main library interface
- `src/parser/` - WebAssembly text format parsing logic
- `src/emitter/` - Code emission and generation
- `src/ast.rs` - Abstract syntax tree definitions
- `src/opcode.rs` - WebAssembly opcode definitions

## Dependencies

- `nom` - Parser combinator library for efficient parsing

## License

This project is licensed under the terms found in the LICENSE file.
