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

/// Parses a quoted string literal with escape sequence support.
///
/// This function parses strings enclosed in double quotes, handling escaped
/// characters within the string content. It supports standard escape sequences
/// like `\"` for literal quote characters within the string.
///
/// Does not eat leading whitespace.
///
/// # Arguments
/// * `input` - The input string slice to parse
///
/// # Returns
/// * `IResult<&str>` - On success, returns the remaining input and the parsed string content
///
/// # Examples
/// ```
/// use water::parser::parse_string;
///
/// assert_eq!(parse_string("\"hello\""), Ok(("", "hello")));
/// assert_eq!(parse_string("\"hello \\\"world\\\"\""), Ok(("", "hello \"world\"")));
/// assert_eq!(parse_string("\"\""), Ok(("", "")));
/// ```
#[must_use]
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
#[must_use]
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
#[must_use]
pub fn parse_type(input: &str) -> IResult<Type> {
    context(
        "type",
        alt((parse_numerical_type.map(Type::Numerical),)),
    )(input)
}

/// Parses one of the four built-in numerical WASM types.
///
/// Does not eat leading whitespace.
#[must_use]
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
#[must_use]
pub fn parse_index(input: &str) -> IResult<Index> {
    alt((
        parse_identifier
            .map(Index::Identifier),
        nom::character::complete::i64.map(Index::Numerical),
    ))(input)
}

/// Creates a parser combinator for parsing content enclosed in parentheses.
///
/// This higher-order function takes an inner parser and returns a new parser
/// that expects the inner content to be wrapped in parentheses `(...)`. It
/// automatically handles whitespace before the inner content and after it,
/// and provides meaningful error messages for missing closing parentheses.
///
/// The function uses `cut` to provide better error recovery - once an opening
/// parenthesis is found, a missing closing parenthesis will generate a specific
/// error message rather than trying alternative parsers.
///
/// # Type Parameters
/// * `T` - The type that the inner parser produces
/// * `F` - The type of the inner parser function
///
/// # Arguments
/// * `inner` - A parser that will be applied to the content inside the parentheses
///
/// # Returns
/// * A new parser that parses parenthesis-enclosed content using the inner parser
///
/// # Examples
/// ```
/// use nom::character::complete::alpha1;
/// use water::parser::parse_parenthesis_enclosed;
///
/// let mut parser = parse_parenthesis_enclosed(alpha1);
/// assert_eq!(parser("(hello)"), Ok(("", "hello")));
/// assert_eq!(parser("( world )"), Ok(("", "world")));
/// ```
///
/// Based on https://github.com/Geal/nom/blob/761ab0a24fccb4c560367b583b608fbae5f31647/examples/s_expression.rs#L155
#[must_use]
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

/// Determines whether a character is valid in a WebAssembly Text Format identifier.
///
/// According to the WebAssembly Text Format specification, identifiers can contain
/// ASCII alphanumeric characters plus a specific set of special characters. This
/// function is used by the identifier parser to validate character sequences.
///
/// # Arguments
/// * `ch` - The character to test for validity in an identifier
///
/// # Returns
/// * `bool` - `true` if the character is acceptable in a WASM identifier, `false` otherwise
///
/// # Valid Characters
/// * ASCII letters (a-z, A-Z)
/// * ASCII digits (0-9)  
/// * Special characters: `! # $ % & ´ * + - . / : < = > ? @ \ ^ _ ` | ~`
///
/// # Examples
/// ```
/// use water::parser::utils::is_acceptable_identifier_character;
///
/// assert!(is_acceptable_identifier_character('a'));
/// assert!(is_acceptable_identifier_character('Z'));
/// assert!(is_acceptable_identifier_character('5'));
/// assert!(is_acceptable_identifier_character('_'));
/// assert!(is_acceptable_identifier_character('$'));
/// assert!(!is_acceptable_identifier_character(' '));
/// assert!(!is_acceptable_identifier_character('('));
/// ```
#[must_use]
pub fn is_acceptable_identifier_character(ch: char) -> bool {
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
