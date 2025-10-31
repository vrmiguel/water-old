# water (WebAssembly TExt foRmat compiler) - Zig Edition

This is a complete Zig translation of the original Rust `water` project, a tiny and performant WebAssembly Text Format compiler.

## Project Structure

All Rust source files have been converted to Zig:

- `src/lib.zig` - Main library entry point (was `src/lib.rs`)
- `src/main.zig` - Main executable (was `src/main.rs`)
- `src/ast.zig` - Abstract Syntax Tree definitions (was `src/ast.rs`)
- `src/parser.zig` - Parser implementation (was `src/parser.rs` and `src/parser/` directory)
- `src/emitter.zig` - Emitter for WASM bytecode (was `src/emitter.rs` and `src/emitter/` directory)
- `src/leb128.zig` - LEB128 encoding implementation (was `src/leb128.rs`)
- `src/opcode.zig` - Opcode mappings (was `src/opcode.rs`)
- `src/small_string.zig` - Optimized string type (was `src/small_string.rs`)

## Building

To build the project:

```bash
zig build
```

To run tests:

```bash
zig build test
```

To run the executable:

```bash
zig build run
```

## Key Differences from Rust Version

1. **Memory Management**: Zig requires explicit allocator passing, unlike Rust's implicit allocator
2. **Error Handling**: Zig uses `!` for error unions instead of Rust's `Result<T, E>`
3. **Parser**: The Rust version used the `nom` parsing library. The Zig version implements a simplified hand-written parser
4. **String Handling**: Adapted the `SmallString` optimization to use Zig's memory model
5. **LEB128 Encoding**: Converted from trait-based to direct method calls
6. **Testing**: Zig uses built-in `test` blocks instead of `#[test]` attributes

## Dependencies

The Rust version depended on:
- `nom` (7.1.1) - parsing library

The Zig version has **zero external dependencies**, using only the Zig standard library.

## License

Same as the original project (see LICENSE file).
