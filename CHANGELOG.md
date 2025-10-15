# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Changed
- Simplified README documentation by removing usage instructions, documentation links, and contributing sections
- Cleaned up repository by removing binary artifacts

### Removed
- Removed "Getting Help" section from README
- Removed usage examples and build instructions
- Removed documentation section describing parser modules
- Removed contributing guidelines
- Removed support section
- Removed accidentally committed binary `core` file

## [0.1.0] - 2025-10-15

### Added
- Initial release of water - WebAssembly Text Format compiler
- Core WebAssembly parser implementation using nom parser combinator library
- Support for parsing WebAssembly instructions
- Support for parsing import declarations
- Support for parsing module structure
- Support for parsing functions
- LEB128 encoding implementation
- AST (Abstract Syntax Tree) representation for WebAssembly constructs
- Opcode definitions for WebAssembly instructions
- Emitter system for generating WebAssembly binary code
- Support for arithmetic operations
- Support for constant values
- Support for numerical values
- Support for unreachable instruction
- Comprehensive parser utilities
- Small string optimization implementation
- Project uses Rust edition 2021
- Dependency on nom 7.1.1 for parsing

[0.2.0]: https://github.com/vrmiguel/water-old/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/vrmiguel/water-old/releases/tag/v0.1.0
