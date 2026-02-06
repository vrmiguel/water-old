# Next Implementation Priority: Complete the Module Emitter Pipeline

## Recommendation

The single most impactful thing to implement next is **a working `Function` emitter** -- the ability to take a parsed `Function` AST node (with its body of `Instruction`s) and emit valid WebAssembly binary output for it.

## Why This Is the Highest-Priority Next Step

Right now, `water` can:
- **Parse** individual instructions, function signatures, imports, and module shells
- **Emit** individual constants, arithmetic operations, and unreachable instructions

But there is no way to connect these two halves into a working compiler. The emitter has no concept of a function body -- it can emit a single `i32.const 5` to bytes, but it cannot emit an entire function section that a WebAssembly runtime would accept.

Implementing `Emittable<Function>` (and its prerequisite, `Emittable<Instruction>`) would be the **bridge that turns `water` from a collection of utilities into an actual compiler**. Every other feature (control flow, memory, globals, tables) builds on top of being able to emit a complete function.

## What This Involves

### 1. Add a body field to `Function` in `ast.rs`

The `Function` struct currently has parameters, locals, exports, and an identifier -- but no body. It needs:

```rust
pub struct Function {
    pub identifier: Option<SmallString>,
    pub exports: Vec<SmallString>,
    pub parameters: Vec<Parameter>,
    pub local_variables: Vec<Local>,
    pub body: Vec<Instruction>,  // <-- add this
}
```

### 2. Implement `Emittable<Instruction>`

An `Instruction` contains an `Opcode` and nested `arguments`. The emitter needs to:
- Recursively emit argument instructions first (they push values onto the stack)
- Then emit the instruction's own opcode

### 3. Implement `Emittable<Function>`

Following the WebAssembly binary format specification, a function body requires:
- A byte-count prefix (LEB128-encoded size of the body)
- Local variable declarations (count + type pairs, LEB128-encoded)
- The instruction sequence (the body)
- An `0x0B` end marker

### 4. Wire up the module parser to parse function bodies

`parse_module` currently returns an empty `Module {}`. It should call `parse_function` for each function definition inside the module, and `parse_function` should parse the instruction sequence as the function body.

## What This Unlocks

With function emission working, `water` could compile a simple WAT program like:

```wat
(module
  (func (export "add") (param i32) (param i32) (result i32)
    local.get 0
    local.get 1
    i32.add))
```

into a valid `.wasm` binary -- a concrete, demonstrable milestone that proves the compiler works end-to-end. Every subsequent feature (control flow with `if`/`block`/`loop`, memory operations, globals, etc.) would then be an incremental addition on top of this foundation.
