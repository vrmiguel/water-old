# water (WebAssembly TExt foRmat compiler)

`water` is an experimental Rust crate for parsing pieces of the
WebAssembly Text Format (WAT) and emitting WebAssembly bytes.

The project currently has stronger parser support than compiler support, so it
is best understood as an early-stage WAT toolkit rather than a complete
`wat`-to-`wasm` compiler.

## What it currently does

- Parses WAT modules such as `(module)`
- Parses function signatures, parameters, locals, and exports
- Parses function imports
- Parses a subset of instructions, including:
  - numeric constants like `i32.const 5`
  - variable operations like `local.set $x`
  - `call`
  - `unreachable`
- Emits the WebAssembly magic header and version bytes through the `Emitter`

## Example

```rust
use water::parser::{parse_function_import, parse_instruction, parse_module};

fn main() {
    assert!(parse_module("(module)").is_ok());
    assert!(parse_instruction("i32.const 5").is_ok());

    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
    assert!(parse_function_import(import_wat).is_ok());
}
```

## Crate layout

- `src/parser.rs`: public parser entry points and parser result type
- `src/ast.rs`: AST nodes used by the parser and emitter
- `src/emitter.rs`: binary emitter scaffolding
- `src/main.rs`: small local parsing demo

## Status

This repository is still incomplete:

- instruction coverage is partial
- full module compilation is not implemented yet
- the emitter currently only writes the WASM preamble

If you want to explore the supported syntax in more detail, the parser modules
contain concise doc tests with representative examples.
