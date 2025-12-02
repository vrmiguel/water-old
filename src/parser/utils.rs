use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, multispace0, none_of},
    combinator::{cut, value},
    error::{context, VerboseError},
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

/// Parses a comma-separated list of items enclosed in
/// parentheses.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_identifier;
/// use water::small_string::SmallString;
///
/// let input = "($a, $b, $c)";
/// let result = water::parser::parse_comma_list(
///     parse_identifier
/// )(input);
/// assert!(result.is_ok());
/// ```
pub fn parse_comma_list<'a, T, F>(
    item_parser: F,
) -> impl FnMut(&'a str) -> IResult<Vec<T>>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    parse_parenthesis_enclosed(nom::multi::separated_list0(
        delimited(multispace0, char(','), multispace0),
        item_parser,
    ))
}

/// Parses a hexadecimal number prefixed with "0x".
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_hex_number;
///
/// assert_eq!(parse_hex_number("0xFF"), Ok(("", 255)));
/// assert_eq!(parse_hex_number("0x10"), Ok(("", 16)));
/// assert_eq!(parse_hex_number("0xDEADBEEF"), Ok(("", 3735928559)));
/// ```
pub fn parse_hex_number(input: &str) -> IResult<u64> {
    context(
        "hexadecimal number",
        preceded(
            tag("0x"),
            nom::combinator::map_res(
                nom::bytes::complete::take_while1(|c: char| {
                    c.is_ascii_hexdigit()
                }),
                |hex_str: &str| u64::from_str_radix(hex_str, 16),
            ),
        ),
    )(input)
}

/// Parses an optional identifier, returning None if no
/// identifier is present.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_optional_identifier;
/// use water::small_string::SmallString;
///
/// assert_eq!(
///     parse_optional_identifier("$var"),
///     Ok(("", Some(SmallString::new("var"))))
/// );
/// assert_eq!(
///     parse_optional_identifier("123"),
///     Ok(("123", None))
/// );
/// ```
pub fn parse_optional_identifier(
    input: &str,
) -> IResult<Option<SmallString>> {
    nom::combinator::opt(parse_identifier)(input)
}
