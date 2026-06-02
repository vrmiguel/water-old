# Changelog

All notable changes to this project are documented in this file.

## 0.2.0 - 2026-06-02

### Added

- Added parsing support for function identifiers, exports, function imports, and module-level parsing.
- Added instruction parsing for `call`, `unreachable`, numerical constants, variable access, and nested/inlined instruction arguments.
- Added AST types for WebAssembly numerical types and values, parameters, locals, function imports, instructions, opcodes, indexes, constants, arithmetic operations, comparisons, variable operations, and `unreachable`.
- Added an initial WebAssembly emitter that writes the WASM magic header and version.
- Added emission support for constants, arithmetic operations, numerical values, and `unreachable`.
- Added signed and unsigned LEB128 encoding utilities with test coverage.
- Added WebAssembly opcode mappings for constants, arithmetic, comparisons, variable operations, `call`, and `unreachable`.
- Added a GitHub Actions workflow for building, formatting, linting, and testing the crate.
- Added Tembo project configuration and agent instructions.
- Added `SENTRY_ERRORS.md` to track Sentry issue investigation notes.
- Added a French README translation and linked it from the English README.

### Changed

- Reworked the parser into focused modules for functions, imports, instructions, modules, and shared parsing utilities.
- Expanded the README with a fuller project overview, features, project structure, build instructions, dependency notes, and project status.
- Bumped the crate version from `0.1.0` to `0.2.0`.
- Bumped the lockfile package version from `0.1.0` to `0.1.1`.

### Fixed

- Fixed doctests after parser and emitter refactors.
- Tightened Clippy configuration for stricter linting.
