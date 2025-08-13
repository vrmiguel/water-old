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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string(r#""hello""#), Ok(("", "hello")));
        assert_eq!(parse_string(r#""hello world""#), Ok(("", "hello world")));
        assert_eq!(parse_string(r#""""#), Ok(("", "")));
        assert_eq!(parse_string(r#""test\" escaped""#), Ok(("", r#"test\" escaped"#)));
        
        assert!(parse_string(r#""unclosed string"#).is_err());
        assert!(parse_string(r#"no quotes"#).is_err());
        assert!(parse_string(r#""multiple""quotes""#).is_ok());
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(parse_identifier("$hello"), Ok(("", SmallString::new("hello"))));
        assert_eq!(parse_identifier("$test123"), Ok(("", SmallString::new("test123"))));
        assert_eq!(parse_identifier("$with-dash"), Ok(("", SmallString::new("with-dash"))));
        assert_eq!(parse_identifier("$with.dot"), Ok(("", SmallString::new("with.dot"))));
        assert_eq!(parse_identifier("$with_underscore"), Ok(("", SmallString::new("with_underscore"))));
        assert_eq!(parse_identifier("$special!@#$%"), Ok(("", SmallString::new("special!@#$%"))));
        
        assert_eq!(parse_identifier("$idx remaining"), Ok((" remaining", SmallString::new("idx"))));
        
        assert!(parse_identifier("no_dollar").is_err());
        assert!(parse_identifier("$").is_err());
        assert!(parse_identifier("").is_err());
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type("i32"), Ok(("", Type::Numerical(NumericalType::Int32))));
        assert_eq!(parse_type("i64"), Ok(("", Type::Numerical(NumericalType::Int64))));
        assert_eq!(parse_type("f32"), Ok(("", Type::Numerical(NumericalType::Float32))));
        assert_eq!(parse_type("f64"), Ok(("", Type::Numerical(NumericalType::Float64))));
        
        assert!(parse_type("invalid").is_err());
        assert!(parse_type("i33").is_err());
        assert!(parse_type("").is_err());
    }

    #[test]
    fn test_parse_numerical_type() {
        assert_eq!(parse_numerical_type("i32"), Ok(("", NumericalType::Int32)));
        assert_eq!(parse_numerical_type("i64"), Ok(("", NumericalType::Int64)));
        assert_eq!(parse_numerical_type("f32"), Ok(("", NumericalType::Float32)));
        assert_eq!(parse_numerical_type("f64"), Ok(("", NumericalType::Float64)));
        
        assert_eq!(parse_numerical_type("i32 remaining"), Ok((" remaining", NumericalType::Int32)));
        
        assert!(parse_numerical_type("invalid").is_err());
        assert!(parse_numerical_type("i33").is_err());
        assert!(parse_numerical_type("f128").is_err());
    }

    #[test]
    fn test_parse_index() {
        assert_eq!(parse_index("$variable"), Ok(("", Index::Identifier("variable".into()))));
        assert_eq!(parse_index("42"), Ok(("", Index::Numerical(42))));
        assert_eq!(parse_index("0"), Ok(("", Index::Numerical(0))));
        assert_eq!(parse_index("-1"), Ok(("", Index::Numerical(-1))));
        
        assert_eq!(parse_index("$var remaining"), Ok((" remaining", Index::Identifier("var".into()))));
        assert_eq!(parse_index("123 remaining"), Ok((" remaining", Index::Numerical(123))));
        
        assert!(parse_index("").is_err());
        assert!(parse_index("invalid").is_err());
    }

    #[test]
    fn test_parse_parenthesis_enclosed() {
        let simple_parser = |input: &str| -> IResult<&str> {
            nom::bytes::complete::tag("test")(input)
        };
        
        let mut parser = parse_parenthesis_enclosed(simple_parser);
        assert_eq!(parser("(test)"), Ok(("", "test")));
        assert_eq!(parser("( test)"), Ok(("", "test")));
        assert_eq!(parser("(test )"), Ok(("", "test")));
        assert_eq!(parser("( test )"), Ok(("", "test")));
        
        assert!(parser("test)").is_err());
        assert!(parser("(test").is_err());
        assert!(parser("test").is_err());
        assert!(parser("(wrong)").is_err());
    }

    #[test]
    fn test_is_acceptable_identifier_character() {
        assert!(is_acceptable_identifier_character('a'));
        assert!(is_acceptable_identifier_character('Z'));
        assert!(is_acceptable_identifier_character('0'));
        assert!(is_acceptable_identifier_character('9'));
        assert!(is_acceptable_identifier_character('_'));
        assert!(is_acceptable_identifier_character('-'));
        assert!(is_acceptable_identifier_character('.'));
        assert!(is_acceptable_identifier_character('!'));
        assert!(is_acceptable_identifier_character('#'));
        assert!(is_acceptable_identifier_character('$'));
        
        assert!(!is_acceptable_identifier_character(' '));
        assert!(!is_acceptable_identifier_character('\t'));
        assert!(!is_acceptable_identifier_character('\n'));
        assert!(!is_acceptable_identifier_character('('));
        assert!(!is_acceptable_identifier_character(')'));
        assert!(!is_acceptable_identifier_character('"'));
    }
}
