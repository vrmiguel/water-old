# water (WebAssembly TExt foRmat compiler)

`water` is a tiny and performant WebAssembly Text Format compiler. It parses WebAssembly Text Format (WAT) code and compiles it to WebAssembly binary format (WASM).

## Features

- Parse WebAssembly Text Format (WAT) syntax
- Support for WebAssembly instructions like `i32.const`, `local.set`, etc.
- Function import parsing
- Efficient binary encoding with LEB128

## Prerequisites

- Rust toolchain (1.56.0 or later recommended)
- Cargo (Rust's package manager)

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/your-username/water.git
cd water
cargo build --release
```

The compiled binary will be available at `target/release/water`.

## Usage

Currently, the project is in development and doesn't have a command-line interface. You can use the library in your Rust projects by adding it as a dependency:

```toml
[dependencies]
water = { git = "https://github.com/your-username/water.git" }
```

Example usage in your code:

```rust
use water::parser::parse_instruction;

fn main() {
    let result = parse_instruction("i32.const 42").unwrap();
    println!("{:?}", result);
}
```

## Development

Build the project:

```bash
cargo build
```

Run the default example:

```bash
cargo run
```

## License

[Insert your license information here]
