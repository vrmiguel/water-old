use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::{context, ParseError}, multi::many0,
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
/// // Test parsing function with multiple parameters in a single param declaration
/// let multi_param_test = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("x".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     },
/// ];
///
/// let function_multi_params = Function { 
///     identifier: None, 
///     parameters: multi_param_test, 
///     local_variables: vec![], 
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func (param f32 f32) (param $x i32))"),
///     Ok(("", function_multi_params))
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
        
        // Parse parameters - need to handle both single and multi-param declarations
        let (rest, param_groups) = many0(parse_multiple_parameters)(rest)?;
        // Flatten the nested vectors of parameters
        let parameters = param_groups.into_iter().flatten().collect();
        
        let (rest, local_variables) = many0(parse_local)(rest)?;

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
/// This function parses a single parameter from a parameter declaration.
/// For handling multiple parameters in a single declaration like `(param f32 f32)`,
/// use `parse_multiple_parameters` instead.
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
/// let anonymous_f32 = Parameter {
///     identifier: None,
///     type_: Type::Numerical(NumericalType::Float32)
/// };
///
/// assert_eq!(parse_parameter("(param i32)"), Ok(("", anonymous_i32)));
/// assert_eq!(parse_parameter("( param $number f64)"), Ok(("", named_f64)));
/// 
/// // When given a parameter with multiple types, only the first parameter is parsed
/// // and the rest are ignored (they should be handled with parse_multiple_parameters)
/// assert_eq!(parse_parameter("(param f32 f32)"), Ok(("", anonymous_f32)));
/// ```
pub fn parse_parameter(input: &str) -> IResult<Parameter> {
    // Delegate to parse_multiple_parameters and take the first one
    let (rest, mut parameters) = parse_multiple_parameters(input)?;
    if parameters.is_empty() {
        // This should not happen since parse_multiple_parameters ensures at least one parameter
        return Err(nom::Err::Error(
            nom::error::VerboseError::from_error_kind(input, nom::error::ErrorKind::Verify)
        ));
    }
    
    Ok((rest, parameters.remove(0)))
}

/// Parses a param declaration that may contain multiple parameters of the same type.
///
/// This function handles all cases of parameter declarations:
/// - A single parameter with a type: `(param i32)`
/// - A named parameter with a type: `(param $number f64)`
/// - Multiple parameters of the same type: `(param f32 f32)`
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parse_multiple_parameters;
///
/// // Single anonymous parameter
/// let result = parse_multiple_parameters("(param i32)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 1);
/// assert_eq!(result.1[0].identifier, None);
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Int32));
///
/// // Single named parameter
/// let result = parse_multiple_parameters("(param $name f64)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 1);
/// assert_eq!(result.1[0].identifier, Some("name".into()));
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Float64));
///
/// // Multiple anonymous parameters
/// let result = parse_multiple_parameters("(param f32 f32 f32)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 3);
/// for param in &result.1 {
///     assert_eq!(param.identifier, None);
///     assert_eq!(param.type_, Type::Numerical(NumericalType::Float32));
/// }
/// ```
pub fn parse_multiple_parameters(input: &str) -> IResult<Vec<Parameter>> {
    fn inner(input: &str) -> IResult<Vec<Parameter>> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        
        // Look for an optional identifier first
        let (rest, identifier) = 
            opt(preceded(multispace0, parse_identifier))(rest)?;
        
        // Parse the first type, which is required
        let (mut rest, type_) = 
            preceded(multispace0, parse_type)(rest)?;
        
        // Check if we have an identifier before using it
        let has_identifier = identifier.is_some();
        
        // Create the first parameter
        let mut parameters = vec![Parameter { identifier, type_: type_.clone() }];
        
        // If there's an identifier, we can only have one parameter in this declaration
        if !has_identifier {
            // Look for additional types (all without identifiers)
            while let Ok((new_rest, additional_type)) = preceded(multispace0, parse_type)(rest) {
                parameters.push(Parameter {
                    identifier: None,
                    type_: additional_type.clone(),
                });
                rest = new_rest;
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
