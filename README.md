# water

`water` is a tiny Rust library for parsing pieces of the WebAssembly Text format (WAT).

The crate currently focuses on small, composable parser entry points rather than a full end-to-end compiler workflow. It is a good fit if you want to experiment with WAT syntax, build tooling around a lightweight AST, or reuse individual parsers in tests and prototypes.

## Current scope

Today, the crate exposes parsers for a few core WAT building blocks, including:

- modules
- functions, parameters, locals, and exports
- function imports
- instructions such as `call`, `unreachable`, `*.const`, `local.*`, and `global.*`

The public parser functions live under `water::parser`.

## Example

```rust
use water::parser::{parse_function, parse_instruction, parse_module};

assert!(parse_module("(module)").is_ok());
assert!(parse_function("(func $add (param i32) (local i32))").is_ok());
assert!(parse_instruction("(call $add (i32.const 5))").is_ok());
```

## Why this README exists

The repository is still small, so the code and doc-tests are the most complete source of truth. A useful next place to look is `src/parser.rs:1`, which re-exports the parser entry points used throughout the crate.
