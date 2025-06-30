use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::context, multi::many0,
    sequence::preceded,
};

use super::IResult;
use crate::{
    ast::{Function, Local, Parameter},
    parser::utils::{
        parse_identifier, parse_parenthesis_enclosed,
        parse_string, parse_type,
    },
    small_string::SmallString,
};

/// Parses a function definition.
///
/// ```
/// use water::parser::parse_function;
/// use water::ast::{Function, Parameter, Local, Type, NumericalType};
///
/// let parameters = vec![
///     Parameter {
///         identifier: Some("number".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
/// ];
///
/// let local_variables = vec![
///     Local {
///         identifier: Some("l1".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
/// ];
///
/// let function = Function { identifier: Some("add".into()), parameters, local_variables, exports: vec![] };
///
/// assert_eq!(
///     parse_function("(func $add (param $number f64) (param i64) (local $l1 i32) (local f32))"),
///     Ok(("", function))
/// );
///
/// // Test for multiple parameters of the same type in a single param declaration
/// let multi_params = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
/// ];
///
/// let multi_locals = vec![
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
/// ];
///
/// let func_with_multi = Function {
///     identifier: None,
///     parameters: multi_params,
///     local_variables: multi_locals,
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func (param f32 f32) (param i32) (local i64 i64))"),
///     Ok(("", func_with_multi))
/// );
/// ```
pub fn parse_function(input: &str) -> IResult<Function> {
    fn inner(input: &str) -> IResult<Function> {
        let (rest, _) =
            preceded(multispace0, tag("func"))(input)?;

        let (rest, identifier) =
            preceded(multispace0, opt(parse_identifier))(rest)?;

        // TODO: WASM allows more than one `export` instructions
        // in a function, but they cannot have duplicated
        // names. Check for this either here or at a later step.
        let (rest, exports) = many0(parse_export)(rest)?;

        // Parse all parameter instructions and flatten them into a single vector
        let (rest, parameter_groups) =
            many0(parse_parameter)(rest)?;
        let parameters =
            parameter_groups.into_iter().flatten().collect();

        // Parse all local variable instructions and flatten them into a single vector
        let (rest, local_groups) = many0(parse_local)(rest)?;
        let local_variables =
            local_groups.into_iter().flatten().collect();

        let function = Function {
            identifier,
            parameters,
            local_variables,
            exports,
        };

        Ok((rest, function))
    }

    parse_parenthesis_enclosed(context("function", inner))(input)
}

/// Parses an `export` definition.
///
/// ```
/// use water::parser::parse_export;
/// use water::small_string::SmallString;
///
/// assert_eq!(parse_export(r#"(export "add")"#), Ok(("", "add".into())));
/// assert_eq!(parse_export(r#"(  export  "doSomethingUseful")"#), Ok(("", "doSomethingUseful".into())));
/// // WASM allows "" as a valid export name
/// assert_eq!(parse_export(r#"(export"")"#), Ok(("", "".into())));
///
/// // Wrong: missing name
/// assert!(parse_export(r#"(export)"#).is_err());
///
/// // Wrong: unclosed quoted string
/// assert!(parse_export(r#"(export ")"#).is_err());
///
/// // Wrong: missing terminating parenthesis
/// assert!(parse_export(r#"(export "valid""#).is_err());
///
/// // Wrong: missing first parenthesis
/// assert!(parse_export(r#"export "valid")"#).is_err());
///
/// // Wrong: missing both parenthesis
/// assert!(parse_export(r#"export "valid""#).is_err());
///
/// // Wrong: incorrect keyword
/// assert!(parse_export(r#"(expor "valid""))"#).is_err());
/// assert!(parse_export(r#"(exporT "valid""))"#).is_err());
///
/// // Wrong: extra string quote
/// assert!(parse_export(r#"(export "valid"")"#).is_err());
/// ```
pub fn parse_export(input: &str) -> IResult<SmallString> {
    fn inner(input: &str) -> IResult<SmallString> {
        let (rest, _) =
            preceded(multispace0, tag("export"))(input)?;

        let (rest, name) =
            preceded(multispace0, parse_string)(rest)?;

        Ok((rest, name.into()))
    }

    parse_parenthesis_enclosed(context("export", inner))(input)
}

