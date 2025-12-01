# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-01

### Added

- Initial implementation of WebAssembly Text Format (WAT) parser
- Abstract Syntax Tree (AST) module for representing WebAssembly structures
- Parser module with support for:
  - Function parsing
  - Import statements parsing
  - Instruction parsing
  - Module parsing
- Opcode module with implementations for:
  - Arithmetic operations (add, sub, mul, div, rem) for i32, i64, f32, f64
  - Comparison operations (eq, ne, lt, le, gt, ge) for numerical types
  - Variable operations (local.get, local.set)
  - Constant instructions for all numerical types
  - Unreachable instruction
- Emitter module for converting AST to WebAssembly bytecode:
  - Arithmetic operation emitter
  - Constant emitter
  - Numerical value emitter
  - Unreachable instruction emitter
- LEB128 encoding implementation for WebAssembly binary format
- SmallString optimization for efficient string handling
- Support for function parameters with identifiers
- Support for multiple export instructions with duplicate name validation
- Support for function imports from external modules
- CI/CD workflow with build and test automation
- MIT License
- rustfmt configuration for code formatting

### Changed

- Bumped version from 0.1.0 to 0.2.0

## [0.1.0] - Initial Release

- Project initialization
- Basic project structure
