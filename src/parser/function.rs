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
/// // Test multi-parameter parsing
/// let multi_param_function = Function {
///     identifier: Some("multi_param".into()),
///     parameters: vec![
///         Parameter {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Float32)
///         },
///         Parameter {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Float32)
///         },
///         Parameter {
///             identifier: Some("named".into()),
///             type_: Type::Numerical(NumericalType::Int32)
///         },
///     ],
///     local_variables: vec![],
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func $multi_param (param f32 f32) (param $named i32))"),
///     Ok(("", multi_param_function))
/// );
/// ```
/// Parse a multi-parameter declaration like `(param f32 f32
/// f32)`. This function handles the case where multiple types
/// are specified in a single param instruction. It parses each
/// type and creates a parameter for each, with all parameters
/// having no identifier.
fn parse_multi_parameter(
    input: &str,
) -> IResult<Vec<Parameter>> {
    fn inner(input: &str) -> IResult<Vec<Parameter>> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;

        // Parse multiple types
        let current_input = rest;
        let mut parameters = Vec::new();

        // Check for an identifier
        let (current_input, maybe_id) =
            opt(preceded(multispace0, parse_identifier))(
                current_input,
            )?;

        // Parse the first type
        let (mut current_input, first_type) =
            preceded(multispace0, parse_type)(current_input)?;

        parameters.push(Parameter {
            identifier: maybe_id,
            type_: first_type,
        });

        // Try to parse additional types (which will all be
        // anonymous parameters)
        let mut had_additional_types = false;

        while let Ok((new_input, type_)) =
            preceded(multispace0, parse_type)(current_input)
        {
            parameters.push(Parameter {
                identifier: None,
                type_,
            });
            current_input = new_input;
            had_additional_types = true;
        }

        // If we didn't find any additional types, this isn't a
        // multi-parameter declaration and should be
        // handled by the regular parameter parser
        if !had_additional_types {
            return Err(nom::Err::Error(
                nom::error::make_error(
                    input,
                    nom::error::ErrorKind::Tag,
                ),
            ));
        }

        Ok((current_input, parameters))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context(
            "multi-parameter",
            inner,
        )),
    )(input)
}

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

        // Try to parse parameters - first try multi-parameters,
        // then fall back to single parameters
        let mut parameters = Vec::with_capacity(2);
        let mut current_input = rest;

        loop {
            // Try multi-parameter first (it handles the case
            // with multiple types)
            if let Ok((new_input, mut multi_params)) =
                parse_multi_parameter(current_input)
            {
                parameters.append(&mut multi_params);
                current_input = new_input;
            }
            // If not a multi-parameter, try a regular parameter
            else if let Ok((new_input, param)) =
                parse_parameter(current_input)
            {
                parameters.push(param);
                current_input = new_input;
            }
            // If neither works, we're done parsing parameters
            else {
                break;
            }
        }

        let (rest, local_variables) =
            many0(parse_local)(current_input)?;

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
/// // Test single parameter cases
/// assert_eq!(parse_parameter("(param i32)"), Ok(("", anonymous_i32)));
/// assert_eq!(parse_parameter("( param $number f64)"), Ok(("", named_f64)));
/// ```
pub fn parse_parameter(input: &str) -> IResult<Parameter> {
    fn inner(input: &str) -> IResult<Parameter> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;
        let (rest, type_) =
            preceded(multispace0, parse_type)(rest)?;

        let parameter = Parameter { identifier, type_ };

        Ok((rest, parameter))
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
/// assert_eq!(parse_local("(local f32)"), Ok(("", anonymous_f32)));
/// assert_eq!(parse_local("( local $number i64)"), Ok(("", named_i64)));
/// ```
pub fn parse_local(input: &str) -> IResult<Local> {
    fn inner(input: &str) -> IResult<Local> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;
        let (rest, type_) =
            preceded(multispace0, parse_type)(rest)?;

        let local = Local { identifier, type_ };

        Ok((rest, local))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("local", inner)),
    )(input)
}
