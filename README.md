# water - WebAssembly Text Format Compiler

> **[Lire en français](README.fr.md)**

A lightweight and performant compiler for WebAssembly Text Format (WAT), written in Rust.

## Overview

`water` is a minimal yet efficient parser and compiler for WebAssembly's human-readable text format. It provides a foundation for parsing WAT modules, instructions, imports, functions, and other WebAssembly components.

## Features

- **Fast parsing** - Efficient WAT instruction and module parsing using `nom`
- **Minimal dependencies** - Lightweight codebase with only essential dependencies
- **Type-safe** - Leverages Rust's type system for safe AST representation
- **Extensible** - Modular architecture supporting custom emitters and transformations

## Project Structure

- `src/parser/` - WAT parsing logic for instructions, functions, imports, and modules
- `src/emitter/` - Code emission and transformation utilities
- `src/ast.rs` - Abstract Syntax Tree definitions
- `src/leb128.rs` - LEB128 variable-length integer encoding
- `src/opcode.rs` - WebAssembly opcode definitions

## Getting Started

### Prerequisites

- Rust 1.56 or later

### Building

```bash
cargo build --release
```

### Running the example harness

`cargo run` prints parsing results for a few hard-coded strings from `src/main.rs`. It demonstrates the low-level parser helpers like `parse_instruction` and `parse_function_import`, so you can see how the library navigates atoms such as `i32.const`, `local.set`, and imports.

### Library Usage

```rust
use water::parser::{parse_instruction, parse_function_import};

let ok = parse_instruction("(i32.const 42)").unwrap();
let err = parse_function_import(r#"(import "env" "log" (func))"#);

match err {
    Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
        println!("{}", nom::error::convert_error(r#"(import ...)"#, e));
    }
    _ => {}
}
```

This highlights how you can call the public parser helpers directly and convert `nom` errors into readable diagnostics: the example in `main.rs` wraps this in `stringify_error`.

## Dependencies

- **nom** (7.1.1) - Parser combinators library

## Testing

```bash
cargo test
```

Currently the test suite exercises the parser combinators indirectly through the examples in `main.rs`.

## License

Licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## Project Status

This is an experimental/educational project focused on understanding WebAssembly Text Format parsing and compilation.

## Contributing

Feedback, issues, and pull requests are welcome. Feel free to open an issue if you hit a parsing edge case or want help building atop `water`.

## Need More?

If you want to explore the parser in another language, try translating `water::parser` into your language of choice and compare results with `wabt` or other tooling to learn more about WAT formats.
