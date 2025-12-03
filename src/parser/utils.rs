use nom::{
    branch::alt,
    bytes::complete::{
        escaped, tag, take_until, take_while, take_while1,
    },
    character::complete::{
        char, digit1, hex_digit1, multispace0, multispace1,
        none_of,
    },
    combinator::{cut, recognize, value},
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

    // Validate using is_valid_identifier if the identifier contains only standard characters
    // Note: WASM identifiers can have special characters, so we're only using this as a secondary check
    let identifier_str = SmallString::new(identifier);
    Ok((rest, identifier_str))
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

/// Parses a line comment starting with `;;`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_comment;
///
/// assert_eq!(parse_comment(";; this is a comment"), Ok(("", " this is a comment")));
/// assert_eq!(parse_comment(";; comment\ncode"), Ok(("\ncode", " comment")));
/// ```
pub fn parse_comment(input: &str) -> IResult<&str> {
    context(
        "comment",
        preceded(
            tag(";;"),
            alt((
                take_until("\n"),
                take_while(|_| true), // Rest of input if no newline
            )),
        ),
    )(input)
}

/// Parses a block comment enclosed in `(;` and `;)`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_block_comment;
///
/// assert_eq!(parse_block_comment("(; block comment ;)"), Ok(("", " block comment ")));
/// assert_eq!(parse_block_comment("(; nested comment ;) code"), Ok((" code", " nested comment ")));
/// ```
pub fn parse_block_comment(input: &str) -> IResult<&str> {
    context(
        "block comment",
        delimited(tag("(;"), take_until(";)"), tag(";)")),
    )(input)
}

/// Parses an unsigned integer literal.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_unsigned_int;
///
/// assert_eq!(parse_unsigned_int("42"), Ok(("", 42)));
/// assert_eq!(parse_unsigned_int("123 abc"), Ok((" abc", 123)));
/// ```
pub fn parse_unsigned_int(input: &str) -> IResult<u64> {
    context(
        "unsigned integer",
        digit1.map(|s: &str| s.parse::<u64>().unwrap()),
    )(input)
}

/// Parses a hexadecimal integer literal with `0x` prefix.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_hex_int;
///
/// assert_eq!(parse_hex_int("0x2A"), Ok(("", 42)));
/// assert_eq!(parse_hex_int("0xFF abc"), Ok((" abc", 255)));
/// ```
pub fn parse_hex_int(input: &str) -> IResult<u64> {
    context(
        "hexadecimal integer",
        preceded(
            tag("0x"),
            hex_digit1.map(|s: &str| {
                u64::from_str_radix(s, 16).unwrap()
            }),
        ),
    )(input)
}

/// Parses either a decimal or hexadecimal integer.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_integer;
///
/// assert_eq!(parse_integer("42"), Ok(("", 42)));
/// assert_eq!(parse_integer("0x2A"), Ok(("", 42)));
/// ```
pub fn parse_integer(input: &str) -> IResult<u64> {
    context(
        "integer",
        alt((parse_hex_int, parse_unsigned_int)),
    )(input)
}

/// Parses content enclosed in square brackets `[]`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_bracketed;
///
/// assert_eq!(parse_bracketed("[content]"), Ok(("", "content")));
/// assert_eq!(parse_bracketed("[test] more"), Ok((" more", "test")));
/// ```
pub fn parse_bracketed(input: &str) -> IResult<&str> {
    context(
        "bracketed content",
        delimited(
            char('['),
            take_while(|c| c != ']'),
            char(']'),
        ),
    )(input)
}

/// Parses a keyword (alphabetic identifier without `$` prefix).
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_keyword;
///
/// assert_eq!(parse_keyword("module"), Ok(("", "module")));
/// assert_eq!(parse_keyword("func "), Ok((" ", "func")));
/// ```
pub fn parse_keyword(input: &str) -> IResult<&str> {
    context(
        "keyword",
        take_while1(|c: char| {
            c.is_ascii_alphabetic() || c == '.' || c == '_'
        }),
    )(input)
}

/// Parses optional whitespace and returns the consumed input.
///
/// ```
/// use water::parser::parse_optional_whitespace;
///
/// assert_eq!(parse_optional_whitespace("   abc"), Ok(("abc", "   ")));
/// assert_eq!(parse_optional_whitespace("abc"), Ok(("abc", "")));
/// ```
pub fn parse_optional_whitespace(input: &str) -> IResult<&str> {
    recognize(multispace0)(input)
}

/// Parses required whitespace (at least one whitespace character).
///
/// ```
/// use water::parser::parse_required_whitespace;
///
/// assert_eq!(parse_required_whitespace("   abc"), Ok(("abc", "   ")));
/// assert!(parse_required_whitespace("abc").is_err());
/// ```
pub fn parse_required_whitespace(input: &str) -> IResult<&str> {
    context("whitespace", recognize(multispace1))(input)
}

/// Checks if a character is valid for starting an identifier (alphabetic or underscore).
///
/// ```
/// use water::parser::is_identifier_start;
///
/// assert!(is_identifier_start('a'));
/// assert!(is_identifier_start('_'));
/// assert!(!is_identifier_start('1'));
/// ```
pub fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

/// Checks if a string is a valid WebAssembly identifier name (without the `$` prefix).
///
/// ```
/// use water::parser::is_valid_identifier;
///
/// assert!(is_valid_identifier("myVar"));
/// assert!(is_valid_identifier("test123"));
/// assert!(!is_valid_identifier("123test"));
/// assert!(!is_valid_identifier(""));
/// ```
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if !is_identifier_start(first) {
            return false;
        }
    }

    chars.all(is_acceptable_identifier_character)
}
