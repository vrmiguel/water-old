use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{
        char, multispace0, multispace1, none_of,
    },
    combinator::{cut, opt, value},
    error::{context, VerboseError},
    multi::separated_list0,
    sequence::{delimited, preceded},
    Parser,
};

use super::IResult;
use crate::{
    ast::{Index, NumericalType, Type},
    small_string::SmallString,
};

pub fn parse_string(input: &str) -> IResult<&str> {
    let esc = escaped(none_of("\\\""), '\\', tag("\""));
    let esc_or_empty = alt((esc, tag("")));

    delimited(tag("\""), esc_or_empty, tag("\""))(input)
}

/// Parses an identifier. WebAssembly Text Format identifiers
/// always start with `$`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_identifier;
/// use water::small_string::SmallString;
///
/// assert_eq!(parse_identifier("$idx"), Ok(("", SmallString::new("idx"))));
/// assert_eq!(parse_identifier("$asd_aa? a"), Ok((" a", SmallString::new("asd_aa?"))));
/// ```
pub fn parse_identifier(input: &str) -> IResult<SmallString> {
    let (rest, identifier) = context(
        "identifier",
        preceded(
            char('$'),
            take_while1(is_acceptable_identifier_character),
        ),
    )(input)?;

    Ok((rest, SmallString::new(identifier)))
}

/// Parses a WASM type.
///
/// Does not eat leading whitespace.
pub fn parse_type(input: &str) -> IResult<Type> {
    context(
        "type",
        alt((parse_numerical_type.map(Type::Numerical),)),
    )(input)
}

/// Parses one of the four built-in numerical WASM types.
///
/// Does not eat leading whitespace.
pub fn parse_numerical_type(
    input: &str,
) -> IResult<NumericalType> {
    alt((
        value(NumericalType::Int32, tag("i32")),
        value(NumericalType::Int64, tag("i64")),
        value(NumericalType::Float32, tag("f32")),
        value(NumericalType::Float64, tag("f64")),
    ))(input)
}

/// Parses an index, either numerical or as an identifier.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_index;
/// use water::small_string::SmallString;
/// use water::ast::Index;
///
/// assert_eq!(parse_index("$var"), Ok(("", Index::Identifier("var".into()))));
/// assert_eq!(parse_index("5"), Ok(("", Index::Numerical(5))));
/// ```
pub fn parse_index(input: &str) -> IResult<Index> {
    alt((
        parse_identifier
            .map(SmallString::new)
            .map(Index::Identifier),
        nom::character::complete::i64.map(Index::Numerical),
    ))(input)
}

// Based on https://github.com/Geal/nom/blob/761ab0a24fccb4c560367b583b608fbae5f31647/examples/s_expression.rs#L155
pub fn parse_parenthesis_enclosed<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context(
            "closing parenthesis",
            cut(preceded(multispace0, char(')'))),
        ),
    )
}

fn is_acceptable_identifier_character(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '!' | '#'
                | '$'
                | '%'
                | '&'
                | '´'
                | '*'
                | '+'
                | '-'
                | '.'
                | '/'
                | ':'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '\\'
                | '^'
                | '_'
                | '`'
                | '|'
                | '~'
        )
}

/// Parses a whitespace-separated list of elements.
///
/// This combinator is useful for parsing sequences of items
/// where elements are separated by one or more whitespace
/// characters. Returns an empty vector if no elements are
/// found.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_whitespace_separated;
/// use water::parser::parse_numerical_type;
/// use water::ast::NumericalType;
///
/// let result = parse_whitespace_separated(
///     parse_numerical_type
/// )("i32 i64 f32");
/// assert_eq!(result, Ok(("", vec![
///     NumericalType::Int32,
///     NumericalType::Int64,
///     NumericalType::Float32
/// ])));
///
/// let empty = parse_whitespace_separated(
///     parse_numerical_type
/// )("");
/// assert_eq!(empty, Ok(("", vec![])));
/// ```
pub fn parse_whitespace_separated<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<Vec<T>>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    separated_list0(multispace1, inner)
}

/// Parses an optional element, returning `Some(T)` if the
/// parser succeeds or `None` if it fails without consuming
/// input.
///
/// This combinator wraps nom's `opt` with proper error context
/// for WebAssembly parsing. Useful for parsing elements that
/// may or may not be present, such as optional identifiers or
/// export declarations.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_optional;
/// use water::parser::parse_identifier;
/// use water::small_string::SmallString;
///
/// let with_id = parse_optional(parse_identifier)("$foo rest");
/// assert_eq!(with_id, Ok((" rest", Some(SmallString::new("foo")))));
///
/// let without_id = parse_optional(parse_identifier)("rest");
/// assert_eq!(without_id, Ok(("rest", None)));
/// ```
pub fn parse_optional<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<Option<T>>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    opt(inner)
}
