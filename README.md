# water (WebAssembly TExt foRmat compiler)

`water` is a tiny and performant WebAssembly Text Format (WAT) compiler written in Rust. It parses WebAssembly Text Format code and compiles it to WebAssembly binary format (WASM).

## Features

- Parsing of WebAssembly Text Format (WAT) code
- Support for WASM numeric types: i32, i64, f32, f64
- Support for local and global variable operations (get, set, tee)
- Support for constants, function calls, and arithmetic operations
- Parsing of function imports with namespaces
- Efficient binary emission using LEB128 encoding

## Supported Instructions

- Constants (i32.const, i64.const, f32.const, f64.const)
- Variable operations (local.get, local.set, local.tee, global.get, global.set)
- Function calls (call)
- Arithmetic operations (add, sub, mul, div, etc.)
- Comparison operations
- Unreachable instruction

## Parsing Examples

```wat
;; Constants
i32.const 5
f64.const 2.5

;; Variable operations
local.set $idx
(local.set $idx (i32.const 5))

;; Function imports
(import "console" "log" (func $log (param i32) (param i32)))
```

## Status

This project is under active development. Not all WebAssembly features are currently implemented.

## License

TBD
