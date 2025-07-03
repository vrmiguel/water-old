use nom::{
    bytes::complete::tag, character::complete::multispace0,
    combinator::opt, error::context, multi::many0,
    sequence::preceded,
};

use super::IResult;
use crate::{
    ast::Module, 
    parser::{
        utils::{parse_identifier, parse_parenthesis_enclosed},
        parse_function, parse_function_import,
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
        let (rest, _) =
            preceded(multispace0, tag("module"))(input)?;
        let (rest, identifier) =
            preceded(multispace0, opt(parse_identifier))(rest)?;
        let (rest, functions) = many0(parse_function)(rest)?;
        let (rest, imports) = many0(parse_function_import)(rest)?;

        Ok((rest, Module {
            functions,
            imports,
            identifier,
        }))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}
