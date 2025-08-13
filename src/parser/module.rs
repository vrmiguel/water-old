use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, sequence::preceded,
};

use super::IResult;
use crate::{
    ast::Module, parser::utils::parse_parenthesis_enclosed,
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

        Ok((rest, Module {}))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_module_basic() {
        let result = parse_module("(module)");
        assert!(result.is_ok());
        let (remaining, module) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(module, Module {});
    }

    #[test]
    fn test_parse_module_with_leading_whitespace() {
        assert!(parse_module("  (module)").is_ok());
        assert!(parse_module("\n(module)").is_ok());
        assert!(parse_module("\t (module)").is_ok());
        assert!(parse_module("   \n  (module)").is_ok());
    }

    #[test]
    fn test_parse_module_with_whitespace_inside() {
        assert!(parse_module("( module)").is_ok());
        assert!(parse_module("(module )").is_ok());
        assert!(parse_module("( module )").is_ok());
        assert!(parse_module("(\n  module\n)").is_ok());
    }

    #[test]
    fn test_parse_module_failures() {
        assert!(parse_module("module)").is_err());
        assert!(parse_module("(module").is_err());
        assert!(parse_module("(mod)").is_err());
        assert!(parse_module("(MODULE)").is_err());
        assert!(parse_module("(modulee)").is_err());
        assert!(parse_module("").is_err());
        assert!(parse_module("()").is_err());
    }

    #[test]
    fn test_parse_module_with_trailing_content() {
        let result = parse_module("(module) extra");
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(remaining, " extra");
    }
}
