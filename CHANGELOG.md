# Changelog

All notable changes to this project are documented in this file.

## 0.2.0 - 2026-06-02

### Added

- Added parsing support for function identifiers, exports, function imports, module-level parsing, `call`, `unreachable`, numeric constants, variable access, and nested instruction arguments.
- Added AST types for WebAssembly values, numeric types, parameters, locals, function imports, indexes, constants, instructions, arithmetic operations, comparison operations, variable operations, and `unreachable`.
- Added WebAssembly emission infrastructure, including the `Emittable` trait, WASM magic/version bytes, numeric constants, arithmetic operations, numerical values, and `unreachable`.
- Added signed and unsigned LEB128 encoding utilities with test coverage.
- Added opcode mappings for constants, arithmetic operations, comparison operations, variable operations, `call`, and `unreachable`.
- Added CI coverage for formatting, linting, building, and testing.
- Added Tembo configuration and contributor/agent instructions.
- Added `SENTRY_ERRORS.md` for Sentry issue investigation notes.
- Added a French README translation and linked it from the English README.

### Changed

- Renamed the crate from `wasm` to `water`.
- Reworked parser internals into focused modules for functions, imports, instructions, modules, and shared utilities.
- Split arithmetic, comparison, and variable access instructions into more specific operation types.
- Expanded the README with a fuller project overview, feature summary, setup instructions, dependency notes, and project status.
- Updated the crate version from `0.1.0` to `0.2.0`.

### Fixed

- Fixed doctests after parser and emitter refactors.
- Tightened Clippy configuration for stricter linting.
