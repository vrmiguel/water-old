# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format (WAT) compiler.

## Status

This project is currently **work-in-progress**:

- The parser is implemented for a growing subset of WAT (see `src/parser/`).
- The emitter exists but is not yet feature-complete (it currently only writes the WASM header).
- The `src/main.rs` binary is a small scratchpad used for local debugging.

## Usage (library)

Today, `water` is primarily a Rust library you can use to parse pieces of WAT.

```rust
use water::parser::parse_instruction;

let (_, instr) = parse_instruction("(i32.const 5)")?;
println!("{instr:?}");
# Ok::<(), nom::Err<nom::error::VerboseError<&'static str>>>(())
```

More parsing helpers are re-exported from `water::parser` (for example `parse_function_import`).

## Goals

- Small, straightforward implementation with good error messages.
- A “compile WAT → WASM” pipeline that can be embedded as a library.

## Non-goals (for now)

- Supporting every corner of the WAT spec immediately.
- Providing a polished command-line interface (CLI) before the core is stable.

## License

MIT (see `LICENSE`).
