# water (WebAssembly TExt foRmat compiler)

`water` is a small Rust project for parsing pieces of the WebAssembly Text
Format (WAT). It currently exposes parser building blocks that can be used to
recognize modules, imports, instructions, functions, and related syntax.

## Why this project exists

The goal of `water` is to stay lightweight and fast while keeping the parser API
easy to compose. The crate uses [`nom`](https://crates.io/crates/nom) internally
and exposes public parsing helpers from `water::parser`.

## Current capabilities

Today the crate includes parsers for:

- modules
- function imports
- functions, params, exports, and locals
- instructions such as constants, calls, and variable operations
- identifiers, indices, strings, and value types

## Quick start

Build the project:

```bash
cargo build
```

Run the current demo binary:

```bash
cargo run
```

The binary in `src/main.rs` currently exercises a few parser entry points and
prints the parsed structures for quick manual inspection.

## Library usage

Parse a single instruction:

```rust
use water::parser::parse_instruction;

let instruction = parse_instruction("(i32.const 5)").unwrap().1;
println!("{instruction:?}");
```

Parse a function import:

```rust
use water::parser::parse_function_import;

let wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
let import = parse_function_import(wat).unwrap().1;
println!("{import:?}");
```

Parse a minimal module:

```rust
use water::parser::parse_module;

assert!(parse_module("(module)").is_ok());
```

## Project layout

- `src/parser.rs` re-exports the public parser surface
- `src/ast.rs` defines the syntax tree types
- `src/emitter.rs` contains emission-related code
- `src/main.rs` is a small parser demo

## Status

This repository is still early-stage. If you are exploring or extending it, the
best place to start is the parser module and the doctest examples alongside each
parsing function.
