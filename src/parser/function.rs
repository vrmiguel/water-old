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
/// Supports both individual parameter declarations like `(param i32)` and multiple
/// parameter declarations of the same type like `(param f32 f32)`.
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
/// // Example with multiple parameters of the same type in a single declaration
/// let multi_params = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
/// ];
///
/// let function_with_multi_params = Function {
///     identifier: Some("mul".into()),
///     parameters: multi_params,
///     local_variables: vec![],
///     exports: vec![],
/// };
///
/// assert_eq!(
///     parse_function("(func $mul (param f32 f32))"),
///     Ok(("", function_with_multi_params))
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
        
        // Parse all parameter declarations (both single and multiple parameters)
        let mut parameters = Vec::new();
        let mut current_rest = rest;
        
        // Try to parse both single parameters and multiple parameters
        loop {
            let result_single = parse_parameter(current_rest);
            let result_multiple = parse_multiple_parameters(current_rest);
            
            match (result_single, result_multiple) {
                (Ok((new_rest, param)), _) => {
                    // Single parameter was successfully parsed
                    parameters.push(param);
                    current_rest = new_rest;
                }
                (_, Ok((new_rest, params))) => {
                    // Multiple parameters were successfully parsed
                    parameters.extend(params);
                    current_rest = new_rest;
                }
                _ => {
                    // Neither single nor multiple parameters could be parsed, we're done
                    break;
                }
            }
        }
        
        let (rest, local_variables) = many0(parse_local)(current_rest)?;

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

/// Parses multiple function parameters in a single param declaration.
/// 
/// Handles cases such as `(param f32 f32)` which defines multiple parameters of the same type.
/// 
/// ```
/// use water::parser::parse_multiple_parameters;
/// use water::ast::{Parameter, Type, NumericalType};
/// 
/// let parameters = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
/// ];
/// 
/// assert_eq!(parse_multiple_parameters("(param f32 f32)"), Ok(("", parameters)));
/// ```
pub fn parse_multiple_parameters(input: &str) -> IResult<Vec<Parameter>> {
    fn inner(input: &str) -> IResult<Vec<Parameter>> {
        let (rest, _) = preceded(multispace0, tag("param"))(input)?;
        
        // Check if the next token is an identifier
        let (rest, maybe_identifier) = opt(preceded(multispace0, parse_identifier))(rest)?;
        
        // If we have an identifier, this is a named parameter case
        if let Some(identifier) = maybe_identifier {
            let (rest, type_) = preceded(multispace0, parse_type)(rest)?;
            let parameter = Parameter { identifier: Some(identifier), type_ };
            return Ok((rest, vec![parameter]));
        }
        
        // If there's no identifier, we expect one or more types
        let mut parameters = Vec::new();
        let mut current_rest = rest;
        
        // Parse all types until we can't parse any more
        loop {
            match preceded(multispace0, parse_type)(current_rest) {
                Ok((new_rest, type_)) => {
                    parameters.push(Parameter { identifier: None, type_ });
                    current_rest = new_rest;
                }
                Err(_) => {
                    // No more types to parse
                    break;
                }
            }
        }
        
        // We should have at least one parameter
        if parameters.is_empty() {
            return Err(nom::Err::Error(
                nom::error::make_error(input, nom::error::ErrorKind::Tag)
            ));
        }
        
        Ok((current_rest, parameters))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("multiple parameters", inner)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{NumericalType, Type};

    #[test]
    fn test_parse_multiple_parameters() {
        // Test single type
        let single_param = "(param f32)";
        let (rest, parameters) = parse_multiple_parameters(single_param).unwrap();
        assert_eq!(rest, "");
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0].identifier, None);
        assert_eq!(parameters[0].type_, Type::Numerical(NumericalType::Float32));

        // Test multiple types
        let multi_param = "(param f32 f32)";
        let (rest, parameters) = parse_multiple_parameters(multi_param).unwrap();
        assert_eq!(rest, "");
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].identifier, None);
        assert_eq!(parameters[0].type_, Type::Numerical(NumericalType::Float32));
        assert_eq!(parameters[1].identifier, None);
        assert_eq!(parameters[1].type_, Type::Numerical(NumericalType::Float32));

        // Test multiple types of different types
        let mixed_types = "(param i32 f64)";
        let (rest, parameters) = parse_multiple_parameters(mixed_types).unwrap();
        assert_eq!(rest, "");
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].identifier, None);
        assert_eq!(parameters[0].type_, Type::Numerical(NumericalType::Int32));
        assert_eq!(parameters[1].identifier, None);
        assert_eq!(parameters[1].type_, Type::Numerical(NumericalType::Float64));

        // Test with whitespace
        let with_whitespace = "(param  i32   f64 )";
        let (rest, parameters) = parse_multiple_parameters(with_whitespace).unwrap();
        assert_eq!(rest, "");
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].identifier, None);
        assert_eq!(parameters[0].type_, Type::Numerical(NumericalType::Int32));
        assert_eq!(parameters[1].identifier, None);
        assert_eq!(parameters[1].type_, Type::Numerical(NumericalType::Float64));
    }

    #[test]
    fn test_parse_function_with_multiple_parameters() {
        // Test a function with mixed parameter declarations
        let func_text = "(func $test (param $x f32) (param i32 i64))";
        let (rest, function) = parse_function(func_text).unwrap();
        
        assert_eq!(rest, "");
        assert_eq!(function.identifier, Some("test".into()));
        assert_eq!(function.parameters.len(), 3);
        
        // First parameter: named f32
        assert_eq!(function.parameters[0].identifier, Some("x".into()));
        assert_eq!(function.parameters[0].type_, Type::Numerical(NumericalType::Float32));
        
        // Second parameter: anonymous i32
        assert_eq!(function.parameters[1].identifier, None);
        assert_eq!(function.parameters[1].type_, Type::Numerical(NumericalType::Int32));
        
        // Third parameter: anonymous i64
        assert_eq!(function.parameters[2].identifier, None);
        assert_eq!(function.parameters[2].type_, Type::Numerical(NumericalType::Int64));
    }
}
