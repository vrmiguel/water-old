# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Added
- Complete WebAssembly Text Format compiler implementation
- Core parser module with support for:
  - Function parsing and compilation
  - Module structure parsing
  - Import declarations
  - Instruction parsing (arithmetic, comparison, constants, control flow)
- AST (Abstract Syntax Tree) representation for WebAssembly constructs
- Bytecode emitter with implementations for:
  - Arithmetic operations
  - Constant values (i32, i64, f32, f64)
  - Numerical value encoding
  - Unreachable instruction
- LEB128 encoding/decoding implementation for variable-length integer encoding
- Opcode definitions covering WebAssembly instruction set
- SmallString utility for efficient string handling
- Main CLI entry point for the compiler
- CI/CD workflow for automated building and testing
- Project documentation and README
- MIT License
- Rust formatting configuration (.rustfmt.toml)
- .gitignore for Rust projects

### Technical Details
- Built with Rust 2021 edition
- Uses `nom` parser combinator library (v7.1.1) for parsing
- Comprehensive implementation with ~2,000+ lines of code
- Support for WebAssembly Text Format to binary compilation

## [0.1.0] - Initial Release

### Added
- Initial project setup
- Basic project structure

---

[0.2.0]: https://github.com/vrmiguel/water-old/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/vrmiguel/water-old/releases/tag/v0.1.0
