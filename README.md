# water - WebAssembly Text Format Compiler

> **[Lire en français](README.fr.md)**

A lightweight, experimental compiler for WebAssembly Text Format (WAT), written in Rust.

## Overview

`water` is a minimal parser and compiler for WebAssembly's human-readable text format. It provides a foundation for parsing WAT modules, instructions, imports, functions, and other WebAssembly components while keeping the codebase approachable.

## Features

- **Efficient parsing** - WAT instruction and module parsing powered by `nom`
- **Minimal dependencies** - Lightweight codebase with only essential dependencies
- **Type-safe** - Leverages Rust's type system for safe AST representation
- **Extensible** - Modular architecture supporting custom emitters and transformations

## Project Structure

- `src/parser/` - WAT parsing logic for instructions, functions, imports, and modules
- `src/emitter/` - Code emission and transformation utilities
- `src/ast.rs` - Abstract Syntax Tree definitions
- `src/leb128.rs` - LEB128 variable-length integer encoding
- `src/opcode.rs` - WebAssembly opcode definitions
- `src/small_string.rs` - Compact string helper used by the AST

## Getting Started

### Prerequisites

- Rust 1.56 or later

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

## Dependencies

- **nom** (7.1.1) - Parser combinators library

## License

Licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## Project Status

This is an experimental and educational project focused on understanding WebAssembly Text Format parsing and compilation.
