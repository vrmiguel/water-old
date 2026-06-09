# water - WebAssembly Text Format Compiler

`water` is a small, dependency-light WebAssembly Text Format (WAT) parser and binary emission toolkit written in Python.

The project currently supports the same experimental subset as the original implementation:

- WAT identifiers, strings, indexes, numeric types, and numeric constants
- `local.get`, `local.set`, `local.tee`, `global.get`, and `global.set`
- `call` and `unreachable`
- nested parenthesized instructions
- function signatures, exports, locals, function imports, and empty modules
- LEB128 integer encoding and basic WebAssembly opcode emission helpers

## Installation

```bash
python -m pip install -e ".[dev]"
```

## Running

```bash
water "i32.const 5"
python -m water "(local.set $idx (i32.const 5))"
```

With no arguments, the CLI parses a few sample WAT snippets.

## Testing

```bash
pytest
```

## Project Structure

- `water/ast.py` - dataclass-based AST definitions
- `water/parser.py` - hand-written WAT parser for the supported subset
- `water/emitter.py` - binary emission helpers
- `water/leb128.py` - signed and unsigned LEB128 encoders
- `water/opcode.py` - WebAssembly opcode mappings

## Project Status

This is an experimental and educational project focused on understanding WebAssembly Text Format parsing and compilation.
