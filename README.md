# water - WebAssembly Text Format Compiler

> **[Lire en français](README.fr.md)**

`water` is a small, performant compiler toolkit for the WebAssembly Text Format (WAT) written in Rust. It focuses on parsing, validating, and transforming WAT snippets with very few dependencies so the crate can be embedded in tooling chains, editors, or custom emitters.

## Highlights

- **Fast parsing** – `nom`-based parsers that parse instructions, functions, imports, and modules in a single pass.
- **Minimal dependencies** – The crate keeps its dependency graph tight (just `nom` at 7.1.1) so it is easy to audit and build.
- **Type-safe AST** – `water::ast` exposes strongly typed representations of WAT constructs, making downstream analysis straightforward.
- **Flexible emission** – Emitters in `src/emitter` let you walk the AST and generate custom output or bytecode formats.

## Quick start

### Prerequisites

- Rust toolchain 1.56 or later with `cargo` available on your `PATH`.

### Build

```bash
cargo build --release
```

### Run the demo

`water` ships with a simple `main.rs` that exercises the parser. Running the binary shows how the parser decodes instructions and imports:

```bash
cargo run
```

The output displays the parsed instruction ASTs and a formatted parser error (if any).

## Library usage example

Use `water` as a library from other Rust code:

```rust
use water::parser::{parse_function_import, parse_instruction};

fn main() {
    let instruction = parse_instruction("(i32.const 5)").unwrap();
    println!("Parsed: {instruction:#?}");

    let import = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
    parse_function_import(import).expect("valid import");
}
```

This mirrors the examples in `src/main.rs` and demonstrates how to parse both standalone instructions and imports with the provided helpers.

## Project structure

- `src/main.rs` – sample entry point that prints debug information from the parser.
- `src/lib.rs` – crate root exposing parser, AST, emitter, opcode, and helper modules.
- `src/parser/` – detailed parsers for WAT instructions, imports, and modules.
- `src/emitter/` – utilities that walk the AST and emit other representations or optimizations.
- `src/ast.rs`, `src/leb128.rs`, `src/opcode.rs`, `src/small_string.rs` – foundational helpers for the AST and binary encodings.

## Testing

```bash
cargo test
```

Runs the default test suite (currently empty, but ready for future parser and emitter tests).

## Contributing

Contributions, bug reports, and feature requests are welcome. Open an issue or submit a pull request with a clear description of the change.

## License

Licensed under the MIT License – see [LICENSE](LICENSE) for details.

## Project status

Experimental/educational: the crate is a learning exercise around WAT parsing and compiler tooling. Expect progress in small, incremental improvements.
