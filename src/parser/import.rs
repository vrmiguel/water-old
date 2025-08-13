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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{NumericalType, Parameter, Type};

    #[test]
    fn test_parse_function_import_basic() {
        let import_wat = r#"(import "console" "log" (func))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "console".into(),
            fn_name: "log".into(),
            signature: Function {
                identifier: None,
                parameters: vec![],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_with_identifier() {
        let import_wat = r#"(import "console" "log" (func $log))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "console".into(),
            fn_name: "log".into(),
            signature: Function {
                identifier: Some("log".into()),
                parameters: vec![],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_with_parameters() {
        let import_wat = r#"(import "console" "log" (func $log (param f32) (param f32)))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "console".into(),
            fn_name: "log".into(),
            signature: Function {
                identifier: Some("log".into()),
                parameters: vec![
                    Parameter { 
                        identifier: None, 
                        type_: Type::Numerical(NumericalType::Float32) 
                    },
                    Parameter { 
                        identifier: None, 
                        type_: Type::Numerical(NumericalType::Float32) 
                    }
                ],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_with_named_parameters() {
        let import_wat = r#"(import "math" "pow" (func $pow (param $base f64) (param $exp f64)))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "math".into(),
            fn_name: "pow".into(),
            signature: Function {
                identifier: Some("pow".into()),
                parameters: vec![
                    Parameter { 
                        identifier: Some("base".into()), 
                        type_: Type::Numerical(NumericalType::Float64) 
                    },
                    Parameter { 
                        identifier: Some("exp".into()), 
                        type_: Type::Numerical(NumericalType::Float64) 
                    }
                ],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_various_types() {
        let types = [
            ("i32", NumericalType::Int32),
            ("i64", NumericalType::Int64),
            ("f32", NumericalType::Float32),
            ("f64", NumericalType::Float64),
        ];

        for (type_str, expected_type) in types {
            let import_wat = format!(r#"(import "test" "func" (func (param {})))"#, type_str);
            let result = parse_function_import(&import_wat);
            assert!(result.is_ok(), "Failed to parse import with type {}", type_str);
            let (_, import) = result.unwrap();
            assert_eq!(import.signature.parameters.len(), 1);
            assert_eq!(import.signature.parameters[0].type_, Type::Numerical(expected_type));
        }
    }

    #[test]
    fn test_parse_function_import_empty_strings() {
        let import_wat = r#"(import "" "" (func))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "".into(),
            fn_name: "".into(),
            signature: Function {
                identifier: None,
                parameters: vec![],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_with_whitespace() {
        let import_wat = r#"(  import   "console"   "log"   (func $log)  )"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import.namespace, SmallString::new("console"));
        assert_eq!(import.fn_name, SmallString::new("log"));
    }

    #[test]
    fn test_parse_function_import_multiline() {
        let import_wat = r#"(import "console" "log" 
                              (func $log 
                                (param $x i32) 
                                (param $y i32)))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import.signature.parameters.len(), 2);
    }

    #[test]
    fn test_parse_function_import_failures() {
        let invalid_imports = vec![
            r#"import "console" "log" (func))"#, // missing opening paren
            r#"(import "console" "log" (func)"#, // missing closing paren
            r#"(import "console" (func))"#,      // missing function name
            r#"(import (func))"#,                // missing namespace and name
            r#"(import "console" "log")"#,       // missing function
            r#"(import "console" "log" func)"#,  // function not in parens
            r#"(IMPORT "console" "log" (func))"#, // wrong case
            r#"(import console "log" (func))"#,  // unquoted namespace
            r#"(import "console" log (func))"#,  // unquoted function name
        ];

        for invalid_import in invalid_imports {
            let result = parse_function_import(invalid_import);
            assert!(result.is_err(), "Should fail parsing: {}", invalid_import);
        }
    }

    #[test]
    fn test_parse_function_import_complex() {
        let import_wat = r#"(import "env" "memory_grow" (func $grow (param $pages i32)))"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, import) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(import, FunctionImport {
            namespace: "env".into(),
            fn_name: "memory_grow".into(),
            signature: Function {
                identifier: Some("grow".into()),
                parameters: vec![
                    Parameter { 
                        identifier: Some("pages".into()), 
                        type_: Type::Numerical(NumericalType::Int32) 
                    }
                ],
                exports: vec![],
                local_variables: vec![],
            }
        });
    }

    #[test]
    fn test_parse_function_import_with_trailing_content() {
        let import_wat = r#"(import "test" "func" (func)) extra"#;
        let result = parse_function_import(import_wat);
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(remaining, " extra");
    }
}
