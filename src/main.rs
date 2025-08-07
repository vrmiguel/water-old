use water::parser::{parse_function_import, parse_instruction};

fn stringify_error(
    input: &str,
    error: nom::Err<nom::error::VerboseError<&str>>,
) -> String {
    match error {
        nom::Err::Incomplete(_) => unreachable!(),
        nom::Err::Error(error) | nom::Err::Failure(error) => {
            nom::error::convert_error(input, error)
        }
    }
}

fn main() {
    // Parse simple constant instruction
    match parse_instruction("i32.const 5") {
        Ok((_, instruction)) => println!("Parsed: {:?}", instruction),
        Err(err) => eprintln!("Failed to parse: {}", stringify_error("i32.const 5", err)),
    }

    // Parse parenthesized constant instruction
    match parse_instruction("(i32.const 5)") {
        Ok((_, instruction)) => println!("Parsed: {:?}", instruction),
        Err(err) => eprintln!("Failed to parse: {}", stringify_error("(i32.const 5)", err)),
    }

    // Parse local.set instruction
    match parse_instruction("(local.set $idx)") {
        Ok((_, instruction)) => println!("Parsed: {:?}", instruction),
        Err(err) => eprintln!("Failed to parse: {}", stringify_error("(local.set $idx)", err)),
    }

    // Parse complex local.set instruction
    let complex_instr = "(local.set $idx (i32.const 5))";
    match parse_instruction(complex_instr) {
        Ok((_, instruction)) => println!("Parsed: {:?}", instruction),
        Err(err) => eprintln!("Failed to parse: {}", stringify_error(complex_instr, err)),
    }

    // Parse function import
    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;
    match parse_function_import(import_wat) {
        Ok((_, import)) => println!("Parsed import: {:?}", import),
        Err(err) => eprintln!("Failed to parse import: {}", stringify_error(import_wat, err)),
    }
}
