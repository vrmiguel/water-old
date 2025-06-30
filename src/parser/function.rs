use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::{context, ParseError, VerboseError}, multi::many0,
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
/// // Example with multiple parameters in a single param instruction
/// let multi_param_function = Function { 
///     identifier: Some("multiParam".into()), 
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
///         }
///     ], 
///     local_variables: vec![], 
///     exports: vec![] 
/// };
///
/// assert_eq!(
///     parse_function("(func $multiParam (param f32 f32) (param $named i32))"),
///     Ok(("", multi_param_function))
/// );
///
/// // Example with multiple parameters and multiple locals
/// let multi_param_and_local_function = Function { 
///     identifier: Some("complexFunc".into()), 
///     parameters: vec![
///         Parameter {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Float32)
///         },
///         Parameter {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Float32)
///         }
///     ], 
///     local_variables: vec![
///         Local {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Int32)
///         },
///         Local {
///             identifier: None,
///             type_: Type::Numerical(NumericalType::Int32)
///         }
///     ], 
///     exports: vec![] 
/// };
///
/// assert_eq!(
///     parse_function("(func $complexFunc (param f32 f32) (local i32 i32))"),
///     Ok(("", multi_param_and_local_function))
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
        
        // Parse parameters - both single parameter and multiple parameters forms
        let (rest, parameters) = {
            let mut params = Vec::new();
            let mut current_rest = rest;
            
            // Keep parsing until we can't find any more parameters
            loop {
                // First try parsing multiple parameters (param f32 f32)
                if let Ok((new_rest, mut multiple_params)) = parse_multiple_parameters(current_rest) {
                    params.append(&mut multiple_params);
                    current_rest = new_rest;
                    continue;
                }
                
                // Then try parsing a single parameter
                if let Ok((new_rest, param)) = parse_parameter(current_rest) {
                    params.push(param);
                    current_rest = new_rest;
                    continue;
                }
                
                // If we can't parse any more parameters, break
                break;
            }
            
            (current_rest, params)
        };
        
        let (rest, local_variables) = {
            let mut locals = Vec::new();
            let mut current_rest = rest;
            
            // Keep parsing until we can't find any more locals
            loop {
                // First try parsing multiple locals (local f32 f32)
                if let Ok((new_rest, mut multiple_locals)) = parse_multiple_locals(current_rest) {
                    locals.append(&mut multiple_locals);
                    current_rest = new_rest;
                    continue;
                }
                
                // Then try parsing a single local
                if let Ok((new_rest, local)) = parse_local(current_rest) {
                    locals.push(local);
                    current_rest = new_rest;
                    continue;
                }
                
                // If we can't parse any more locals, break
                break;
            }
            
            (current_rest, locals)
        };

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
        
        // First check if we have a parameter with an identifier
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

/// Parses multiple function parameters defined in a single param instruction.
///
/// Handles cases such as (param f32 f32), where multiple types are defined
/// without identifiers.
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parse_multiple_parameters;
///
/// let params = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     }
/// ];
///
/// assert_eq!(parse_multiple_parameters("(param f32 f32)"), Ok(("", params)));
/// ```
pub fn parse_multiple_parameters(input: &str) -> IResult<Vec<Parameter>> {
    fn inner(input: &str) -> IResult<Vec<Parameter>> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        
        // For (param f32 f32), we need to parse multiple types
        let mut result = Vec::new();
        let mut current_rest = rest;
        
        // Keep parsing types until there are no more
        while let Ok((new_rest, type_)) = preceded(multispace0, parse_type)(current_rest) {
            result.push(Parameter {
                identifier: None,
                type_,
            });
            current_rest = new_rest;
        }
        
        // We need at least one type
        if result.is_empty() {
            return Err(nom::Err::Error(
                VerboseError::from_error_kind(rest, nom::error::ErrorKind::Alt)
            ));
        }
        
        Ok((current_rest, result))
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

/// Parses multiple local variable definitions in a single local instruction.
///
/// Handles cases such as (local f32 f32), where multiple types are defined
/// without identifiers.
///
/// ```
/// use water::ast::{Local, Type, NumericalType};
/// use water::parser::parse_multiple_locals;
///
/// let locals = vec![
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     }
/// ];
///
/// assert_eq!(parse_multiple_locals("(local f32 f32)"), Ok(("", locals)));
/// ```
pub fn parse_multiple_locals(input: &str) -> IResult<Vec<Local>> {
    fn inner(input: &str) -> IResult<Vec<Local>> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;
        
        // For (local f32 f32), we need to parse multiple types
        let mut result = Vec::new();
        let mut current_rest = rest;
        
        // Keep parsing types until there are no more
        while let Ok((new_rest, type_)) = preceded(multispace0, parse_type)(current_rest) {
            result.push(Local {
                identifier: None,
                type_,
            });
            current_rest = new_rest;
        }
        
        // We need at least one type
        if result.is_empty() {
            return Err(nom::Err::Error(
                VerboseError::from_error_kind(rest, nom::error::ErrorKind::Alt)
            ));
        }
        
        Ok((current_rest, result))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("multiple locals", inner)),
    )(input)
}
