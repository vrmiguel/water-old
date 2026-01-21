# water (WebAssembly TExt foRmat compiler)

`water` is a tiny and performant WebAssembly Text Format (WAT) compiler written in Rust.

## Features

- **Fast parsing** of WebAssembly Text Format
- **Minimal dependencies** - uses only `nom` for parsing
- **LEB128 encoding** support for compact binary output
- **Instruction parsing** with support for nested expressions
- **Function imports** and other WAT constructs

## Installation

Add `water` to your `Cargo.toml`:

```toml
[dependencies]
water = { path = "." }
```

Or clone and build from source:

```bash
git clone <repository>
cd water
cargo build --release
```

## Usage

Parse WebAssembly Text Format instructions:

```rust
use water::parser::parse_instruction;

// Simple instruction
let result = parse_instruction("i32.const 5").unwrap();

// With parentheses
let result = parse_instruction("(i32.const 5)").unwrap();

// Local assignment
let result = parse_instruction("(local.set $idx (i32.const 5))").unwrap();
```

Parse function imports:

```rust
use water::parser::parse_function_import;

let import = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
let result = parse_function_import(import).unwrap();
```

## Project Structure

- `parser/` - WAT text parsing logic
- `emitter/` - Binary output generation
- `ast.rs` - Abstract syntax tree definitions
- `opcode.rs` - WebAssembly instruction opcodes
- `leb128.rs` - LEB128 variable-length integer encoding

## License

See LICENSE file for details.
