# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Current status

The project already includes a parser for several WAT constructs and the
beginning of a binary emitter. Right now the crate is best viewed as an
early-stage compiler toolkit rather than a finished command-line compiler.

## What is in the crate

- `water::parser` exposes parsers for instructions, imports, identifiers, and
  other WAT syntax pieces.
- `water::emitter` contains the binary emission building blocks used to write
  WebAssembly bytes.
- `water::ast` defines the syntax tree shared by the parser and emitter.

## Quick example

```rust
use water::parser::parse_instruction;

fn main() {
    let instruction = parse_instruction("i32.const 5").unwrap();
    dbg!(instruction);
}
```

## Development

The current `src/main.rs` file acts as a small playground for parser
experiments while the public library lives under `src/lib.rs`.
