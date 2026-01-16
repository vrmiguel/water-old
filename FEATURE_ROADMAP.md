# Feature Implementation Roadmap for Water

## Executive Summary

This document outlines a plan for implementing **WebAssembly Memory Operations** as a meaningful and impactful feature for the water compiler. This feature will significantly enhance the compiler's capabilities by enabling programs to manage linear memory, which is essential for real-world WebAssembly applications.

## Project Context

**Water** is a Rust-based WebAssembly Text Format (WAT) compiler that parses WAT code and emits WASM binary format. The project currently supports:
- Core numeric operations (i32, i64, f32, f64)
- Arithmetic and comparison operations
- Variable operations (local/global get/set/tee)
- Function imports and calls
- Basic module structure

## Proposed Feature: WebAssembly Memory Operations

### Why This Feature?

Memory operations are foundational to WebAssembly's capabilities. Without them, programs cannot:
- Work with strings or byte arrays
- Implement data structures (arrays, structs)
- Interface with external memory
- Build practical applications (parsers, encoders, game engines)

Currently, water can only handle computational operations on the stack. Adding memory support will unlock the next tier of WebAssembly functionality.

### Feature Scope

Implement the complete set of WebAssembly linear memory instructions:

1. **Memory Declaration & Management**
   - `(memory <limits>)` - Memory section declaration
   - `(data <offset> <bytes>)` - Data section for initialization
   - Memory import/export

2. **Load Instructions**
   - `i32.load` / `i64.load` - Load 32/64-bit integers
   - `f32.load` / `f64.load` - Load floats
   - `i32.load8_s` / `i32.load8_u` - Load bytes (signed/unsigned)
   - `i32.load16_s` / `i32.load16_u` - Load 16-bit values
   - `i64.load8_s/u`, `i64.load16_s/u`, `i64.load32_s/u` - Load with extension

3. **Store Instructions**
   - `i32.store` / `i64.store` - Store 32/64-bit integers
   - `f32.store` / `f64.store` - Store floats
   - `i32.store8` / `i32.store16` - Store truncated values
   - `i64.store8` / `i64.store16` / `i64.store32` - Store truncated 64-bit values

4. **Memory Size Operations**
   - `memory.size` - Get current memory size in pages
   - `memory.grow` - Grow memory by pages

### Implementation Plan

#### Phase 1: AST Extensions (Est. Complexity: Medium)

**Goal**: Extend the abstract syntax tree to represent memory constructs.

**Files to Modify**:
- `src/ast.rs`

**Tasks**:
1. Add `Memory` struct to represent memory declarations
   ```rust
   pub struct Memory {
       pub limits: MemoryLimits,
       pub id: Option<SmallString>,
   }

   pub struct MemoryLimits {
       pub min: u32,  // minimum pages
       pub max: Option<u32>,  // optional maximum pages
   }
   ```

2. Add `Data` struct for data sections
   ```rust
   pub struct DataSegment {
       pub memory_index: u32,
       pub offset: Vec<Instruction>,  // constant expression
       pub data: Vec<u8>,
   }
   ```

3. Extend `Module` struct to include memory and data
   ```rust
   pub struct Module {
       pub memories: Vec<Memory>,
       pub data: Vec<DataSegment>,
       // ... existing fields
   }
   ```

4. Add memory operation variants to `Opcode` enum
   ```rust
   pub enum Opcode {
       // ... existing variants
       MemoryLoad(MemoryLoadOp),
       MemoryStore(MemoryStoreOp),
       MemorySize,
       MemoryGrow,
   }

   pub struct MemoryLoadOp {
       pub ty: NumericalType,
       pub size: Option<LoadSize>,  // None for full load
       pub signed: bool,
       pub align: u32,
       pub offset: u32,
   }

   pub struct MemoryStoreOp {
       pub ty: NumericalType,
       pub size: Option<StoreSize>,  // None for full store
       pub align: u32,
       pub offset: u32,
   }

   pub enum LoadSize {
       Bit8,
       Bit16,
       Bit32,
   }

   pub enum StoreSize {
       Bit8,
       Bit16,
       Bit32,
   }
   ```

**Testing Strategy**:
- Unit tests for each new struct's construction
- Derive tests for Debug, Clone, PartialEq traits

#### Phase 2: Opcode Mapping (Est. Complexity: Low)

**Goal**: Map memory operations to WebAssembly binary opcodes.

**Files to Modify**:
- `src/opcode.rs`

