# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Added

#### Core Features
- Initial WebAssembly Text Format (WAT) to WebAssembly binary compiler implementation
- Complete parser for WebAssembly Text Format including:
  - Function definitions with parameters and local variables
  - Import statements
  - Module declarations
  - Instruction parsing with support for arithmetic, comparison, and control flow operations
- Binary emitter for generating WebAssembly bytecode
- LEB128 encoding/decoding implementation for efficient integer representation
- Support for arithmetic operations (add, sub, mul, div, rem)
- Support for comparison opcodes (eq, ne, lt, gt, le, ge for both signed and unsigned)
- Constant value handling (i32.const, f32.const)
- Export instruction support with duplicate name validation
- Generic type system for Instructions and Values
- Module struct to store module components

#### Testing & Quality
- Comprehensive parser unit tests
- Tests for Emittable<ArithmeticOperation>
- Python test file for binary validation
- CI/CD workflow with build and test automation
- Clippy linting integration
- Rustfmt code formatting

#### Documentation
- Comprehensive function documentation across the codebase
- Detailed docstrings for:
  - Parser functions (parse_instruction, parse_opcode, parse_string)
  - Emitter functions (emit_program, emit_byte)
  - LEB128 functions (low_bits)
  - Core types and functions
- Project README with description and usage instructions
- Localized README translations in multiple languages:
  - Spanish
  - French
  - Chinese (multiple variants)
  - Russian
  - Japanese
  - Cantonese
  - Taiwanese
  - Swahili
  - Thai
  - Swedish
- Cross-references to related projects in READMEs

### Fixed
- Typo in ArithmeticInstruction enum
- Spelling error: UnsignedDisivion → UnsignedDivision
- Correct bit masking in low_bits function
- Proper f32 parser implementation (using nom::number::complete::f32 instead of casting from f64)
- emit_byte now returns actual write result instead of always returning 1
- Replaced unreachable!() macros with proper error handling
- Resolved clippy lint warnings including:
  - Elided lifetime annotations
  - Doc comment indentation
  - Incorrect #[must_use] annotations on trait methods
- Fixed nom error handling in import parser
- Removed redundant SmallString::new call in parse_index
- Proper handling of function parameters and removal of unused imports
- Code formatting issues across multiple files
- Removed accidentally committed binary core dump files
- CI workflow fixes:
  - Updated actions/checkout from v2 to v4
  - Updated cargo rustfmt dependency
  - Added rustfmt and clippy configs
  - Added rustfmt configuration for nightly formatting
  - Corrected formatting issues for clippy and fmt checks
  - Resolved cargo fmt check failures

### Changed
- Transformed assertions into nom errors in function import parser for better error handling
- Refactored lower_bits to low_bits in leb128 module (naming consistency)
- Refactored Instruction to use a generic type parameter
- Refactored Value into a generic type
- Transformed arguments into a generic Value type
- Use BTreeSet instead of HashSet for export name validation (deterministic ordering)
- Replace manual loops with many1 combinator for parameter and local parsing
- Enhanced error handling throughout parser
- Improved safety documentation in arithmetic and unreachable operations

### Security
- Added check for duplicate function export names to prevent conflicts
- Validation of export name uniqueness within functions
- Support for multiple export instructions with duplicate name checking

## [0.1.0] - Initial Release

### Added
- Initial project structure
- Basic WebAssembly compiler foundation
- MIT License
- Project configuration files (.gitignore, .rustfmt.toml)
- Cargo.toml with nom dependency for parsing
