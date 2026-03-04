# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Current scope

The project is early-stage and currently focuses on parsing pieces of WAT
syntax (for example instructions and function imports).

## Run locally

```bash
cargo run
```

The binary in `src/main.rs` demonstrates parser calls and prints parsed output.

## Library usage

Use parser entrypoints from `water::parser`, for example:

```rust
use water::parser::{parse_function_import, parse_instruction};
```