**Tasks**:
1. Implement `ToOpcode` for memory load operations
   - i32.load: 0x28
   - i64.load: 0x29
   - f32.load: 0x2a
   - f64.load: 0x2b
   - i32.load8_s: 0x2c
   - i32.load8_u: 0x2d
   - i32.load16_s: 0x2e
   - i32.load16_u: 0x2f
   - i64.load8_s: 0x30
   - i64.load8_u: 0x31
   - i64.load16_s: 0x32
   - i64.load16_u: 0x33
   - i64.load32_s: 0x34
   - i64.load32_u: 0x35

2. Implement `ToOpcode` for memory store operations
   - i32.store: 0x36
   - i64.store: 0x37
   - f32.store: 0x38
   - f64.store: 0x39
   - i32.store8: 0x3a
   - i32.store16: 0x3b
   - i64.store8: 0x3c
   - i64.store16: 0x3d
   - i64.store32: 0x3e

3. Implement `ToOpcode` for memory size operations
   - memory.size: 0x3f
   - memory.grow: 0x40

**Testing Strategy**:
- Unit tests verifying correct opcode values
- Test coverage for all memory instruction variants

#### Phase 3: Parser Implementation (Est. Complexity: High)

**Goal**: Parse WAT memory syntax into AST nodes.

**Files to Create/Modify**:
- `src/parser/memory.rs` (new file)
- `src/parser/data.rs` (new file)
- `src/parser/instruction.rs` (modify)
- `src/parser/module.rs` (modify)
- `src/parser/parser.rs` (modify)

**Tasks**:

1. **Create `src/parser/memory.rs`**
   ```rust
   // Parse: (memory $name 1)
   // Parse: (memory 1 10)  // min 1, max 10 pages
   pub fn parse_memory(input: &str) -> IResult<&str, Memory> {
       // Implementation using nom combinators
   }

   pub fn parse_memory_limits(input: &str) -> IResult<&str, MemoryLimits> {
       // Parse min and optional max page counts
   }
   ```

2. **Create `src/parser/data.rs`**
   ```rust
   // Parse: (data (i32.const 0) "hello")
   // Parse: (data (i32.const 100) "\00\01\02\03")
   pub fn parse_data_segment(input: &str) -> IResult<&str, DataSegment> {
       // Parse memory index, offset expression, and data bytes
   }

   fn parse_data_string(input: &str) -> IResult<&str, Vec<u8>> {
       // Convert string literals to byte arrays
   }
   ```

3. **Extend `src/parser/instruction.rs`**
   ```rust
   pub fn parse_memory_load(input: &str) -> IResult<&str, Instruction> {
       // Parse: i32.load offset=4 align=4
       // Parse: (i32.load offset=4 align=4)
       // Parse: i32.load8_u offset=0
   }

   pub fn parse_memory_store(input: &str) -> IResult<&str, Instruction> {
       // Parse: i32.store offset=4 align=4
       // Parse: (i32.store offset=4 align=4)
   }

   pub fn parse_memory_size(input: &str) -> IResult<&str, Instruction> {
       // Parse: memory.size
   }

   pub fn parse_memory_grow(input: &str) -> IResult<&str, Instruction> {
       // Parse: memory.grow
   }

   // Modify parse_instruction to include memory operations
   pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
       alt((
           // ... existing parsers
           parse_memory_load,
           parse_memory_store,
           parse_memory_size,
           parse_memory_grow,
       ))(input)
   }
   ```

4. **Update `src/parser/module.rs`**
   ```rust
   // Extend module parser to recognize (memory ...) and (data ...)
   pub fn parse_module(input: &str) -> IResult<&str, Module> {
       // Parse memories and data sections alongside functions
   }
   ```

5. **Update `src/parser/parser.rs`**
   ```rust
   // Add memory and data parsing to module exports
   mod memory;
   mod data;
   pub use memory::*;
   pub use data::*;
   ```

**Testing Strategy**:
- Test parsing of all memory instruction variants
- Test with various offset and alignment values
- Test both S-expression and plain syntax styles
- Test memory declarations with min/max limits
- Test data segments with string and hex byte literals
- Test error cases (invalid offsets, alignment values)

#### Phase 4: Binary Emission (Est. Complexity: Medium-High)

**Goal**: Emit WASM binary format for memory operations.

**Files to Create/Modify**:
- `src/emitter/memory.rs` (new file)
- `src/emitter/emitter.rs` (modify)
- `src/emitter/emittable.rs` (modify)

**Tasks**:

1. **Create `src/emitter/memory.rs`**
   ```rust
   // Implement emission for Memory and DataSegment
   impl<W: Write> Emittable<W> for Memory {
       fn emit(&self, emitter: &mut Emitter<W>) -> Result<()> {
           // Emit memory section (section id: 5)
           // Format: limits_type, min, [max]
       }
   }

   impl<W: Write> Emittable<W> for DataSegment {
       fn emit(&self, emitter: &mut Emitter<W>) -> Result<()> {
           // Emit data section (section id: 11)
           // Format: memory_index, offset_expr, data_bytes
       }
   }
   ```

