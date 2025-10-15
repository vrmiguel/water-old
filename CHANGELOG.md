# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Added

#### Core Parser Infrastructure
- WebAssembly Text Format (WAT) parser implementation using nom parser combinator library
- Support for parsing instructions (arithmetic, comparison, control flow)
- Support for parsing function imports with namespace and signature
- Support for parsing function parameters and local variables
- Support for parsing numerical types (i32, i64, f32, f64)
- Support for parsing constants (i32.const, i64.const, f32.const, f64.const)
- Support for parsing variable instructions (local.get, local.set, local.tee, global.get, global.set)
- Support for parsing arithmetic operations (add, sub, mul, div, rem)
- Support for parsing comparison operations (eq, ne, gt, lt, ge, le)
- Support for parsing the `unreachable` instruction
- Support for parsing `call` instructions with identifier or numerical index
- Module parser for parsing WebAssembly modules

#### AST (Abstract Syntax Tree)
- Complete AST definition for representing parsed WebAssembly Text Format
- Types: `NumericalType` (i32, i64, f32, f64), `NumericalValue`, `Type`
- Instructions: `Instruction`, `Opcode`, `ArithmeticOperation`, `ComparisonOperation`
- Functions: `Function`, `Parameter`, `Local`, `FunctionImport`
- Variables: `VariableOperation`, `VariableInstruction`, `ScopeKind`
- Values: `Constant`, `Index` (identifier or numerical)
- Module: `Module`, `Program`

#### WebAssembly Emitter
- Binary emitter for converting AST to WebAssembly binary format (.wasm)
- LEB128 encoding for signed and unsigned integers
- Support for emitting constants with proper type encoding
- Support for emitting arithmetic operations with correct opcodes
- Support for emitting the `unreachable` instruction
- Proper IEEE 754 floating-point encoding in little-endian byte order
- WebAssembly magic constant and version header emission

#### Opcode Mapping
- Complete opcode mapping for all supported instructions
- Arithmetic opcodes for i32, i64, f32, and f64 types
- Comparison opcodes for all numerical types
- Variable access opcodes (local/global get/set/tee)
- Control flow opcode (unreachable)
- Constant opcodes for all numerical types

#### Utilities
- `SmallString` implementation for efficient string storage
- Parser utilities for handling whitespace, parentheses, and identifiers
- Index parsing (numerical and identifier-based)

### Infrastructure

#### CI/CD
- GitHub Actions workflow for building and testing on multiple platforms (Linux, macOS, Windows)
- Clippy linting integration with warnings as errors
- Rustfmt formatting checks using nightly toolchain

#### Project Setup
- MIT License
- Rust 2021 edition
- Project configuration with rustfmt settings
- README with project description
- Dependencies: nom 7.1.1

### Technical Details
- Initial project structure with modular architecture
- Comprehensive test coverage for emitter and LEB128 encoding
- Error handling using nom's VerboseError for detailed parse errors
- Type-safe opcode conversion trait system

[0.2.0]: https://github.com/vrmiguel/water-old/compare/v0.1.0...v0.2.0
