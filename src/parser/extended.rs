//! Extended parsing utilities for WebAssembly text format
//!
//! This module provides additional parsers for comments, integers,
//! and other extended syntax elements.

use nom::{
    branch::alt,
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, tuple},
};

use super::IResult;
use crate::parser::utils::{
    parse_block_comment, parse_bracketed, parse_comment,
    parse_hex_int, parse_integer, parse_optional_whitespace,
    parse_required_whitespace, parse_unsigned_int,
};

/// Parses any kind of comment (line or block).
///
/// ```
/// use water::parser::extended::parse_any_comment;
///
/// assert_eq!(parse_any_comment(";; line comment"), Ok(("", " line comment")));
/// assert_eq!(parse_any_comment("(; block comment ;)"), Ok(("", " block comment ")));
/// ```
pub fn parse_any_comment(input: &str) -> IResult<&str> {
    alt((parse_comment, parse_block_comment))(input)
}

/// Parses optional comments and whitespace.
///
/// This is useful for skipping both whitespace and comments in WebAssembly text.
///
/// ```
/// use water::parser::extended::parse_whitespace_and_comments;
///
/// assert_eq!(parse_whitespace_and_comments("  ;; comment\n  code"), Ok(("code", ())));
/// assert_eq!(parse_whitespace_and_comments("(; block ;) code"), Ok(("code", ())));
/// ```
pub fn parse_whitespace_and_comments(
    input: &str,
) -> IResult<()> {
    map(
        many0(alt((
            map(parse_optional_whitespace, |_| ()),
            map(parse_any_comment, |_| ()),
        ))),
        |_| (),
    )(input)
}

/// Parses a memory alignment specification in brackets.
///
/// WebAssembly memory instructions can have alignment hints like `[align=4]`.
///
/// ```
/// use water::parser::extended::parse_alignment;
///
/// assert_eq!(parse_alignment("[align=4]"), Ok(("", "align=4")));
/// assert_eq!(parse_alignment("[offset=8] next"), Ok((" next", "offset=8")));
/// ```
pub fn parse_alignment(input: &str) -> IResult<&str> {
    parse_bracketed(input)
}

/// Parses a memory offset or index, which can be decimal or hexadecimal.
///
/// ```
/// use water::parser::extended::parse_memory_offset;
///
/// assert_eq!(parse_memory_offset("42"), Ok(("", 42)));
/// assert_eq!(parse_memory_offset("0x2A"), Ok(("", 42)));
/// assert_eq!(parse_memory_offset("0xFF rest"), Ok((" rest", 255)));
/// ```
pub fn parse_memory_offset(input: &str) -> IResult<u64> {
    parse_integer(input)
}

/// Parses a data segment size (must be decimal).
///
/// ```
/// use water::parser::extended::parse_data_size;
///
/// assert_eq!(parse_data_size("1024"), Ok(("", 1024)));
/// assert_eq!(parse_data_size("512 bytes"), Ok((" bytes", 512)));
/// ```
pub fn parse_data_size(input: &str) -> IResult<u64> {
    parse_unsigned_int(input)
}

/// Parses a hexadecimal byte literal for data sections.
///
/// ```
/// use water::parser::extended::parse_hex_byte;
///
/// assert_eq!(parse_hex_byte("0xFF"), Ok(("", 255)));
/// assert_eq!(parse_hex_byte("0x00 0x01"), Ok((" 0x01", 0)));
/// ```
pub fn parse_hex_byte(input: &str) -> IResult<u64> {
    parse_hex_int(input)
}

/// Parses an annotated instruction with optional comments.
///
/// WebAssembly text format allows comments after instructions for documentation.
///
/// ```
/// use water::parser::extended::parse_annotated_line;
///
/// assert_eq!(
///     parse_annotated_line("instruction ;; This does something"),
///     Ok(("", ("instruction", Some(" This does something"))))
/// );
/// assert_eq!(
///     parse_annotated_line("instruction"),
///     Ok(("", ("instruction", None)))
/// );
/// ```
pub fn parse_annotated_line(
    input: &str,
) -> IResult<(&str, Option<&str>)> {
    use nom::bytes::complete::take_until;
    use nom::combinator::opt;

    let instruction = take_until(" ");
    let comment_part =
        opt(preceded(parse_required_whitespace, parse_comment));

    tuple((instruction, comment_part))(input)
}

/// Parses a sequence of hex bytes with optional whitespace.
///
/// Useful for parsing data sections in WebAssembly.
///
/// ```
/// use water::parser::extended::parse_hex_sequence;
///
/// assert_eq!(
///     parse_hex_sequence("0x01 0x02 0x03"),
///     Ok(("", vec![1, 2, 3]))
/// );
/// ```
pub fn parse_hex_sequence(input: &str) -> IResult<Vec<u64>> {
    use nom::multi::separated_list0;

    separated_list0(parse_required_whitespace, parse_hex_int)(
        input,
    )
}

/// Parses a table index with optional alignment specification.
///
/// ```
/// use water::parser::extended::parse_table_entry;
///
/// assert_eq!(
///     parse_table_entry("42 [align=4]"),
///     Ok(("", (42, Some("align=4"))))
/// );
/// assert_eq!(
///     parse_table_entry("0x10"),
///     Ok(("", (16, None)))
/// );
/// ```
pub fn parse_table_entry(
    input: &str,
) -> IResult<(u64, Option<&str>)> {
    use nom::combinator::opt;

    let (rest, index) = parse_integer(input)?;
    let (rest, alignment) = opt(preceded(
        parse_required_whitespace,
        parse_alignment,
    ))(rest)?;

    Ok((rest, (index, alignment)))
}

/// Parses a commented block with optional whitespace.
///
/// This handles the common pattern of block comments followed by code.
///
/// ```
/// use water::parser::extended::parse_commented_section;
///
/// assert_eq!(
///     parse_commented_section("(; section comment ;)  content"),
///     Ok(("content", " section comment "))
/// );
/// ```
pub fn parse_commented_section(input: &str) -> IResult<&str> {
    delimited(
        parse_optional_whitespace,
        parse_block_comment,
        parse_optional_whitespace,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_any_comment() {
        assert_eq!(
            parse_any_comment(";; test"),
            Ok(("", " test"))
        );
        assert_eq!(
            parse_any_comment("(; test ;)"),
            Ok(("", " test "))
        );
    }

    #[test]
    fn test_parse_memory_offset() {
        assert_eq!(parse_memory_offset("42"), Ok(("", 42)));
        assert_eq!(parse_memory_offset("0x2A"), Ok(("", 42)));
    }

    #[test]
    fn test_parse_hex_sequence() {
        assert_eq!(
            parse_hex_sequence("0x01 0x02 0x03"),
            Ok(("", vec![1, 2, 3]))
        );
    }

    #[test]
    fn test_parse_table_entry() {
        let result = parse_table_entry("42 [align=4]");
        assert!(result.is_ok());
        let (_, (index, alignment)) = result.unwrap();
        assert_eq!(index, 42);
        assert_eq!(alignment, Some("align=4"));
    }
}
