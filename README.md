# water - WebAssembly Text Format Compiler

> **[Lire en français](README.fr.md)**

A lightweight and performant compiler toolkit for WebAssembly Text Format (WAT), written in Rust.

## Overview

`water` provides parser and emitter building blocks for WebAssembly's human-readable text format. It is focused on small, understandable modules for parsing instructions, functions, imports, and modules.

## Features

- Fast WAT parsing powered by `nom`
- Minimal dependency footprint
- Type-safe AST representation in Rust
- Modular design for parser and emitter experimentation

## Project Structure

- `src/parser/` - WAT parsing logic for instructions, functions, imports, and modules
- `src/emitter/` - Code emission utilities for WebAssembly elements
- `src/ast.rs` - Abstract Syntax Tree (AST) definitions
- `src/leb128.rs` - LEB128 variable-length integer encoding helpers
- `src/opcode.rs` - WebAssembly opcode definitions
- `src/main.rs` - Small executable showcasing parser usage

## Getting Started

### Prerequisites

- Rust 1.56+

### Build

```bash
cargo build --release
```

### Run the example binary

```bash
cargo run
```

### Run tests

```bash
cargo test
```

## Library Usage

```rust
use water::parser::parse_instruction;

fn main() {
    let instruction = parse_instruction("i32.const 5").unwrap();
    println!("{instruction:?}");
}
```

## Dependencies

- [`nom`](https://crates.io/crates/nom) - Parser combinators library

## Project Status

This project is experimental and primarily aimed at learning and exploring WAT parsing and compilation internals.

## License

Licensed under the MIT License. See [LICENSE](LICENSE).
