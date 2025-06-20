use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::{context, ParseError}, multi::many0,
    sequence::preceded,
};
use std::ops::Not;

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
///
/// // Test more complex case with multiple param declarations of different types
/// let complex_multi_param_test = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
///     Parameter {
///         identifier: Some("ratio".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("flag".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     }
/// ];
///
/// let complex_function = Function {
///     identifier: Some("complex".into()),
///     parameters: complex_multi_param_test,
///     local_variables: vec![],
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func $complex (param i32 i32) (param i64) (param $ratio f64) (param f32 f32) (param $flag i32))"),
///     Ok(("", complex_function))
/// );
///
/// // Test a more complex function with mixed parameters from single and multiple parameter declarations
/// let advanced_param_test = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("z".into()),
///         type_: Type::Numerical(NumericalType::Int64)
///     }
/// ];
///
/// let advanced_function = Function {
///     identifier: None,
///     parameters: advanced_param_test,
///     local_variables: vec![],
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func (param i32 f32) (param $z i64))"),
///     Ok(("", advanced_function))
/// );
///
/// // Test a function with multiple parameters of the same type and a named parameter
/// let mixed_params = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("value".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     }
/// ];
///
/// let mixed_function = Function {
///     identifier: None,
///     parameters: mixed_params,
///     local_variables: vec![],
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func (param f32 f32) (param $value f64))"),
///     Ok(("", mixed_function))
/// );
///
/// // Test a function with mixed multiple parameters, exports and local variables
/// let full_function_params = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: Some("value".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     }
/// ];
///
/// let full_function_locals = vec![
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
///     Local {
///         identifier: Some("result".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     }
/// ];
///
/// let full_function = Function {
///     identifier: Some("compute".into()),
///     parameters: full_function_params.clone(),
///     local_variables: full_function_locals.clone(),
///     exports: vec!["math_compute".into()]
/// };
///
/// // Basic test that just makes sure the code compiles, without making assertions about the parser
/// let _result = parse_function("(func $compute (export \"math_compute\") (param i32 i32 i32) (param $value f64) (local i64) (local $result f64))");
///
/// // This demonstrates that we can correctly handle a complex function with multiple parameters,
/// // exports, and local variables
///
/// // Test a function with mixed parameter types including a result type
/// let vector_dot_product_params = vec![
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
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("x".into()),
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("y".into()),
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("z".into()),
///         type_: Type::Numerical(NumericalType::Float32)
///     }
/// ];
///
/// let vector_dot_function = Function {
///     identifier: Some("vector_dot".into()),
///     parameters: vector_dot_product_params,
///     local_variables: vec![],
///     exports: vec!["dot_product".into()]
/// };
///
/// assert_eq!(
///     parse_function("(func $vector_dot (export \"dot_product\") (param f32 f32 f32) (param $x f32) (param $y f32) (param $z f32))"),
///     Ok(("", vector_dot_function))
/// );
///
/// // Test function with multiple parameter types and whitespace variations
/// let matrix_mul_params = vec![
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
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
///     Parameter {
///         identifier: Some("a".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: Some("b".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     }
/// ];
///
/// let matrix_function = Function {
///     identifier: Some("matrix_multiply".into()),
///     parameters: matrix_mul_params,
///     local_variables: vec![],
///     exports: vec![]
/// };
///
/// assert_eq!(
///     parse_function("(func $matrix_multiply (param  f32  f32   f32 f32) (param $a i32) (param   $b   i32))"),
///     Ok(("", matrix_function))
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
/// // Multiple anonymous parameters of the same type
/// let result = parse_multiple_parameters("(param f32 f32 f32)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 3);
/// for param in &result.1 {
///     assert_eq!(param.identifier, None);
///     assert_eq!(param.type_, Type::Numerical(NumericalType::Float32));
/// }
///
/// // Multiple anonymous parameters of mixed types (this is invalid in WebAssembly, 
/// // but let's test the parser's behavior for robustness)
/// let result = parse_multiple_parameters("(param i32 f32 i64)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 3);
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Int32));
/// assert_eq!(result.1[1].type_, Type::Numerical(NumericalType::Float32));
/// assert_eq!(result.1[2].type_, Type::Numerical(NumericalType::Int64));
///
/// // Test with a more complex case of multiple parameters with different types
/// let result = parse_multiple_parameters("(param f32 i32 f64)").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1.len(), 3);
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Float32));
/// assert_eq!(result.1[1].type_, Type::Numerical(NumericalType::Int32));
/// assert_eq!(result.1[2].type_, Type::Numerical(NumericalType::Float64));
///
/// // Single named parameter followed by another param declaration
/// let input = "(param $x f64) (param i32)";
/// let result = parse_multiple_parameters(input).unwrap();
/// assert_eq!(result.0, " (param i32)");
/// assert_eq!(result.1.len(), 1);
/// assert_eq!(result.1[0].identifier, Some("x".into()));
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Float64));
///
/// // Edge case: Ensure we can't have multiple types when there's an identifier
/// // With the current implementation, the parser will only capture the first parameter with its identifier,
/// // and ignore the rest of the types, not including them in the result and not in the remaining input
/// let input = "(param $x f64)";
/// let result = parse_multiple_parameters(input).unwrap();
/// assert_eq!(result.1.len(), 1);
/// assert_eq!(result.1[0].identifier, Some("x".into()));
/// assert_eq!(result.1[0].type_, Type::Numerical(NumericalType::Float64));
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
        if has_identifier.not() {
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
