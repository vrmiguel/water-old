# Changelog

## [0.2.0] - 2025-10-08

### Added
- Function import parsing support
- GitHub Actions CI workflow
- `ImportFunction` AST node
- Clippy lints configuration

### Changed
- Updated Clippy lints
- Restructured parsing functions
- Parse exports functionality
- Allow any sort of arguments as "inlined" instructions in `Instruction`

### Fixed
- Doctests

## [0.1.0] - Initial Release

### Added
- Initial WebAssembly text format parser
- Support for parsing instructions (variable access, comparison, arithmetic operations)
- `Emittable` trait for code generation
- LEB128 encoding for integer literals
- IEEE 754 encoding for floating-point literals
- Support for `{i32, i64, f32, f64}.const` instructions
- Support for `unreachable` instruction
- Basic function parsing with identifiers
