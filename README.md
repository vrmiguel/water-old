# water

> **[Lire en français](README.fr.md)**

`water` is a compiler for the [WebAssembly Text Format (WAT)](https://webassembly.github.io/spec/core/text/index.html), written in Rust. It parses human-readable `.wat` source files and emits WebAssembly binary (`.wasm`) output.

[![build-and-test](https://github.com/vrmiguel/water/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/vrmiguel/water/actions/workflows/build-and-test.yml)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Layout](#project-layout)
- [Supported WAT Constructs](#supported-wat-constructs)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Architecture](#architecture)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

WebAssembly Text Format (WAT) is the human-readable equivalent of WebAssembly binary. `water` takes WAT source code, constructs a typed Abstract Syntax Tree (AST), and emits the corresponding binary. It is designed to be minimal, correct, and easy to extend.

The project is currently in an experimental stage and serves as an educational reference for understanding WebAssembly parsing and compilation.

---

## Features

- **WAT parser** — Recursive, combinator-based parser (powered by [`nom`](https://github.com/rust-bakery/nom)) for WAT instructions, functions, imports, and more.
- **Typed AST** — A fully typed, Rust-native abstract syntax tree covering WebAssembly's four numeric types (`i32`, `i64`, `f32`, `f64`), instructions, and module structure.
- **Binary emitter** — Writes the WebAssembly binary magic header, version tag, and encoded instructions to any `std::io::Write` sink.
- **LEB128 encoding** — Correct signed and unsigned [Little Endian Base 128](https://en.wikipedia.org/wiki/LEB128) encoding, which WebAssembly uses to represent all integer literals in binary.
- **Opcode mapping** — Every arithmetic, comparison, variable, constant, and control instruction maps to its official WebAssembly binary opcode.
- **SmallString** — A heap-avoiding string type that stores identifiers ≤ 22 bytes inline, reducing allocations for the common case.
- **Cross-platform CI** — Tested on Linux, macOS, and Windows via GitHub Actions.

---

## Project Layout

```
water/
├── src/
│   ├── main.rs          # Entry point / demo
│   ├── lib.rs           # Crate root; re-exports all public modules
│   ├── ast.rs           # Abstract Syntax Tree types
│   ├── opcode.rs        # ToOpcode trait + opcode byte mappings
│   ├── leb128.rs        # LEB128 signed & unsigned encoders
│   ├── small_string.rs  # Inline-optimised, cheaply-clonable string
│   ├── parser/
│   │   ├── mod.rs       # Module entry; re-exports public parsing fns
│   │   ├── instruction.rs  # Instruction & opcode parsers
│   │   ├── function.rs     # Function, parameter, local, export parsers
│   │   ├── import.rs       # Function-import parser
│   │   ├── module.rs       # Module-level parser (WIP)
│   │   └── utils.rs        # Shared combinators (types, identifiers, …)
│   └── emitter/
│       ├── mod.rs           # Emitter<W> struct; emits magic, version, program
│       ├── emittable.rs     # Emittable<T> trait
│       ├── arithmetic_operation.rs
│       ├── constant.rs
│       ├── numerical_value.rs
│       └── unreachable.rs
├── Cargo.toml
└── LICENSE
```

---

## Supported WAT Constructs

### Types

| WAT keyword | Rust variant            |
|-------------|-------------------------|
| `i32`       | `NumericalType::Int32`  |
| `i64`       | `NumericalType::Int64`  |
| `f32`       | `NumericalType::Float32`|
| `f64`       | `NumericalType::Float64`|

### Instructions

| WAT instruction            | Description                              |
|----------------------------|------------------------------------------|
| `i32.const`, `f64.const` … | Push a numeric constant onto the stack   |
| `local.get`, `local.set`, `local.tee` | Read / write a local variable |
| `global.get`, `global.set` | Read / write a global variable           |
| `call`                     | Call a function by index or identifier   |
| `i32.add`, `f64.mul` …     | Arithmetic operations                    |
| `i32.eq`, `f32.ne` …       | Comparison operations                    |
| `unreachable`              | Unconditional trap                       |

Both plain-form (`i32.const 5`) and S-expression/folded form (`(i32.const 5)`) are supported.

### Definitions

- **Functions** — `(func $name (param …) (local …) …)`
- **Exports** — `(export "name")` inside a function definition
- **Function imports** — `(import "namespace" "fn_name" (func …))`

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.56 or later (stable toolchain)

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Run

```bash
cargo run
```

The demo in `src/main.rs` parses a handful of example instructions and a function import, printing the resulting AST nodes.

---

## Usage

`water` exposes its parser and AST as a library crate. Add it to your project with:

```toml
[dependencies]
water = { path = "…" }
```

### Parsing an instruction

```rust
use water::parser::parse_instruction;

let (_, instr) = parse_instruction("i32.const 42").unwrap();
println!("{instr:?}");

// Folded (S-expression) form is also accepted:
let (_, instr) = parse_instruction("(local.set $x (i32.const 10))").unwrap();
println!("{instr:?}");
```

### Parsing a function import

```rust
use water::parser::parse_function_import;

let wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
let (_, import) = parse_function_import(wat).unwrap();
println!("{import:?}");
```

### Emitting WebAssembly binary

```rust
use water::emitter::Emitter;
use water::ast::Program;

let mut output: Vec<u8> = Vec::new();
let mut emitter = Emitter::new(&mut output);
emitter.emit_program(Program { modules: vec![] }).unwrap();
// output now starts with the WASM magic bytes: 0x00 0x61 0x73 0x6D
```

---

## Architecture

### Parsing pipeline

```
WAT source (&str)
    │
    ▼
nom combinators (src/parser/)
    │  parse_instruction / parse_function / parse_function_import / …
    ▼
AST (src/ast.rs)
    │  Program → Module → Function → Instruction → Opcode → …
    ▼
Emitter (src/emitter/)
    │  Emittable<T>::emit_element
    ▼
WebAssembly binary (any std::io::Write)
```

### Key types

| Type | Description |
|------|-------------|
| `Instruction` | An opcode plus zero or more inlined argument instructions |
| `Opcode` | The actual operation: `Constant`, `VariableInstruction`, `Arithmetic`, `Comparison`, `Call`, `Unreachable` |
| `Function` | Identifier, exports, parameters, and local variables |
| `FunctionImport` | Namespace, function name, and a `Function` signature |
| `Emitter<W>` | Generic emitter that writes bytes to any `W: Write` |
| `SmallString` | Inline-optimised string (≤ 22 bytes on stack, else `Rc<str>`) |

### LEB128 encoding

WebAssembly stores all integer values in LEB128 variable-length encoding. `water` provides:

- `SignedLeb128` — wraps `i64`, implements `Emittable<SignedLeb128>`
- `UnsignedLeb128` — wraps `u64`, implements `Emittable<UnsignedLeb128>`

Both are thoroughly tested against the full signed and unsigned integer ranges.

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [`nom`](https://crates.io/crates/nom) | 7.1.1 | Parser combinators |

---

## Contributing

Contributions, issues, and feature requests are welcome. Areas that are currently incomplete or marked `TODO` in the source:

- Full module-level parsing (`src/parser/module.rs`)
- Remaining comparison opcodes (`GreaterThan`, `LessThan`, `GreaterOrEqual`, `LessOrEqual`)
- Emitting function bodies, imports, and exports to binary
- `f32` parsing (currently parsed as `f64` and cast)
- Multi-type parameter shorthand `(param i32 i32)`

---

## License

`water` is released under the [MIT License](LICENSE).  
Copyright © 2022 Vinícius Miguel.
