# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Getting Help

If you need help with `water`, here are some resources:

### Usage
To build and run the project:
```bash
cargo build
cargo run
```

### Documentation
- Check the source code in `src/` for implementation details
- The parser modules handle different WAT constructs:
  - `parser/instruction.rs` - WebAssembly instructions
  - `parser/import.rs` - Import declarations
  - `parser/module.rs` - Module structure

### Contributing
This project is built with Rust and uses the `nom` parser combinator library for parsing WebAssembly Text Format.

### Support
For questions or issues, please refer to the project documentation or examine the example usage in `main.rs`.

