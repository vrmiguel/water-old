use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, multi::many0, sequence::preceded, combinator::opt,
};

use super::IResult;
use crate::{
    ast::Module, parser::utils::{parse_parenthesis_enclosed, parse_identifier},
    small_string::SmallString, parser::{parse_function, parse_function_import},
};

/// Parses a WebAssembly Text Format module.
///
/// Eats leading whitespace before and after the first
/// parenthesis.
///
/// ```
/// use water::parser::parse_module;
/// use water::ast::Module;
///
/// // Parse empty module
/// let empty_module = Module {
///     functions: vec![],
///     imports: vec![],
///     identifier: None,
/// };
/// assert_eq!(parse_module("(module)"), Ok(("", empty_module)));
/// assert_eq!(parse_module("\n  (module)"), Ok(("", empty_module)));
///
/// // These should fail
/// assert!(parse_module(" (   module").is_err());
/// assert!(parse_module("module)").is_err());
/// assert!(parse_module("(mod)").is_err());
/// ```
pub fn parse_module(input: &str) -> IResult<Module> {
    fn inner(input: &str) -> IResult<Module> {
        let (rest, _) =
            preceded(multispace0, tag("module"))(input)?;

        // Parse optional module identifier
        let (rest, identifier) =
            preceded(multispace0, opt(parse_identifier))(rest)?;

        // Parse function imports
        let (rest, imports) = many0(preceded(
            multispace0,
            parse_function_import,
        ))(rest)?;

        // Parse function definitions
        let (rest, functions) = many0(preceded(
            multispace0,
            parse_function,
        ))(rest)?;

        Ok((
            rest,
            Module {
                functions,
                imports,
                identifier,
            },
        ))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}