2. **Implement memory instruction emission**
   ```rust
   impl<W: Write> Emittable<W> for MemoryLoadOp {
       fn emit(&self, emitter: &mut Emitter<W>) -> Result<()> {
           // Emit: opcode, align (as LEB128), offset (as LEB128)
       }
   }

   impl<W: Write> Emittable<W> for MemoryStoreOp {
       fn emit(&self, emitter: &mut Emitter<W>) -> Result<()> {
           // Emit: opcode, align (as LEB128), offset (as LEB128)
       }
   }
   ```

3. **Update `src/emitter/emitter.rs`**
   ```rust
   impl<W: Write> Emitter<W> {
       pub fn emit_memory_section(&mut self, memories: &[Memory]) -> Result<()> {
           if memories.is_empty() {
               return Ok(());
           }
           // Emit section 5 with memory definitions
       }

       pub fn emit_data_section(&mut self, data: &[DataSegment]) -> Result<()> {
           if data.is_empty() {
               return Ok(());
           }
           // Emit section 11 with data segments
       }
   }
   ```

4. **WASM Binary Section Structure**

   Memory Section (ID: 5):
   ```
   [section_id: u8 = 5]
   [section_size: LEB128]
   [memory_count: LEB128]
   for each memory:
     [limits_flags: u8]  // 0x00 = min only, 0x01 = min+max
     [min: LEB128]
     [max: LEB128]  // if flag 0x01
   ```

   Data Section (ID: 11):
   ```
   [section_id: u8 = 11]
   [section_size: LEB128]
   [data_count: LEB128]
   for each segment:
     [memory_index: LEB128]
     [offset_expr: instruction_sequence + 0x0b (end)]
     [data_size: LEB128]
     [data_bytes: [u8]]
   ```

**Testing Strategy**:
- Test emission of memory sections with various limit configurations
- Test data segment emission with different offsets and data
- Compare emitted bytes against reference WASM files (from wat2wasm)
- Test round-trip: parse WAT → emit WASM → verify with wasm-objdump

#### Phase 5: Integration & End-to-End Testing (Est. Complexity: Medium)

**Goal**: Ensure all components work together correctly.

**Files to Modify**:
- `src/main.rs`
- Create `tests/memory_tests.rs` (integration tests)

**Tasks**:

1. **Add comprehensive integration tests**
   ```rust
   // Test simple memory allocation and access
   #[test]
   fn test_memory_basic() {
       let wat = r#"
       (module
           (memory 1)
           (func $store_value (param $addr i32) (param $val i32)
               local.get $addr
               local.get $val
               i32.store
           )
       )
       "#;

       let result = compile_wat(wat);
       assert!(result.is_ok());
       // Verify emitted binary structure
   }

   // Test data segment initialization
   #[test]
   fn test_data_segment() {
       let wat = r#"
       (module
           (memory 1)
           (data (i32.const 0) "Hello, WebAssembly!")
       )
       "#;

       let result = compile_wat(wat);
       assert!(result.is_ok());
   }

   // Test all load/store variants
   #[test]
   fn test_all_memory_operations() {
       // Test i32.load, i32.load8_s, i32.load8_u, etc.
       // Test i32.store, i32.store8, i32.store16, etc.
       // Test memory.size and memory.grow
   }
   ```

2. **Add example WAT files to repository**
   - Create `examples/memory/` directory
   - `simple_memory.wat` - Basic load/store example
   - `data_segment.wat` - Data initialization example
   - `memory_grow.wat` - Dynamic memory growth example

3. **Update CLI to handle memory sections**
   - Ensure `src/main.rs` properly emits memory sections
   - Add verbose output option to show parsed memory sections

4. **Create validation against reference tools**
   ```bash
   # Script to validate output
   wat2wasm examples/memory/simple_memory.wat -o reference.wasm
   cargo run -- examples/memory/simple_memory.wat -o output.wasm
   wasm-objdump -d reference.wasm > reference.txt
   wasm-objdump -d output.wasm > output.txt
   diff reference.txt output.txt
   ```

**Testing Strategy**:
- Cross-reference output with WABT tools (wat2wasm, wasm-objdump)
- Test with wasmtime or wasmer to verify executability
- Test edge cases (maximum memory size, alignment edge cases)
- Performance benchmarks (parsing speed, emission speed)

#### Phase 6: Documentation & Examples (Est. Complexity: Low)

**Goal**: Document the new feature for users and contributors.

**Files to Create/Modify**:
- `README.md`
- `docs/MEMORY_OPERATIONS.md` (new file)
- Code documentation (inline)

**Tasks**:

