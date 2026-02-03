# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Features

### Current Status

**Implemented:**
- ✅ Parser for WAT text format (based on nom combinators)
- ✅ Complete AST representation for WebAssembly constructs
- ✅ Basic WASM binary emitter with section support (Type, Function, Export, Code)
- ✅ All numerical types (i32, i64, f32, f64)
- ✅ All arithmetic operations (add, sub, mul, div, rem)
- ✅ All comparison operations (eq, ne, lt, le, gt, ge)
- ✅ Constants and variable operations (local.get, local.set, global.get, global.set, local.tee)
- ✅ Function definitions with parameters, locals, and return types
- ✅ Function exports
- ✅ Function imports
- ✅ LEB128 encoding for integers

**Not Yet Implemented:**
- ❌ Control flow (if/else, blocks, loops)
- ❌ Branch instructions (br, br_if, br_table)
- ❌ Memory operations (load, store, size, grow)
- ❌ Tables and indirect calls
- ❌ Multiple return values
- ❌ Globals section
- ❌ Memory section
- ❌ Start function

## Usage

Add `water` to your `Cargo.toml`:

```toml
[dependencies]
water = { path = "." }
```

### Parsing WAT

```rust
use water::parser::module;

let wat = r#"
(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  (export "add" (func $add))
)
"#;

let result = module(wat);
```

### Emitting WASM Binary

```rust
use water::emitter::Emitter;
use water::ast::*;

let program = Program {
    modules: vec![/* your parsed module */],
};

let mut output = Vec::new();
let mut emitter = Emitter::new(&mut output);
emitter.emit_program(program).unwrap();

// `output` now contains the WASM binary
```

## Architecture

water follows a classic compiler pipeline:

```
WAT Text Input → Parser → AST → Emitter → WASM Binary
```

1. **Parser** (`src/parser/`): Parses WAT text into an AST using nom combinators
2. **AST** (`src/ast.rs`): Type-safe representation of WebAssembly constructs
3. **Emitter** (`src/emitter/`): Transforms the AST into binary WASM format
4. **Opcodes** (`src/opcode.rs`): Maps AST instructions to WASM opcodes

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build --release
```

### Code Structure

- `src/parser/` - WAT text parser
- `src/ast.rs` - Abstract syntax tree definitions
- `src/emitter/` - WASM binary emitter
- `src/opcode.rs` - Opcode mappings
- `src/leb128.rs` - LEB128 integer encoding
- `src/small_string.rs` - Optimized string storage

## Contributing

This is an educational project. Contributions are welcome, especially for:
- Implementing missing WASM features
- Adding more comprehensive tests
- Improving error messages
- Optimizing performance

## License

Check the repository for license information.

