//! Error types for the water crate.

use std::fmt;

/// Error type for the water crate.
#[derive(Debug)]
pub enum Error {
    /// Invalid operation error for attempting an operation that is not valid for a specific type.
    InvalidOperation {
        /// Description of the error.
        message: String,
    },
    /// Parser error for nom parsing errors.
    Parser(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidOperation { message } => {
                write!(f, "Invalid operation: {}", message)
            }
            Error::Parser(msg) => write!(f, "Parser error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for the water crate.
pub type Result<T> = std::result::Result<T, Error>;