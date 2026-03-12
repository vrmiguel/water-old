# water - WebAssembly Text Format Compiler

A lightweight and performant compiler for WebAssembly Text Format (WAT), written in Rust.

## Overview

`water` is a minimal yet efficient parser and compiler for WebAssembly's human-readable text format. It provides a robust foundation for parsing WAT modules, instructions, imports, functions, and other WebAssembly components with a focus on correctness and performance.

## Features

- **Fast parsing** - Efficient WAT instruction and module parsing using parser combinators
- **Minimal dependencies** - Lightweight codebase with only essential dependencies (`nom`)
- **Type-safe** - Leverages Rust's type system for safe AST representation
- **Extensible** - Modular architecture supporting custom emitters and transformations
- **LEB128 support** - Proper variable-length integer encoding for WebAssembly binaries

## Project Structure

The codebase is organized into several key modules:

### Core Components

- **`src/parser/`** - WAT parsing logic
  - `instruction.rs` - WebAssembly instruction parsing
  - `function.rs` - Function definition parsing
  - `import.rs` - Import statement parsing
  - `module.rs` - Module-level parsing
  - `utils.rs` - Parsing utility functions

- **`src/emitter/`** - Code emission and transformation
  - `emittable.rs` - Trait for emittable components
  - `arithmetic_operation.rs` - Arithmetic operation emission
  - `constant.rs` - Constant value emission
  - `numerical_value.rs` - Numerical value handling
  - `unreachable.rs` - Unreachable code handling

### Utilities

- **`src/ast.rs`** - Abstract Syntax Tree definitions
- **`src/leb128.rs`** - LEB128 variable-length integer encoding
- **`src/opcode.rs`** - WebAssembly opcode definitions
- **`src/small_string.rs`** - Optimized string handling
- **`src/lib.rs`** - Library exports
- **`src/main.rs`** - Entry point

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

### Development

Run tests with:
```bash
cargo test
```

Format code with:
```bash
cargo fmt
```

## Dependencies

- **nom** (7.1.1) - Parser combinators library for building efficient parsers

## Documentation

For detailed information about how the parser works and how to use the library, refer to the module documentation in the source code.

## License

Licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## Project Status

This is an experimental/educational project focused on understanding WebAssembly Text Format parsing and compilation. It serves as a reference implementation for WAT parsing in Rust.
