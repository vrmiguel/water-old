# WebAssembly Text Format (WAT) Sample Files

This directory contains example WAT files that demonstrate various WebAssembly features. These files serve as test cases for the `water` compiler.

## Sample Files

### `hello.wat`
The simplest valid WAT file - an empty module. Good for testing basic module parsing.

### `arithmetic.wat`
Demonstrates basic integer arithmetic operations:
- Addition (`i32.add`)
- Subtraction (`i32.sub`)
- Multiplication (`i32.mul`)
- Division (`i32.div_s`)
- Function parameters and return values
- Exporting functions

### `functions.wat`
Shows various function definition patterns:
- Functions with no parameters
- Functions with parameters and return values
- Local variables (`local`)
- Function calls
- The `local.get` and `local.set` instructions

### `memory.wat`
Demonstrates linear memory operations:
- Memory declaration
- Loading values (`i32.load`, `i32.load8_u`)
- Storing values (`i32.store`, `i32.store8`)
- Memory size and growth (`memory.size`, `memory.grow`)
- Exporting memory

### `control_flow.wat`
Shows control flow constructs:
- If/else statements
- Blocks and loops
- Branch instructions (`br`, `br_if`)
- Early returns
- Conditional logic

### `imports_exports.wat`
Demonstrates module imports and exports:
- Importing functions from host environment
- Importing memory
- Importing global variables
- Exporting functions
- Exporting mutable globals

### `tables.wat`
Shows function tables and indirect calls:
- Table declaration
- Element segments
- Function references (`funcref`)
- Indirect function calls (`call_indirect`)
- Type definitions

### `nested_instructions.wat`
Demonstrates complex nested instruction patterns:
- Nested arithmetic expressions
- S-expressions (folded instructions)
- Nested if/else statements
- Complex multi-operation expressions

## Usage

These files can be used to test the `water` compiler as it gains more functionality. Eventually, they should all be successfully compiled to WASM binary format.

To test parsing a sample file:
```bash
water samples/hello.wat
```

## Converting to WASM

Standard WebAssembly tools can convert these to binary format for verification:
```bash
wat2wasm samples/hello.wat -o hello.wasm
```
