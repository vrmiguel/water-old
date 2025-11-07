use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, sequence::preceded,
};

use super::IResult;
use crate::{
    ast::FunctionImport,
    parser::{
        funktion_parsen, in_klammern_eingeschlossen_parsen, zeichenkette_parsen,
    },
};

/// Parses a function import.
///
/// ```
/// use water::ast::{FunctionImport, Function, Parameter, Type, NumericalType};
/// use water::parser::funktions_import_parsen;
///
/// let import_wat = r#"(import "console" "log" (func $log (param f32) (param f32)))"#;
/// let parsed_import = FunctionImport {
///     namespace: "console".into(),
///     fn_name: "log".into(),
///     signature: Function {
///         identifier: Some("log".into()),
///         parameters: vec![Parameter { identifier: None, type_: Type::Numerical(NumericalType::Float32)}; 2],
///         exports: vec![],
///         local_variables: vec![],
///     }
/// };
///
/// assert_eq!(funktions_import_parsen(import_wat), Ok(("", parsed_import)));
/// ```
pub fn funktions_import_parsen(
    input: &str,
) -> IResult<FunctionImport> {
    fn innere(input: &str) -> IResult<FunctionImport> {
        let (rest, _) =
            preceded(multispace0, tag("import"))(input)?;
        let (rest, namespace) =
            preceded(multispace0, zeichenkette_parsen)(rest)?;
        let (rest, fn_name) =
            preceded(multispace0, zeichenkette_parsen)(rest)?;
        let (rest, function) =
            preceded(multispace0, funktion_parsen)(rest)?;

        // TODO: transform into nom errors
        assert!(function.exports.is_empty());
        assert!(function.local_variables.is_empty());

        let fn_import = FunctionImport {
            namespace: namespace.into(),
            fn_name: fn_name.into(),
            signature: function,
        };

        Ok((rest, fn_import))
    }

    in_klammern_eingeschlossen_parsen(context("function import", innere))(
        input,
    )
}
