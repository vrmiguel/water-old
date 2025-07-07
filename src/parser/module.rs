use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, multi::many0, sequence::preceded,
};

use super::IResult;
use crate::{
    ast::Module, 
    parser::{
        function::parse_function,
        import::parse_function_import,
        utils::parse_parenthesis_enclosed,
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

        // Parse function imports and function definitions
        let (rest, imports) = many0(preceded(
            multispace0,
            parse_function_import,
        ))(rest)?;
        
        let (rest, functions) = many0(preceded(
            multispace0,
            parse_function,
        ))(rest)?;

        Ok((rest, Module { functions, imports }))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}
