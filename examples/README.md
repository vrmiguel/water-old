# water Examples

This directory contains example WebAssembly Text Format (WAT) files that demonstrate various features of water.

## Examples

### `simple_add.wat`
A basic example showing:
- Function definition with parameters
- Local variable access
- Basic arithmetic (i32.add)
- Function export

### `arithmetic.wat`
Demonstrates arithmetic operations:
- Integer operations (add, sub, mul, div)
- Floating-point operations
- Both signed and unsigned division
- Multiple function exports

### `comparison.wat`
Demonstrates comparison operations:
- Equality checks (eq, ne)
- Relational comparisons (lt, gt, le, ge)
- Both integer and float comparisons
- Signed vs unsigned comparisons

### `locals.wat`
Shows local variable usage:
- Declaring local variables
- Getting and setting locals
- Using local.tee (set and return)
- Complex expressions with multiple locals

## Usage

To parse and compile these examples with water:

```rust
use water::parser::module;
use water::emitter::Emitter;
use std::fs;

fn main() {
    // Read the WAT file
    let wat = fs::read_to_string("examples/simple_add.wat").unwrap();

    // Parse it
    let (_, parsed_module) = module(&wat).unwrap();

    // Create a program
    let program = Program {
        modules: vec![parsed_module],
    };

    // Emit to binary
    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_program(program).unwrap();

    // Write the WASM binary
    fs::write("output.wasm", output).unwrap();
}
```

## Running in a WebAssembly Runtime

After compiling to WASM, you can run these modules in:

### Node.js

```javascript
const fs = require('fs');
const wasmBuffer = fs.readFileSync('output.wasm');

WebAssembly.instantiate(wasmBuffer).then(({ instance }) => {
  console.log(instance.exports.add(5, 3)); // Outputs: 8
});
```

### Browser

```javascript
fetch('output.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(({ instance }) => {
    console.log(instance.exports.add(5, 3)); // Outputs: 8
  });
```

### wasmtime (Rust runtime)

```bash
wasmtime output.wasm --invoke add 5 3
```

## Notes

These examples currently use features that are implemented in water:
- Basic arithmetic and comparison operations
- Local and global variables
- Function parameters and returns
- Constants

Features not yet available:
- Control flow (if/else, loops)
- Memory operations
- Tables
- Imports from other modules
