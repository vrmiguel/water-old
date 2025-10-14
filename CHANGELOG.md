# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-14

### Added
- Function import parsing support
- Export parsing functionality
- GitHub Actions CI workflow for automated builds and tests
- LEB128 encoding for integer literals
- IEEE 754 bit pattern emission for floating-point numbers
- Emittable trait for code emission
- Support for unreachable instructions
- Parsing for inlined arguments in instructions
- Parsing for variable access instructions (local.get, local.set, etc.)
- Parsing for constant instructions (i32.const, i64.const, f32.const, f64.const)
- Parsing for call instructions
- Multiple WebAssembly opcodes support
- SmallString optimization for string handling
- Comprehensive instruction parsing

### Changed
- Updated Clippy lints configuration
- Restructured parsing functions for better organization
- Refactored Emittable trait implementation
- Separated arithmetic operations into dedicated struct
- Separated comparison opcodes into ComparisonOperation
- Separated variable access opcodes into VariableOperation
- Fixed proper f32 parser to avoid casting from f64

### Fixed
- Fixed doctests
- Corrected floating-point parsing implementation

## [0.1.0] - Initial Release

### Added
- Initial WebAssembly text format parser
- Basic project structure
- Function parsing capabilities
- Core AST definitions
