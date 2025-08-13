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
        let (rest, parameters) = many0(parse_parameter)(rest)?;
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
// TODO: handle cases such as (param f32 f32)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{NumericalType, Type};

    #[test]
    fn test_parse_function_simple() {
        let result = parse_function("(func)");
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: None,
            parameters: vec![],
            local_variables: vec![],
            exports: vec![]
        });
    }

    #[test]
    fn test_parse_function_with_identifier() {
        let result = parse_function("(func $test)");
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: Some("test".into()),
            parameters: vec![],
            local_variables: vec![],
            exports: vec![]
        });
    }

    #[test]
    fn test_parse_function_with_parameters() {
        let result = parse_function("(func $add (param $a i32) (param i64))");
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: Some("add".into()),
            parameters: vec![
                Parameter {
                    identifier: Some("a".into()),
                    type_: Type::Numerical(NumericalType::Int32)
                },
                Parameter {
                    identifier: None,
                    type_: Type::Numerical(NumericalType::Int64)
                }
            ],
            local_variables: vec![],
            exports: vec![]
        });
    }

    #[test]
    fn test_parse_function_with_locals() {
        let result = parse_function("(func (local $temp f32) (local f64))");
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: None,
            parameters: vec![],
            local_variables: vec![
                Local {
                    identifier: Some("temp".into()),
                    type_: Type::Numerical(NumericalType::Float32)
                },
                Local {
                    identifier: None,
                    type_: Type::Numerical(NumericalType::Float64)
                }
            ],
            exports: vec![]
        });
    }

    #[test]
    fn test_parse_function_with_export() {
        let result = parse_function(r#"(func $test (export "testFunc"))"#);
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: Some("test".into()),
            parameters: vec![],
            local_variables: vec![],
            exports: vec!["testFunc".into()]
        });
    }

    #[test]
    fn test_parse_function_complete() {
        let wat = r#"(func $add (export "add") (param $x i32) (param $y i32) (local $result i32))"#;
        let result = parse_function(wat);
        assert!(result.is_ok());
        let (remaining, function) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(function, Function {
            identifier: Some("add".into()),
            parameters: vec![
                Parameter {
                    identifier: Some("x".into()),
                    type_: Type::Numerical(NumericalType::Int32)
                },
                Parameter {
                    identifier: Some("y".into()),
                    type_: Type::Numerical(NumericalType::Int32)
                }
            ],
            local_variables: vec![
                Local {
                    identifier: Some("result".into()),
                    type_: Type::Numerical(NumericalType::Int32)
                }
            ],
            exports: vec!["add".into()]
        });
    }

    #[test]
    fn test_parse_export_basic() {
        let result = parse_export(r#"(export "test")"#);
        assert!(result.is_ok());
        let (remaining, export_name) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(export_name, SmallString::new("test"));
    }

    #[test]
    fn test_parse_export_empty_name() {
        let result = parse_export(r#"(export "")"#);
        assert!(result.is_ok());
        let (remaining, export_name) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(export_name, SmallString::new(""));
    }

    #[test]
    fn test_parse_export_with_whitespace() {
        assert!(parse_export(r#"(  export  "doSomethingUseful")"#).is_ok());
        assert!(parse_export(r#"( export "test" )"#).is_ok());
    }

    #[test]
    fn test_parse_export_failures() {
        assert!(parse_export(r#"(export)"#).is_err());
        assert!(parse_export(r#"(export ")"#).is_err());
        assert!(parse_export(r#"(export "valid""#).is_err());
        assert!(parse_export(r#"export "valid")"#).is_err());
        assert!(parse_export(r#"export "valid""#).is_err());
        assert!(parse_export(r#"(expor "valid")"#).is_err());
        assert!(parse_export(r#"(exporT "valid")"#).is_err());
    }

    #[test]
    fn test_parse_parameter_anonymous() {
        let result = parse_parameter("(param i32)");
        assert!(result.is_ok());
        let (remaining, param) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(param, Parameter {
            identifier: None,
            type_: Type::Numerical(NumericalType::Int32)
        });
    }

    #[test]
    fn test_parse_parameter_named() {
        let result = parse_parameter("(param $number f64)");
        assert!(result.is_ok());
        let (remaining, param) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(param, Parameter {
            identifier: Some("number".into()),
            type_: Type::Numerical(NumericalType::Float64)
        });
    }

    #[test]
    fn test_parse_parameter_with_whitespace() {
        assert!(parse_parameter("( param i32)").is_ok());
        assert!(parse_parameter("(param  i32)").is_ok());
        assert!(parse_parameter("( param  i32 )").is_ok());
        assert!(parse_parameter("( param $number f64)").is_ok());
    }

    #[test]
    fn test_parse_parameter_all_types() {
        let types = [
            ("i32", NumericalType::Int32),
            ("i64", NumericalType::Int64),
            ("f32", NumericalType::Float32),
            ("f64", NumericalType::Float64),
        ];

        for (type_str, expected_type) in types {
            let input = format!("(param {})", type_str);
            let result = parse_parameter(&input);
            assert!(result.is_ok(), "Failed to parse parameter with type {}", type_str);
            let (_, param) = result.unwrap();
            assert_eq!(param.type_, Type::Numerical(expected_type));
        }
    }

    #[test]
    fn test_parse_local_anonymous() {
        let result = parse_local("(local f32)");
        assert!(result.is_ok());
        let (remaining, local) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(local, Local {
            identifier: None,
            type_: Type::Numerical(NumericalType::Float32)
        });
    }

    #[test]
    fn test_parse_local_named() {
        let result = parse_local("(local $number i64)");
        assert!(result.is_ok());
        let (remaining, local) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(local, Local {
            identifier: Some("number".into()),
            type_: Type::Numerical(NumericalType::Int64)
        });
    }

    #[test]
    fn test_parse_local_with_whitespace() {
        assert!(parse_local("( local f32)").is_ok());
        assert!(parse_local("(local  f32)").is_ok());
        assert!(parse_local("( local  f32 )").is_ok());
        assert!(parse_local("( local $number i64)").is_ok());
    }

    #[test]
    fn test_parse_local_all_types() {
        let types = [
            ("i32", NumericalType::Int32),
            ("i64", NumericalType::Int64),
            ("f32", NumericalType::Float32),
            ("f64", NumericalType::Float64),
        ];

        for (type_str, expected_type) in types {
            let input = format!("(local {})", type_str);
            let result = parse_local(&input);
            assert!(result.is_ok(), "Failed to parse local with type {}", type_str);
            let (_, local) = result.unwrap();
            assert_eq!(local.type_, Type::Numerical(expected_type));
        }
    }

    #[test]
    fn test_function_parsing_failures() {
        assert!(parse_function("func)").is_err());
        assert!(parse_function("(func").is_err());
        assert!(parse_function("(function)").is_err());
        assert!(parse_function("(FUNC)").is_err());
        assert!(parse_function("").is_err());
    }

    #[test]
    fn test_parameter_parsing_failures() {
        assert!(parse_parameter("(param)").is_err());
        assert!(parse_parameter("(param invalid_type)").is_err());
        assert!(parse_parameter("param i32)").is_err());
        assert!(parse_parameter("(param i32").is_err());
    }

    #[test]
    fn test_local_parsing_failures() {
        assert!(parse_local("(local)").is_err());
        assert!(parse_local("(local invalid_type)").is_err());
        assert!(parse_local("local i32)").is_err());
        assert!(parse_local("(local i32").is_err());
    }
}
