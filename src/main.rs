use water::parser::{parse_function_import, parse_instruction};

fn stringify_error(
    input: &str,
    error: nom::Err<nom::error::VerboseError<&str>>,
) -> String {
    match error {
        nom::Err::Incomplete(_) => {
            "Incomplete input".to_string()
        }
        nom::Err::Error(error) | nom::Err::Failure(error) => {
            nom::error::convert_error(input, error)
        }
    }
}

fn main() {
    parse_instruction("i32.const 5").unwrap();
    parse_instruction("(i32.const 5)").unwrap();
    parse_instruction("(local.set $idx)").unwrap();
    parse_instruction("(local.set $idx (i32.const 5))")
        .unwrap();

    let import_wat = r#"(import "console" "log"
        (func $log (param i32) (param i32)))"#;

    if let Err(err) = parse_function_import(import_wat) {
        println!("{}", stringify_error(import_wat, err));
    }
}
