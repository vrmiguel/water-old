# Water - Next Steps

An assessment of the current state of the `water` WAT compiler and
recommended next steps, ordered roughly by priority and dependency.

---

## 1. Complete the Comparison Operation Opcodes

**Priority: High** | Files: `src/opcode.rs`

There are currently 16 `todo!()` markers in the `ToOpcode` implementation
for `ComparisonOperation`. These will panic at runtime if any comparison
other than `eq` or `ne` is encountered. The missing opcodes are
straightforward to fill in from the WASM spec:

| Type   | Instruction    | Signed Opcode | Unsigned Opcode |
|--------|----------------|---------------|-----------------|
| i32    | greater_than   | 0x4a (gt_s)   | 0x4b (gt_u)     |
| i32    | less_than      | 0x48 (lt_s)   | 0x49 (lt_u)     |
| i32    | greater_or_eq  | 0x4e (ge_s)   | 0x4f (ge_u)     |
| i32    | less_or_eq     | 0x4c (le_s)   | 0x4d (le_u)     |
| i64    | greater_than   | 0x55 (gt_s)   | 0x56 (gt_u)     |
| i64    | less_than      | 0x53 (lt_s)   | 0x54 (lt_u)     |
| i64    | greater_or_eq  | 0x5a (ge_s)   | 0x5b (ge_u)     |
| i64    | less_or_eq     | 0x58 (le_s)   | 0x59 (le_u)     |
| f32    | greater_than   | 0x5e          |                 |
| f32    | less_than      | 0x5d          |                 |
| f32    | greater_or_eq  | 0x60          |                 |
| f32    | less_or_eq     | 0x5f          |                 |
| f64    | greater_than   | 0x64          |                 |
| f64    | less_than      | 0x63          |                 |
| f64    | greater_or_eq  | 0x66          |                 |
| f64    | less_or_eq     | 0x65          |                 |

**Note:** Integer comparisons in WASM have signed (`_s`) and unsigned (`_u`)
variants. The current `ComparisonInstruction` enum does not distinguish
between signed and unsigned. This needs to be addressed -- either by adding
`SignedGreaterThan` / `UnsignedGreaterThan` variants (mirroring how
`ArithmeticInstruction` handles `SignedDivision` / `UnsignedDivision`), or
by introducing a `Signedness` field on the `ComparisonOperation` struct.

---

## 2. Implement the Module Structure and Section Emission

**Priority: High** | Files: `src/ast.rs`, `src/emitter.rs`, new emitter submodules

The `Module` struct is currently empty (`// TODO`), and `emit_program` only
writes the magic number and version tag. A valid WASM binary requires
emitting structured **sections** in a specific order:

1. **Type Section** (section id 1) -- function signatures
2. **Import Section** (section id 2) -- imported functions/memories/etc.
3. **Function Section** (section id 3) -- index into the type section
4. **Table Section** (section id 4)
5. **Memory Section** (section id 5)
6. **Global Section** (section id 6)
7. **Export Section** (section id 7) -- exported functions/memories/etc.
8. **Start Section** (section id 8)
9. **Element Section** (section id 9)
10. **Code Section** (section id 10) -- function bodies
11. **Data Section** (section id 11)

At minimum, implementing the **Type**, **Function**, **Export**, and
**Code** sections would allow `water` to compile simple WAT modules into
valid, runnable `.wasm` files. This is the single largest piece of work
needed to make the compiler functional end-to-end.

Suggested approach:
- Populate the `Module` struct with fields for functions, imports, exports.
- Add an `Emittable` implementation for `Module` that emits sections.
- Each section gets its own emitter submodule.

---

## 3. Add a CLI Interface

**Priority: High** | Files: `src/main.rs`, `Cargo.toml`

The current `main.rs` is a test harness with hardcoded examples. To make
`water` usable as a tool, it needs a proper CLI:

- Accept an input `.wat` file path as an argument.
- Parse the file contents into the AST.
- Emit the resulting `.wasm` binary to an output file.
- Handle and report errors clearly to the user.

Consider using `clap` for argument parsing, or keep it dependency-free
with `std::env::args()` since the interface is simple.

---

## 4. Implement Function Body Parsing

**Priority: High** | Files: `src/parser/function.rs`

The current function parser (`parse_function`) extracts the function
signature (identifier, exports, parameters, locals) but does **not** parse
the function body (the instruction sequence). The `Function` struct also
lacks a `body: Vec<Instruction>` field.

Adding body parsing is required before functions can be emitted as WASM
code sections. This means:
- Add a `body` field to the `Function` struct.
- After parsing parameters and locals, parse the remaining instructions
  in the function body using the existing `parse_instruction`.
- Handle nested/inlined instruction forms.

---

## 5. Fix the f32 Constant Parsing Hack

**Priority: Medium** | Files: `src/parser/instruction.rs`

As noted in a TODO comment, `f32.const` values are currently parsed as
`f64` and then cast to `f32`. This can silently lose precision. The fix
is straightforward: use `nom::number::complete::float` or parse the text
with Rust's `str::parse::<f32>()` directly.

