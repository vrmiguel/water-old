# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-16

### Added
- Initial WebAssembly Text Format (WAT) compiler implementation
- Core AST (Abstract Syntax Tree) representation for WebAssembly modules
- Parser implementation using nom for WAT syntax
  - Function parsing with parameters and local variables
  - Instruction parsing including nested/inlined arguments
  - Import statement parsing for external functions
  - Module structure parsing
- Emitter for converting AST to WebAssembly binary format
  - Arithmetic operation emission (add, sub, mul, div, rem)
  - Comparison operation emission (eq, ne, gt, lt, ge, le)
  - Constant value emission for all numerical types
  - Variable operations (local.get, local.set, local.tee, global.get, global.set)
  - Unreachable instruction support
- LEB128 encoding implementation for WebAssembly binary format
- Opcode definitions for WebAssembly instructions
- SmallString optimization for efficient string storage
- Support for WebAssembly numerical types (i32, i64, f32, f64)
- Function export declarations with duplicate name validation
- CI/CD workflow with rustfmt and clippy checks
- Comprehensive unit tests for parser functionality
- MIT License

### Features
- Parse WebAssembly Text Format into structured AST
- Support for function definitions with parameters and local variables
- Handle multiple parameters in single parameter declarations
- Function imports from external modules
- Arithmetic operations for all numerical types
- Comparison operations (equal, not equal, greater than, less than, etc.)
- Variable access through identifiers or numerical indices
- Proper f32/f64 floating-point constant parsing
- Export name uniqueness validation within functions
- Generic Value and Instruction types for flexible AST representation

### Fixed
- Correct bit masking in LEB128 low_bits function
- Proper error handling replacing unreachable!() macros
- emit_byte now returns actual write result
- Duplicate function export name detection
- f32 constant parsing using proper nom parser
- Clippy warnings for elided lifetimes and documentation
- Safety documentation for arithmetic and unreachable operations
- Transform assertions into proper nom parser errors
- Correct spelling: UnsignedDisivion → UnsignedDivision (typo remains in code)

### Refactored
- Renamed lower_bits to low_bits in leb128 module
- Transformed assertions into nom errors in import parser
- Value type refactored into generic type parameter
- Instruction type refactored to use generic type parameter
- Use nom::number::complete::f32 for parsing Float32 values
- Replaced manual loops with many1 combinator for parameter parsing
- Use BTreeSet instead of HashSet for export name validation

### Documentation
- Comprehensive inline documentation for core types and functions
- Detailed docstrings for parser functions
- Safety documentation for unsafe operations
- Function documentation across the codebase
- README with project description and goals

[0.2.0]: https://github.com/vrmiguel/water-old/releases/tag/v0.2.0
