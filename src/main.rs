use water::parser::{parse_function_import, parse_instruction};
use water::{calculate_padding, format_byte_size};

fn main() {
    dbg!(parse_instruction("i32.const 5").unwrap());

    dbg!(parse_instruction("(i32.const 5)").unwrap());

    dbg!(parse_instruction("(local.set $idx)").unwrap());
    dbg!(
        parse_instruction("(local.set $idx (i32.const 5))")
            .unwrap()
    );

    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;

    if let Err(err) = parse_function_import(import_wat) {
        println!("{}", stringify_error(import_wat, err));
    }

    // Demonstrate utility functions for WebAssembly binary formatting

    // Example: Calculate padding for 8-byte alignment (common in WASM memory)
    let offset = 13;
    let alignment = 8;
    let padding = calculate_padding(offset, alignment);
    println!(
        "Offset {} needs {} bytes of padding to align to {}-byte boundary (next aligned offset: {})",
        offset, padding, alignment, offset + padding
    );

    // Example: Format various WebAssembly module sizes
    let wasm_sizes = vec![
        ("Small module", 1024_u64),
        ("Medium module", 524288),
        ("Large module", 5242880),
        ("Very large module", 104857600),
    ];

    println!("\nWebAssembly module sizes:");
    for (name, size) in wasm_sizes {
        println!("  {}: {}", name, format_byte_size(size));
    }

    fn stringify_error(
        input: &str,
        error: nom::Err<nom::error::VerboseError<&str>>,
    ) -> String {
        match error {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(error)
            | nom::Err::Failure(error) => {
                nom::error::convert_error(input, error)
            }
        }
    }
}
