# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-15

### Changed
- Bumped version from 0.1.0 to 0.2.0

## [0.1.0] - Initial Release

### Added
- Initial implementation of water (WebAssembly TExt foRmat compiler)
- Core WebAssembly Text Format parsing functionality
- AST (Abstract Syntax Tree) representation for WebAssembly modules
- Emitter for converting AST to binary WebAssembly
- LEB128 encoding support
- Parser for WebAssembly instructions, functions, imports, and modules
- Support for arithmetic operations
- Support for constant values
- Small string optimization utilities
- Opcode definitions and handling
- nom-based parser combinators
- Core library and binary executable

[0.2.0]: https://github.com/vrmiguel/water-old/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/vrmiguel/water-old/releases/tag/0.1.0
