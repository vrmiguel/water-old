use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::context, multi::many0,
    sequence::preceded,
};

use super::IResult;
use crate::{
    ast::{Function, Local, Parameter},
    parser::utils::{
        bezeichner_parsen, in_klammern_eingeschlossen_parsen,
        zeichenkette_parsen, typ_parsen,
    },
    small_string::SmallString,
};

/// Parses a function definition.
///
/// ```
/// use water::parser::funktion_parsen;
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
///     funktion_parsen("(func $add (param $number f64) (param i64) (local $l1 i32) (local f32))"),
///     Ok(("", function))
/// );
/// ```
pub fn funktion_parsen(input: &str) -> IResult<Function> {
    fn innere(input: &str) -> IResult<Function> {
        let (rest, _) =
            preceded(multispace0, tag("func"))(input)?;

        let (rest, identifier) =
            preceded(multispace0, opt(bezeichner_parsen))(rest)?;

        // TODO: WASM allows more than one `export` instructions
        // in a function, but they cannot have duplicated
        // names. Check for this either here or at a later step.
        let (rest, exports) = many0(export_parsen)(rest)?;
        let (rest, parameters) = many0(parameter_parsen)(rest)?;
        let (rest, local_variables) = many0(lokale_parsen)(rest)?;

        let function = Function {
            identifier,
            parameters,
            local_variables,
            exports,
        };

        Ok((rest, function))
    }

    in_klammern_eingeschlossen_parsen(context("function", innere))(input)
}

/// Parses an `export` definition.
///
/// ```
/// use water::parser::export_parsen;
/// use water::small_string::SmallString;
///
/// assert_eq!(export_parsen(r#"(export "add")"#), Ok(("", "add".into())));
/// assert_eq!(export_parsen(r#"(  export  "doSomethingUseful")"#), Ok(("", "doSomethingUseful".into())));
/// // WASM allows "" as a valid export name
/// assert_eq!(export_parsen(r#"(export"")"#), Ok(("", "".into())));
///
/// // Wrong: missing name
/// assert!(export_parsen(r#"(export)"#).is_err());
///
/// // Wrong: unclosed quoted string
/// assert!(export_parsen(r#"(export ")"#).is_err());
///
/// // Wrong: missing terminating parenthesis
/// assert!(export_parsen(r#"(export "valid""#).is_err());
///
/// // Wrong: missing first parenthesis
/// assert!(export_parsen(r#"export "valid")"#).is_err());
///
/// // Wrong: missing both parenthesis
/// assert!(export_parsen(r#"export "valid""#).is_err());
///
/// // Wrong: incorrect keyword
/// assert!(export_parsen(r#"(expor "valid""))"#).is_err());
/// assert!(export_parsen(r#"(exporT "valid""))"#).is_err());
///
/// // Wrong: extra string quote
/// assert!(export_parsen(r#"(export "valid"")"#).is_err());
/// ```
pub fn export_parsen(input: &str) -> IResult<SmallString> {
    fn innere(input: &str) -> IResult<SmallString> {
        let (rest, _) =
            preceded(multispace0, tag("export"))(input)?;

        let (rest, name) =
            preceded(multispace0, zeichenkette_parsen)(rest)?;

        Ok((rest, name.into()))
    }

    in_klammern_eingeschlossen_parsen(context("export", innere))(input)
}

/// Parses a function parameter.
///
/// Handles leading whitespace.
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parameter_parsen;
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
/// assert_eq!(parameter_parsen("(param i32)"), Ok(("", anonymous_i32)));
/// assert_eq!(parameter_parsen("( param $number f64)"), Ok(("", named_f64)));
/// ```
// TODO: handle cases such as (param f32 f32)
pub fn parameter_parsen(input: &str) -> IResult<Parameter> {
    fn innere(input: &str) -> IResult<Parameter> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, bezeichner_parsen))(rest)?;
        let (rest, type_) =
            preceded(multispace0, typ_parsen)(rest)?;

        let parameter = Parameter { identifier, type_ };

        Ok((rest, parameter))
    }

    preceded(
        multispace0,
        in_klammern_eingeschlossen_parsen(context("parameter", innere)),
    )(input)
}

/// Parses a local variable definition.
///
/// ```
/// use water::ast::{Local, Type, NumericalType};
/// use water::parser::lokale_parsen;
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
/// assert_eq!(lokale_parsen("(local f32)"), Ok(("", anonymous_f32)));
/// assert_eq!(lokale_parsen("( local $number i64)"), Ok(("", named_i64)));
/// ```
pub fn lokale_parsen(input: &str) -> IResult<Local> {
    fn innere(input: &str) -> IResult<Local> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, bezeichner_parsen))(rest)?;
        let (rest, type_) =
            preceded(multispace0, typ_parsen)(rest)?;

        let local = Local { identifier, type_ };

        Ok((rest, local))
    }

    preceded(
        multispace0,
        in_klammern_eingeschlossen_parsen(context("local", innere)),
    )(input)
}
