# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Added

#### Core Infrastructure
- Initial project structure with Cargo.toml configuration
- MIT License
- README with project description
- .gitignore for Rust projects
- rustfmt configuration with custom code style settings
- GitHub Actions CI/CD workflow for build, test, clippy, and rustfmt checks across multiple platforms (macOS, Windows, Linux)

#### Abstract Syntax Tree (AST)
- Complete AST implementation for WebAssembly Text Format
- `Program` and `Module` structures for representing WASM modules
- Type system with `NumericalType` enum supporting i32, i64, f32, and f64
- `NumericalValue` enum for carrying actual typed values
- `Function` structure with support for parameters, local variables, and exports
- `Parameter` and `Local` structures with optional identifiers
- `Instruction` structure for representing WASM instructions with arguments
- `FunctionImport` for handling function imports from external modules
- Comprehensive `Opcode` enum supporting:
  - Function calls
  - Variable operations (local/global get/set/tee)
  - Numerical constants
  - Arithmetic operations (add, sub, mul, div, rem)
  - Comparison operations (eq, ne, gt, lt, ge, le)
  - Unreachable instruction
- `Index` enum for identifier or numerical indexing
- `ScopeKind` enum distinguishing between local and global scope

#### Parser
- Parser implementation using the `nom` parser combinator library
- Instruction parsing with support for both parenthesized and non-parenthesized forms
- Function import parsing
- Utility functions for parsing WebAssembly Text Format syntax

#### Emitter (Code Generation)
- WASM binary emitter infrastructure
- Support for emitting WASM magic number and version
- `Emittable` trait for converting AST elements to WASM bytecode
- Emitters for:
  - Numerical constants (i32.const, i64.const, f32.const, f64.const)
  - Arithmetic operations
  - Unreachable instruction
- IEEE 754 compliant floating-point encoding in little-endian format

#### LEB128 Encoding
- Signed LEB128 (Little Endian Base 128) encoder for integer literals
- Unsigned LEB128 encoder
- Comprehensive test suite for LEB128 encoding with edge cases

#### Opcode System
- Complete opcode mapping for WebAssembly instructions
- `ToOpcode` trait for converting AST elements to WASM opcodes
- Opcode definitions for all supported instructions

#### Utilities
- `SmallString` utility for efficient string handling

#### Testing
- Comprehensive test suite for emitters
- Tests for LEB128 encoding/decoding
- Tests for constant emission
- Tests for unreachable instruction

### Dependencies
- nom 7.1.1 - Parser combinator library

## [0.1.0] - Initial

### Added
- Empty project structure (no functional code)

[0.2.0]: https://github.com/vrmiguel/water-old/compare/3343dd4...880596b
