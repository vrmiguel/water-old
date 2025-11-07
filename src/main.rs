use water::parser::{funktions_import_parsen, anweisung_parsen};

fn main() {
    dbg!(anweisung_parsen("i32.const 5").unwrap());

    dbg!(anweisung_parsen("(i32.const 5)").unwrap());

    dbg!(anweisung_parsen("(local.set $idx)").unwrap());
    dbg!(
        anweisung_parsen("(local.set $idx (i32.const 5))")
            .unwrap()
    );

    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;

    if let Err(err) = funktions_import_parsen(import_wat) {
        println!("{}", fehler_zu_string(import_wat, err));
    }

    fn fehler_zu_string(
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
