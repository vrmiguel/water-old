# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Project Structure

### Core Files

- **src/lib.rs** - Library entry point that exports all public modules
- **src/main.rs** - CLI entry point with example usage of the parser
- **src/ast.rs** - Abstract syntax tree definitions for WebAssembly Text Format including types, instructions, functions, and modules
- **src/parser.rs** - Parser entry point that re-exports parsing functions for WAT syntax
- **src/emitter.rs** - Code emitter that converts AST to WebAssembly binary format (WASM bytecode)
- **src/opcode.rs** - WebAssembly opcode mappings that convert AST nodes to their binary opcodes
- **src/leb128.rs** - LEB128 (Little Endian Base 128) encoder for variable-length integer encoding used in WebAssembly
- **src/small_string.rs** - Memory-efficient string type that inlines strings up to 22 bytes, heap-allocates larger strings

### Parser Modules (src/parser/)

- **src/parser/function.rs** - Parses function definitions including parameters, local variables, and exports
- **src/parser/import.rs** - Parses import statements for external functions
- **src/parser/instruction.rs** - Parses WebAssembly instructions (const, call, variable operations, unreachable)
- **src/parser/module.rs** - Parses WebAssembly module structure
- **src/parser/utils.rs** - Common parsing utilities (identifiers, types, strings, parentheses)

### Emitter Modules (src/emitter/)

- **src/emitter/emittable.rs** - Trait defining the interface for types that can be emitted to WebAssembly binary
- **src/emitter/constant.rs** - Emits constant value instructions to WASM bytecode
- **src/emitter/numerical_value.rs** - Emits numerical values (i32, i64, f32, f64) to WASM bytecode
- **src/emitter/arithmetic_operation.rs** - Emits arithmetic operations (add, sub, mul, div) to WASM bytecode
- **src/emitter/unreachable.rs** - Emits the unreachable instruction to WASM bytecode