1. **Update README.md**
   - Add memory operations to supported features list
   - Add example usage with memory operations
   - Update status badges if applicable

2. **Create comprehensive documentation**
   ```markdown
   # Memory Operations in Water

   ## Overview
   Water now supports WebAssembly linear memory operations...

   ## Memory Declaration
   ...

   ## Load/Store Instructions
   ...

   ## Data Segments
   ...

   ## Examples
   ...
   ```

3. **Add inline documentation**
   - Rustdoc comments for all new public APIs
   - Code examples in documentation
   - Link to WebAssembly specification sections

4. **Update CHANGELOG**
   - Document breaking changes (if any)
   - List all new capabilities
   - Provide migration guide

**Testing Strategy**:
- Run `cargo doc` and verify documentation builds
- Test all code examples in documentation
- Get review from potential users

### Implementation Order & Dependencies

```
Phase 1: AST Extensions
    ↓
Phase 2: Opcode Mapping ← (can partially parallel with Phase 1)
    ↓
Phase 3: Parser Implementation
    ↓
Phase 4: Binary Emission
    ↓
Phase 5: Integration & Testing
    ↓
Phase 6: Documentation
```

### Success Metrics

1. **Functional Completeness**
   - All 28 memory instructions parse correctly
   - All memory instructions emit correct WASM bytecode
   - Memory and data sections emit according to spec

2. **Correctness**
   - Output matches wat2wasm for all test cases
   - Generated WASM validates with wasm-validate
   - Generated WASM executes correctly in wasmtime/wasmer

3. **Code Quality**
   - 100% test coverage for new code
   - All clippy warnings resolved
   - Documentation for all public APIs

4. **Performance**
   - No regression in parsing speed for non-memory code
   - Memory instruction parsing within 10% of other instructions

### Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Binary format complexity | High | Use reference tools for validation, implement incrementally |
| Parser complexity for offset/align syntax | Medium | Start with simple cases, add variants iteratively |
| Data segment offset expressions | Medium | Reuse existing constant expression parser |
| LEB128 encoding edge cases | Medium | Comprehensive unit tests, use existing leb128 module |
| Breaking changes to Module struct | Low | Carefully version and document changes |

### Alternative Approaches Considered

1. **Alternative: Control Flow Instructions First**
   - Would add block, loop, if/else, br, br_if, br_table
   - Pros: Also fundamental to WASM
   - Cons: More complex semantics, harder to test without memory

2. **Alternative: Table Operations**
   - Would add tables, elem sections, call_indirect
   - Pros: Enables indirect function calls
   - Cons: Less commonly used than memory in practice

3. **Alternative: Multi-Memory Proposal**
   - Would support multiple linear memories (recent WASM proposal)
   - Pros: Forward-looking
   - Cons: Not standardized yet, complicates implementation

**Decision**: Memory operations are the most impactful next feature because:
- Essential for practical programs (strings, arrays, data structures)
- Well-defined in the spec (v1 feature, stable)
- Complements existing instruction set well
- Natural progression in compiler implementation

### Timeline Estimate

Assuming one developer working part-time:

- **Phase 1 (AST)**: 2-3 days
- **Phase 2 (Opcodes)**: 1 day
- **Phase 3 (Parsing)**: 5-7 days
- **Phase 4 (Emission)**: 4-6 days
- **Phase 5 (Integration)**: 3-4 days
- **Phase 6 (Documentation)**: 2-3 days

**Total**: 17-24 days of focused development

### Future Extensions

Once memory operations are implemented, natural follow-ups include:

1. **Control Flow Instructions**
   - Block, loop, if/else
   - Branch instructions (br, br_if, br_table)
   - Required for Turing-completeness

2. **Table Operations**
   - Table declarations
   - Element sections
   - call_indirect for dynamic dispatch

3. **Global Variables**
   - Global section (currently only supports local variables)
   - Mutable and immutable globals
   - Global imports/exports

4. **Reference Types**
   - funcref, externref
   - Modern WASM feature for GC integration

5. **SIMD Instructions**
   - 128-bit vector operations
   - Performance-critical applications

6. **Bulk Memory Operations**
   - memory.copy, memory.fill
   - table.copy, table.fill
   - Optimization for block operations

## Conclusion

Implementing WebAssembly memory operations is a high-value feature that will significantly expand water's capabilities. The implementation is well-scoped, follows the existing architecture patterns, and builds naturally on the current codebase. With proper testing and validation against reference tools, this feature will position water as a more complete and useful WAT compiler.

The phased approach allows for incremental progress with validation at each step, reducing risk and ensuring quality throughout the implementation process.

---

**Document Version**: 1.0
**Date**: 2026-01-16
**Author**: Claude Code Assistant
**Status**: Proposed