---

## 6. Add Signed/Unsigned Distinction for Integer Comparisons

**Priority: Medium** | Files: `src/ast.rs`, `src/parser/instruction.rs`, `src/opcode.rs`

WASM integer comparison instructions come in signed (`_s`) and unsigned
(`_u`) variants (e.g., `i32.gt_s` vs `i32.gt_u`). The current
`ComparisonInstruction` enum has no way to express this distinction. This
blocks correct opcode emission for comparisons.

Options:
- Add a `Signedness` enum (`Signed`, `Unsigned`) and a field on
  `ComparisonOperation`, similar to the approach for division.
- Or expand the enum variants: `SignedGreaterThan`, `UnsignedGreaterThan`, etc.

The parser will also need to distinguish `gt_s` from `gt_u` in the
instruction text.

---

## 7. Improve Error Handling Consistency

**Priority: Medium** | Files: `src/parser/import.rs`, various

Some parser functions use `assert!` macros instead of returning proper
`nom` errors (e.g., in `parse_function_import`). These will panic instead
of producing useful error messages. Convert all assertions to
`nom::error::VerboseError` returns for consistent, user-friendly error
reporting throughout the parser.

---

## 8. Handle Multi-Value Parameters

**Priority: Medium** | Files: `src/parser/function.rs`

WAT supports inline multi-value parameter declarations like
`(param f32 f32)`, which declares two unnamed parameters of the same type.
The current parser only handles `(param $name type)` or `(param type)`.
Adding support for the multi-value form improves spec compliance.

---

## 9. Add Export Validation

**Priority: Medium** | Files: `src/parser/function.rs` or a new validation pass

As noted in a TODO: WASM allows multiple export instructions on a single
function, but exported names must be unique across the entire module. This
validation should be implemented either during parsing or as a separate
validation pass after parsing.

---

## 10. Add Emitter Tests for Arithmetic Operations

**Priority: Medium** | Files: `src/emitter/arithmetic_operation.rs`

The `Emittable<ArithmeticOperation>` implementation has no tests (noted
with a TODO comment). Tests should verify that each arithmetic operation
produces the correct opcode byte.

---

## 11. Implement Control Flow Instructions

**Priority: Medium-Low** | Files: `src/ast.rs`, `src/parser/instruction.rs`, `src/opcode.rs`

The compiler currently has no support for control flow. These are
essential for non-trivial programs:

- `block` / `end` -- structured block
- `loop` / `end` -- loop construct
- `if` / `else` / `end` -- conditional
- `br` / `br_if` / `br_table` -- branching
- `return` -- early return
- `nop` -- no operation

These require extending the AST, parser, and opcode modules.

---

## 12. Implement Memory Instructions

**Priority: Medium-Low** | Files: AST, parser, opcode, emitter

Memory load/store instructions (`i32.load`, `i32.store`, etc.) are needed
for any program that works with linear memory. This includes:

- `memory` declarations in modules
- Load instructions: `i32.load`, `i64.load`, `f32.load`, `f64.load`
  and their variants (`load8_s`, `load8_u`, `load16_s`, etc.)
- Store instructions: `i32.store`, `i64.store`, `f32.store`, `f64.store`
  and their variants

---

## 13. End-to-End Integration Tests

**Priority: Medium** | Files: new `tests/` directory

Once section emission is working, add integration tests that:

1. Parse a `.wat` string.
2. Emit it to a `.wasm` byte vector.
3. Validate the output against a known-good `.wasm` binary (e.g., one
   produced by `wat2wasm` from the WebAssembly Binary Toolkit).

This would catch regressions and prove correctness across the full
pipeline.

---

## 14. Improve the README and Add Usage Examples

**Priority: Low** | Files: `README.md`

The README is very brief. It would benefit from:

- A description of what WAT is and why someone would use `water`.
- Build/install instructions.
- Usage examples showing input `.wat` and the corresponding compilation
  command.
- A feature status table showing what's implemented vs. planned.
- Links to the WebAssembly spec and related tools.

---

## 15. Consider Investigating SmallVec for Instruction Arguments

**Priority: Low** | Files: `src/ast.rs`, `Cargo.toml`

As noted in a TODO, `Instruction.arguments` uses `Vec<Instruction>`. Most
instructions have 0-2 arguments, so `SmallVec<[Instruction; 2]>` from the
`smallvec` crate could avoid heap allocation in the common case. This is
a minor optimization but aligns with the project's goal of being
performant.

---

## Summary

The most impactful work items that would move `water` toward being a
functional compiler are:

1. **Complete comparison opcodes** (unblocks all comparison operations)
2. **Implement Module structure + section emission** (required for valid .wasm output)
3. **Add function body parsing** (required to compile function code)
4. **Add a CLI** (required for actual usability)

These four items together would allow `water` to take a `.wat` file
containing simple functions and produce a valid, runnable `.wasm` binary.
