# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Current scope

The project is still small and currently exposes parser building blocks for a
subset of the WebAssembly text format. The crate already includes support for
parsing pieces such as:

- modules
- functions and function imports
- exports, params, and locals
- instructions like `call`, `unreachable`, `local.*`, `global.*`, and
  `*.const`

## Example

```rust
use water::parser::{parse_function_import, parse_instruction, parse_module};

assert!(parse_module("(module)").is_ok());
assert!(parse_instruction("(local.set $idx (i32.const 5))").is_ok());
assert!(parse_function_import(
    r#"(import "console" "log" (func $log (param i32) (param i32)))"#,
)
.is_ok());
```

## Status

`water` is an early-stage crate and the API is still evolving.
