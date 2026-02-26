# water Architecture

This document describes the internal architecture of the water WebAssembly compiler.

## Overview

water is a compiler that transforms WebAssembly Text Format (WAT) into binary WebAssembly (WASM). It follows a traditional compiler pipeline with distinct parsing, representation, and code generation phases.

## Pipeline

```
┌──────────────┐      ┌──────────┐      ┌──────────┐      ┌──────────────┐
│  WAT Text    │ ───> │  Parser  │ ───> │   AST    │ ───> │  WASM Binary │
└──────────────┘      └──────────┘      └──────────┘      └──────────────┘
                           │                  │                    │
                           │                  │                    │
                      nom parsers        Type-safe           LEB128 encoding
                      combinators        structures          Section emission
```

## Components

### 1. Parser (`src/parser/`)

The parser transforms WAT text into an Abstract Syntax Tree (AST) using the `nom` parser combinator library.

**Key modules:**
- `module.rs` - Top-level module parsing
- `function.rs` - Function definitions, parameters, locals
- `instruction.rs` - Individual WAT instructions
- `import.rs` - Function imports
- `numerical_type.rs` - Type parsing (i32, i64, f32, f64)

**Design principles:**
- Uses nom's combinator-based approach for composability
- Returns `IResult<&str, T>` from parser functions
- Handles both identifier-based (`$name`) and index-based (`0`) references
- Supports S-expression syntax of WAT

**Example parser flow:**
```
"(func $add (param i32) (result i32) ...)"
          ↓
  function parser
          ↓
  Function { identifier: Some("add"), parameters: [...], ... }
```

### 2. AST (`src/ast.rs`)

The Abstract Syntax Tree provides a type-safe representation of WebAssembly constructs.

**Core types:**

- `Program` - Top-level container for modules
- `Module` - A WASM module containing functions and imports
- `Function` - Function definition with parameters, locals, return type, and body
- `Instruction` - Individual instructions with opcodes and arguments
- `Opcode` - Enum of all supported operations
- `Type` / `NumericalType` - Type system representation
- `Index` - Identifier or numerical index for references

**Key enums:**

```rust
pub enum Opcode {
    Call(Index),
    VariableInstruction(VariableOperation),
    Constant(Constant),
    Arithmetic(ArithmeticOperation),
    Comparison(ComparisonOperation),
    Unreachable(Unreachable),
}
```

**Design decisions:**
- Instructions carry their inline arguments as nested `Vec<Instruction>`
- Types are explicit and checked at the AST level
- Uses `SmallString` for efficient string storage
- Separates signed/unsigned operations at the type level

### 3. Opcodes (`src/opcode.rs`)

Maps high-level AST operations to WebAssembly bytecode opcodes.

**The `ToOpcode` trait:**
```rust
pub trait ToOpcode {
    fn to_opcode(&self) -> u8;
}
```

**Implemented for:**
- `Unreachable` → `0x00`
- `NumericalValue` → `0x41` (i32.const), `0x42` (i64.const), etc.
- `ArithmeticOperation` → Type-specific opcodes (e.g., i32.add = `0x6a`)
- `ComparisonOperation` → Type-specific comparisons (e.g., i32.lt_s = `0x48`)
- `VariableOperation` → local.get (`0x20`), local.set (`0x21`), etc.

**Opcode mapping example:**
```rust
match (NumericalType::Int32, ArithmeticInstruction::Addition) {
    => 0x6a  // i32.add
}
```

### 4. Emitter (`src/emitter/`)

Transforms the AST into binary WebAssembly format following the WebAssembly specification.

**Main struct:**
```rust
pub struct Emitter<W: Write> {
    writer: W,
}
```

**Core responsibilities:**
1. Emit WASM magic number (`\0asm`) and version
2. Generate WASM sections (Type, Function, Export, Code)
3. Encode instructions as bytecode
4. Handle LEB128 encoding for variable-length integers

**Section emission:**

Each WASM module consists of sections:

```
┌─────────────────┐
│ Magic + Version │
├─────────────────┤
│  Type Section   │  ← Function signatures
├─────────────────┤
│ Function Section│  ← Function type indices
├─────────────────┤
│ Export Section  │  ← Exported names
├─────────────────┤
│  Code Section   │  ← Function bodies
└─────────────────┘
```

**Section format:**
```
[Section ID: u8] [Section Size: varuint] [Section Data: bytes]
```

