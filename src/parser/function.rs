use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::context, multi::{many0, many1},
    sequence::preceded, branch::alt,
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
/// // Test with multiple types in a single param and local
/// let multi_type_parameters = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
/// ];
///
/// let multi_type_locals = vec![
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float64)
///     },
/// ];
///
/// let multi_type_function = Function { 
///     identifier: Some("multi".into()), 
///     parameters: multi_type_parameters, 
///     local_variables: multi_type_locals, 
///     exports: vec![] 
/// };
///
/// assert_eq!(
///     parse_function("(func $multi (param i32 i64 f32) (local i32 f64))"),
///     Ok(("", multi_type_function))
/// );
pub fn parse_function(input: &str) -> IResult<'_, Function> {
    fn inner(input: &str) -> IResult<'_, Function> {
        let (rest, _) =
            preceded(multispace0, tag("func"))(input)?;

        let (rest, identifier) =
            preceded(multispace0, opt(parse_identifier))(rest)?;

        // TODO: WASM allows more than one `export` instructions
        // in a function, but they cannot have duplicated
        // names. Check for this either here or at a later step.
        let (rest, exports) = many0(parse_export)(rest)?;
        
        // Parse all parameters (single-type and multi-type)
        let (rest, all_params) = many0(alt((
            parse_multi_type_parameter.map(|params| params),
            parse_parameter.map(|param| vec![param])
        )))(rest)?;
        
        // Flatten the list of parameter vectors
        let parameters = all_params.into_iter().flatten().collect();
        
        // Parse all locals (single-type and multi-type)
        let (rest, all_locals) = many0(alt((
            parse_multi_type_local.map(|locals| locals),
            parse_local.map(|local| vec![local])
        )))(rest)?;
        
        // Flatten the list of local vectors
        let local_variables = all_locals.into_iter().flatten().collect();

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
pub fn parse_export(input: &str) -> IResult<'_, SmallString> {
    fn inner(input: &str) -> IResult<'_, SmallString> {
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
pub fn parse_parameter(input: &str) -> IResult<'_, Parameter> {
    fn inner(input: &str) -> IResult<'_, Parameter> {
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

/// Parses a function parameter that contains multiple types.
/// Returns a vector of parameters, one for each type.
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parse_multi_type_parameter;
///
/// let expected = vec![
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float64)
///     }
/// ];
///
/// assert_eq!(parse_multi_type_parameter("(param i32 f64)"), Ok(("", expected)));
/// ```
pub fn parse_multi_type_parameter(input: &str) -> IResult<'_, Vec<Parameter>> {
    fn inner(input: &str) -> IResult<'_, Vec<Parameter>> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        
        let (rest, types) = 
            many1(preceded(multispace0, parse_type))(rest)?;
            
        let parameters = types
            .into_iter()
            .map(|type_| Parameter { identifier: None, type_ })
            .collect();

        Ok((rest, parameters))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("multi-type parameter", inner)),
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
pub fn parse_local(input: &str) -> IResult<'_, Local> {
    fn inner(input: &str) -> IResult<'_, Local> {
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

/// Parses a local variable definition that contains multiple types.
/// Returns a vector of locals, one for each type.
///
/// ```
/// use water::ast::{Local, Type, NumericalType};
/// use water::parser::parse_multi_type_local;
///
/// let expected = vec![
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float64)
///     }
/// ];
///
/// assert_eq!(parse_multi_type_local("(local i32 f64)"), Ok(("", expected)));
/// ```
pub fn parse_multi_type_local(input: &str) -> IResult<'_, Vec<Local>> {
    fn inner(input: &str) -> IResult<'_, Vec<Local>> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;
        
        let (rest, types) = 
            many1(preceded(multispace0, parse_type))(rest)?;
            
        let locals = types
            .into_iter()
            .map(|type_| Local { identifier: None, type_ })
            .collect();

        Ok((rest, locals))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("multi-type local", inner)),
    )(input)
}
