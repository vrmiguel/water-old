use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, sequence::preceded,
};

use super::IResult;
use crate::{
    ast::FunctionImport,
    parser::{
        parse_function, parse_parenthesis_enclosed, parse_string,
    },
};

/// Parses a WebAssembly function import declaration from WAT text format.
/// 
/// This function handles the parsing of import statements that bring external
/// functions into the WebAssembly module. It's essential for modules that
/// need to call host functions or functions from other modules.
/// 
/// # Arguments
/// * `input` - The input string containing the WAT function import declaration
/// 
/// # Returns
/// * `IResult<FunctionImport>` - The parsed function import structure or an error
/// 
/// # WAT Format
/// ```text
/// (import "namespace" "function_name" (func $local_name (param types...)))
/// ```
///
/// # Example
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

    parse_parenthesis_enclosed(context("function import", inner))(
        input,
    )
}
