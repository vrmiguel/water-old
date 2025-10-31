# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Zig Port

This repository has been ported from Rust to Zig. The core functionality including:
- AST (Abstract Syntax Tree) definitions
- Opcode mappings
- Emitter for WebAssembly binary format
- LEB128 encoding
- Small string optimization

have all been successfully ported to Zig.

**Note on Parser**: The parser is currently a stub implementation. The original Rust version used the `nom` parser combinator library. To fully port the parser, you would need to either:
1. Use or create a Zig parser combinator library
2. Write a custom recursive descent parser
3. Use an existing Zig parsing library

## Building

### Requirements
- Zig 0.11.0 or later

### Build Commands

```bash
# Build the project
zig build

# Run the executable
zig build run

# Run tests
zig build test
```

### Project Structure

```
src/
├── root.zig          - Library root, exports all public modules
├── main.zig          - Executable entry point
├── ast.zig           - Abstract Syntax Tree definitions
├── emitter.zig       - WebAssembly binary emitter
├── opcode.zig        - Opcode mappings
├── leb128.zig        - LEB128 encoding
├── small_string.zig  - Optimized string type
└── parser.zig        - Parser (stub implementation)
```

## Original Rust Version

The original Rust implementation included a full parser using the `nom` library. If you need the full parsing functionality, please refer to the Rust source files or implement a complete parser in Zig.

