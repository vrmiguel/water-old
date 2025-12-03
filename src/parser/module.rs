use nom::{error::context, sequence::preceded};

use super::IResult;
use crate::{
    ast::Module,
    parser::utils::{
        parse_keyword, parse_optional_whitespace,
        parse_parenthesis_enclosed,
    },
};

/// Parses a WebAssembly Text Format module.
///
/// Eats leading whitespace before and after the first
/// parenthesis.
///
/// ```
/// use water::parser::parse_module;
///
/// assert!(parse_module("(module)").is_ok());
/// assert!(parse_module("\n  (module)").is_ok());
///
/// assert!(parse_module(" (   module").is_err());
/// assert!(parse_module("module)").is_err());
/// assert!(parse_module("(mod)").is_err());
/// ```
pub fn parse_module(input: &str) -> IResult<Module> {
    fn inner(input: &str) -> IResult<Module> {
        let (rest, keyword) = preceded(
            parse_optional_whitespace,
            parse_keyword,
        )(input)?;

        if keyword != "module" {
            return Err(nom::Err::Error(
                nom::error::VerboseError {
                    errors: vec![(
                        input,
                        nom::error::VerboseErrorKind::Context(
                            "expected 'module' keyword",
                        ),
                    )],
                },
            ));
        }

        Ok((rest, Module {}))
    }

    preceded(
        parse_optional_whitespace,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}
