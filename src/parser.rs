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
    function::*, import::*, instruction::*, module::*,
};
pub use self::utils::{
    parse_identifier, parse_index, parse_numerical_type, 
    parse_parenthesis_enclosed, parse_string, parse_type,
};

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;
