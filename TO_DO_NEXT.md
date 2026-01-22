# TO_DO_NEXT

Possible improvements for the `water` WebAssembly Text Format compiler.

## Documentation

- Expand README.md with usage examples, installation instructions, and feature overview
- Add inline documentation to public APIs
- Create a CONTRIBUTING.md guide for contributors

## Features

- Add support for additional WebAssembly instructions beyond current arithmetic operations
- Implement memory operations (load/store instructions)
- Add support for control flow instructions (if/else, loop, br, br_if)
- Implement table operations
- Add support for global variables
- Implement multi-value returns
- Add support for WebAssembly 2.0 features (reference types, bulk memory operations)

## Code Quality

- Add comprehensive unit tests for parser and emitter modules
- Add integration tests with real WAT files
- Implement property-based testing for the parser
- Add benchmarks to track performance

## Error Handling

- Improve error messages with line/column information
- Add error recovery in the parser for better diagnostics
- Implement a proper error type with rich context

## Tooling

- Add a WASM validator to verify output correctness
- Implement a pretty-printer for the AST
- Add source maps for debugging support
- Create a REPL for interactive WAT compilation

## Performance

- Profile and optimize hot paths in the parser
- Consider using arena allocation for AST nodes
- Implement incremental compilation for large files