**Type Section (0x01):**
- Contains function signatures
- Format: param types → return types
- Each type: `0x60` [param count] [param types...] [result count] [result types...]

**Function Section (0x03):**
- Maps functions to their type indices
- Format: [function count] [type index per function...]

**Export Section (0x07):**
- Exports functions by name
- Format: [export count] [name length] [name bytes] [export kind: 0x00 for func] [function index]

**Code Section (0x0a):**
- Contains function bodies
- Format: [function count] { [body size] [local count] [locals...] [instructions...] [0x0b end] }

**Module emission:**
```rust
impl<W: Write> Emitter<W> {
    fn emit_module(&mut self, module: Module) -> io::Result<()> {
        self.emit_type_section(&module)?;
        self.emit_function_section(&module)?;
        self.emit_export_section(&module)?;
        self.emit_code_section(&module)?;
        Ok(())
    }
}
```

### 5. LEB128 Encoding (`src/leb128.rs`)

Implements Little Endian Base 128 variable-length integer encoding used by WebAssembly.

**Two variants:**
- `SignedLeb128` - For signed integers (i32, i64)
- `UnsignedLeb128` - For unsigned integers (sizes, counts, indices)

**Encoding algorithm:**
- Encodes 7 bits per byte
- Uses high bit (0x80) as continuation flag
- More compact than fixed-width for small numbers

**Example:**
- `128` → `[0x80, 0x01]` (needs 2 bytes instead of 4)
- `5` → `[0x05]` (needs 1 byte)

### 6. Emittable Trait (`src/emitter/emittable.rs`)

Provides a uniform interface for emitting different AST elements:

```rust
pub trait Emittable<T> {
    fn emit_element(&mut self, element: T) -> io::Result<usize>;
}
```

**Implemented for:**
- `Constant` - Emits opcode + LEB128-encoded value
- `ArithmeticOperation` - Emits single operation opcode
- `NumericalValue` - Emits LEB128 or IEEE 754 float bits
- `Unreachable` - Emits unreachable opcode
- `SignedLeb128` / `UnsignedLeb128` - Encodes integers

## Data Flow Example

Let's trace how `(i32.add)` is processed:

1. **Input**: `"i32.add"`

2. **Parser** (`parser/instruction.rs`):
   ```rust
   "i32.add" → ArithmeticOperation {
       type_: NumericalType::Int32,
       instr: ArithmeticInstruction::Addition
   }
   ```

3. **AST**:
   ```rust
   Instruction {
       opcode: Opcode::Arithmetic(ArithmeticOperation { ... }),
       arguments: vec![]
   }
   ```

4. **Opcode Mapping**:
   ```rust
   (NumericalType::Int32, ArithmeticInstruction::Addition) → 0x6a
   ```

5. **Emission**:
   ```rust
   emitter.emit_byte(0x6a) → [0x6a] in binary output
   ```

## Error Handling

**Current state:**
- Parser uses nom's `VerboseError` for detailed error reporting
- Some TODOs remain for transforming asserts into proper nom errors
- Emitter propagates IO errors through `io::Result`

**Future improvements:**
- Span-based error reporting with source locations
- Better error recovery in parser
- Semantic validation (type checking, variable scope)

## Performance Considerations

**Optimizations:**
- `SmallString` for efficient identifier storage
- Direct byte emission without intermediate allocations where possible
- LEB128 encoding reduces binary size
- Zero-copy parsing where possible with nom

**Potential improvements:**
- SmallVec for instruction arguments (mentioned in TODO)
- Parallel module compilation
- Incremental parsing and emission

## Testing Strategy

**Current coverage:**
- LEB128 encoding tests
- Constant emission tests
- Magic/version tests
- Some parser doc-tests

**Needed:**
- Comprehensive parser unit tests
- End-to-end integration tests
- Round-trip tests (parse → emit → parse)
- Error case testing
- Fuzzing for parser robustness

## Future Directions

**High priority:**
1. Control flow (blocks, if/else, loops)
2. Branch instructions
3. Memory operations
4. Proper semantic validation

**Medium priority:**
1. Tables and indirect calls
2. Multiple return values
3. Better error messages
4. Import section emission

**Lower priority:**
1. SIMD instructions
2. Reference types
3. Multi-memory
4. Optimization passes

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [WebAssembly Binary Format](https://webassembly.github.io/spec/core/binary/index.html)
- [nom Parser Combinator Library](https://github.com/rust-bakery/nom)
- [LEB128 Encoding](https://en.wikipedia.org/wiki/LEB128)
