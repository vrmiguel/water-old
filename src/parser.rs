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
    function::{
        parse_function, parse_export, parse_parameter,
        parse_local,
    },
    import::parse_function_import,
    instruction::{
        parse_instruction, parse_opcode, parse_const,
        parse_call, parse_variable_instruction,
        parse_unreachable,
    },
    module::parse_module,
    utils::{
        parse_string, parse_identifier, parse_type,
        parse_numerical_type, parse_index,
        parse_parenthesis_enclosed,
    },
};

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;
