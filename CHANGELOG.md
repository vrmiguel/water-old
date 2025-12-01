# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-01

### Added
- Initial release of `water` - a tiny and performant WebAssembly Text Format compiler
- Core parser module with support for:
  - WebAssembly instructions parsing (`parser/instruction.rs`)
  - Function definitions and parsing (`parser/function.rs`)
  - Import statements parsing (`parser/import.rs`)
  - Module structure parsing (`parser/module.rs`)
  - Parser utility functions (`parser/utils.rs`)
- Abstract Syntax Tree (AST) representation for WebAssembly structures (`ast.rs`)
- Emitter module for generating WebAssembly binary:
  - Arithmetic operation emitters (`emitter/arithmetic_operation.rs`)
  - Constant value emitters (`emitter/constant.rs`)
  - Numerical value emitters (`emitter/numerical_value.rs`)
  - Unreachable instruction emitter (`emitter/unreachable.rs`)
  - Emittable trait for code generation (`emitter/emittable.rs`)
- LEB128 encoding implementation for WebAssembly integer encoding (`leb128.rs`)
- WebAssembly opcode definitions and mappings (`opcode.rs`)
- Small string optimization for efficient string handling (`small_string.rs`)
- CI/CD workflow with GitHub Actions for build and test automation
- Comprehensive LICENSE file (MIT)
- Basic README with project description
- `nom` parser combinator library integration (v7.1.1)
- Rust formatting configuration (`.rustfmt.toml`)

### Features
- Parse WebAssembly Text Format (WAT) instructions
- Support for i32 constants and local variable operations
- Function import parsing with parameter type support
- Detailed error reporting using `nom::error::VerboseError`
- Binary WebAssembly generation from parsed instructions

[0.2.0]: https://github.com/vrmiguel/water-old/releases/tag/v0.2.0
