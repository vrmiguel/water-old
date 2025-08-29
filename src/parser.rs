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

/// The result type used throughout the WebAssembly Text Format parser.
///
/// This type alias provides consistent error handling across all parsing functions
/// by using nom's `VerboseError` to give detailed context about parse failures.
/// The verbose error type helps identify exactly where parsing failed and why.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the input string slice
/// * `T` - The type of the successfully parsed value
///
/// # Examples
///
/// ```
/// use water::parser::{IResult, parse_module};
/// use water::ast::Module;
///
/// fn example_parser(input: &str) -> IResult<Module> {
///     parse_module(input)
/// }
/// ```
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;
