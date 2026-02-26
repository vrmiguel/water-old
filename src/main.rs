use water::parser::{
    parse_function_import, parse_instruction, stringify_error,
};

fn main() {
    for input in [
        "i32.const 5",
        "(i32.const 5)",
        "(local.set $idx)",
        "(local.set $idx (i32.const 5))",
    ] {
        match parse_instruction(input) {
            Ok((rest, instr)) if rest.trim().is_empty() => {
                dbg!(instr);
            }
            Ok((rest, instr)) => {
                eprintln!(
                    "warning: unconsumed input after parse: {rest:?}"
                );
                dbg!(instr);
            }
            Err(err) => {
                eprintln!("{}", stringify_error(input, err));
            }
        }
    }

    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;

    if let Err(err) = parse_function_import(import_wat) {
        println!("{}", stringify_error(import_wat, err));
    }
}
