use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::{context, VerboseError}, sequence::preceded,
    Err, error::ParseError,
};

use super::IResult;
use crate::{
    ast::FunctionImport,
    parser::{
        parse_function, parse_parenthesis_enclosed, parse_string,
    },
};

/// Parses a function import.
///
/// ```
/// use water::ast::{FunctionImport, Function, Parameter, Type, NumericalType};
/// use water::parser::parse_function_import;
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
/// assert_eq!(parse_function_import(import_wat), Ok(("", parsed_import)));
/// ```
pub fn parse_function_import(
    input: &str,
) -> IResult<FunctionImport> {
    fn inner(input: &str) -> IResult<FunctionImport> {
        let (rest, _) =
            preceded(multispace0, tag("import"))(input)?;
        let (rest, namespace) =
            preceded(multispace0, parse_string)(rest)?;
        let (rest, fn_name) =
            preceded(multispace0, parse_string)(rest)?;
        let (rest, function) =
            preceded(multispace0, parse_function)(rest)?;

        // Create a validation error with context
        fn validation_error<'a>(input: &'a str, message: &'static str) -> nom::Err<VerboseError<&'a str>> {
            let mut err = VerboseError { errors: vec![] };
            err.errors.push((input, nom::error::ErrorKind::Verify));
            Err::Error(ParseError::add_context(input, message, err))
        }
        
        // Validate that the function doesn't have exports or local variables
        if !function.exports.is_empty() {
            return validation_error(rest, "imported functions cannot have exports");
        }
        
        if !function.local_variables.is_empty() {
            return validation_error(rest, "imported functions cannot have local variables");
        }

        let fn_import = FunctionImport {
            namespace: namespace.into(),
            fn_name: fn_name.into(),
            signature: function,
        };

        Ok((rest, fn_import))
    }

    parse_parenthesis_enclosed(context("function import", inner))(
        input,
    )
}