/// Parses a function parameter.
///
/// Handles leading whitespace.
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parse_parameter;
///
/// let anonymous_i32 = Parameter {
///     identifier: None,
///     type_: Type::Numerical(NumericalType::Int32)
/// };
///
/// let named_f64 = Parameter {
///     identifier: Some("number".into()),
///     type_: Type::Numerical(NumericalType::Float64)
/// };
///
/// assert_eq!(parse_parameter("(param i32)"), Ok(("", vec![anonymous_i32])));
/// assert_eq!(parse_parameter("( param $number f64)"), Ok(("", vec![named_f64])));
/// assert_eq!(parse_parameter("(param f32 f32)"), Ok(("", vec![
///     Parameter { identifier: None, type_: Type::Numerical(NumericalType::Float32) },
///     Parameter { identifier: None, type_: Type::Numerical(NumericalType::Float32) }
/// ])));
/// ```
pub fn parse_parameter(input: &str) -> IResult<Vec<Parameter>> {
    fn inner(input: &str) -> IResult<Vec<Parameter>> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;

        // Try to parse an identifier (optional)
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;

        // Parse the first type (required)
        let (mut rest, first_type) =
            preceded(multispace0, parse_type)(rest)?;

        // Check if we have an identifier
        let has_identifier = identifier.is_some();

        // Create the first parameter
        let mut parameters = vec![Parameter {
            identifier,
            type_: first_type,
        }];

        // If we have an identifier, we can only have one type
        if has_identifier {
            return Ok((rest, parameters));
        }

        // Otherwise, try to parse additional types (for case like "param f32 f32")
        loop {
            // Try to parse another type with whitespace before it
            match preceded(multispace0, parse_type)(rest) {
                Ok((new_rest, next_type)) => {
                    // Add a new parameter with no identifier and this type
                    parameters.push(Parameter {
                        identifier: None,
                        type_: next_type,
                    });
                    rest = new_rest;
                }
                Err(_) => {
                    // No more types to parse
                    break;
                }
            }
        }

        Ok((rest, parameters))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("parameter", inner)),
    )(input)
}

/// Parses a local variable definition.
///
/// ```
/// use water::ast::{Local, Type, NumericalType};
/// use water::parser::parse_local;
/// use water::small_string::SmallString;
///
/// let anonymous_f32 = Local {
///     identifier: None,
///     type_: Type::Numerical(NumericalType::Float32)
/// };
///
/// let named_i64 = Local {
///     identifier: Some("number".into()),
///     type_: Type::Numerical(NumericalType::Int64)
/// };
///
/// assert_eq!(parse_local("(local f32)"), Ok(("", vec![anonymous_f32])));
/// assert_eq!(parse_local("( local $number i64)"), Ok(("", vec![named_i64])));
/// assert_eq!(parse_local("(local i32 i32)"), Ok(("", vec![
///     Local { identifier: None, type_: Type::Numerical(NumericalType::Int32) },
///     Local { identifier: None, type_: Type::Numerical(NumericalType::Int32) }
/// ])));
/// ```
pub fn parse_local(input: &str) -> IResult<Vec<Local>> {
    fn inner(input: &str) -> IResult<Vec<Local>> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;

        // Try to parse an identifier (optional)
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;

        // Parse the first type (required)
        let (mut rest, first_type) =
            preceded(multispace0, parse_type)(rest)?;

        // Check if we have an identifier
        let has_identifier = identifier.is_some();

        // Create the first local variable
        let mut locals = vec![Local {
            identifier,
            type_: first_type,
        }];

        // If we have an identifier, we can only have one type
        if has_identifier {
            return Ok((rest, locals));
        }

        // Otherwise, try to parse additional types (for case like "local i32 i32")
        loop {
            // Try to parse another type with whitespace before it
            match preceded(multispace0, parse_type)(rest) {
                Ok((new_rest, next_type)) => {
                    // Add a new local with no identifier and this type
                    locals.push(Local {
                        identifier: None,
                        type_: next_type,
                    });
                    rest = new_rest;
                }
                Err(_) => {
                    // No more types to parse
                    break;
                }
            }
        }

        Ok((rest, locals))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("local", inner)),
    )(input)
}
