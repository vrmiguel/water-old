//! A parser for WebAssembly Text Format.
//!
//! Functions are mostly all public as to allow doc-tests.

mod function;
mod import;
mod instruction;
mod module;
mod utils;

use nom::error::VerboseError;

pub use self::{
    function::*, import::*, instruction::*, module::*, utils::*,
};

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;

/// Converts a `nom` verbose error into a human-readable string.
///
/// Useful for CLI/front-end error reporting.
pub fn stringify_error<'a>(
    input: &'a str,
    error: nom::Err<VerboseError<&'a str>>,
) -> String {
    match error {
        nom::Err::Incomplete(needed) => {
            format!("incomplete input: {needed:?}")
        }
        nom::Err::Error(error) | nom::Err::Failure(error) => {
            nom::error::convert_error(input, error)
        }
    }
}
