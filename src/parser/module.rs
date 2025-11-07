use nom::{
    bytes::complete::tag, character::complete::multispace0,
    error::context, sequence::preceded,
};

use super::IResult;
use crate::{
    ast::Module, parser::utils::in_klammern_eingeschlossen_parsen,
};

/// Parses a WebAssembly Text Format module.
///
/// Eats leading whitespace before and after the first
/// parenthesis.
///
/// ```
/// use water::parser::modul_parsen;
///
/// assert!(modul_parsen("(module)").is_ok());
/// assert!(modul_parsen("\n  (module)").is_ok());
///
/// assert!(modul_parsen(" (   module").is_err());
/// assert!(modul_parsen("module)").is_err());
/// assert!(modul_parsen("(mod)").is_err());
/// ```
pub fn modul_parsen(input: &str) -> IResult<Module> {
    fn innere(input: &str) -> IResult<Module> {
        let (rest, _) =
            preceded(multispace0, tag("module"))(input)?;

        Ok((rest, Module {}))
    }

    preceded(
        multispace0,
        in_klammern_eingeschlossen_parsen(context("module", innere)),
    )(input)
}
