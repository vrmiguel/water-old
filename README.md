# water - WebAssembly Text Format Compiler

> **[Lire en français](README.fr.md)**

A lightweight, Rust-native compiler for WebAssembly's textual format (WAT) that focuses on minimal dependencies, safe AST structures, and a modular architecture.

## Table of Contents
- Overview
- Features
- Example library usage
- Quick CLI sample
- Project structure
- Development
- Testing
- Contributing
- License

## Overview

`water` provides a fast, focused parser for WAT files, along with AST definitions and emission helpers that make it easy to build higher-level tooling (e.g., custom emitters, analyzers, or transformation passes).

## Features

- **Nom-powered parsing** – All parsing is built on `nom`, giving a predictable, highly composable parser that can describe instructions, imports, functions, and whole modules.
- **Rich AST** – `water::ast` exposes instruction, function, import, and value types so you can inspect or transform parsed modules while keeping Rust's safety guarantees.
- **Emitter helpers** – The emitter module contains reusable primitives for encoding arithmetic, constants, and unreachable markers, which can be extended when targeting bytecode formats.
- **Minimal dependencies** – Only the essentials (primarily `nom`) are pulled in so this crate can be embedded in binaries or used as a building block for other toolchains.

## Example library usage

```rust
use nom::error::VerboseError;
use water::parser::{parse_function_import, parse_instruction};

fn stringify_error(
    input: &str,
    error: nom::Err<VerboseError<&str>>,
) -> String {
    match error {
        nom::Err::Incomplete(_) => unreachable!(),
        nom::Err::Error(err) | nom::Err::Failure(err) => {
            nom::error::convert_error(input, err)
        }
    }
}

fn main() -> Result<(), nom::Err<VerboseError<&'static str>>> {
    let i = parse_instruction("i32.const 5")?;
    println!("parsed instruction: {:?}", i);

    let import = r#"(import \"console\" \"log\" (func $log (param i32) (param i32)))"#;
    if let Err(err) = parse_function_import(import) {
        println!("error: {}", stringify_error(import, err));
    }

    Ok(())
}
```

This mirrors the `main` binary and shows how to inspect instructions or surfacing parsing errors with `nom::error::convert_error`.

## Quick CLI sample

The crate ships with a tiny binary (`cargo run`) that exercises the parser and prints the resulting AST nodes via `dbg!`. Running the binary demonstrates parsing plain instructions, parenthesized instructions, and an import definition, including error formatting for malformed input.

## Project structure

- `src/parser/` – All the nom-based parsers for instructions, functions, modules, and imports.
- `src/ast.rs` – AST nodes such as `Instruction`, `Opcode`, `VariableOperation`, and import signatures.
- `src/emitter/` – Reusable emission logic for constants, arithmetic operations, and unreachable traps.
- `src/leb128.rs` – Helper utilities for LEB128 integer encoding used when emitting binary representations.
- `src/opcode.rs` – Opcode-level helpers and enumerations.
- `src/main.rs` – Example binary showing how to parse and pretty-print instructions.

## Development

### Prerequisites
- [Rust](https://www.rust-lang.org/) 1.56 or later

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Testing

No unit tests exist yet, but you can run the standard Rust test harness in case additional coverage is added later.

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or PR if you want to add new parsing rules, AST helpers, emitters, or tooling around WAT compilation. Keep changes small—focus on either parser ergonomics, AST expressiveness, or emitter utilities.

## License

Licensed under the MIT License – see the [LICENSE](LICENSE) file for details.
