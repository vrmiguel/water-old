use nom::{bytes::complete::tag, error::context};

use super::IResult;
use crate::{
    ast::Module,
    parser::utils::{parse_parenthesis_enclosed, ws},
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
        let (rest, _) = ws(tag("module"))(input)?;

        Ok((rest, Module {}))
    }

    ws(parse_parenthesis_enclosed(context(
        "module", inner,
    )))(input)
}
