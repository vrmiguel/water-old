pub mod ast;
pub mod emitter;
pub mod leb128;
pub mod opcode;
pub mod parser;
pub mod small_string;

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
#[pymodule]
fn _water(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_module, m)?)?;
    m.add_function(wrap_pyfunction!(parse_function, m)?)?;
    Ok(())
}

#[cfg(feature = "extension-module")]
/// Parse a WebAssembly Text Format module from a string.
#[pyfunction]
fn parse_module(input: &str) -> PyResult<bool> {
    match parser::parse_module(input) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(feature = "extension-module")]
/// Parse a WebAssembly Text Format function from a string.
#[pyfunction]
fn parse_function(input: &str) -> PyResult<bool> {
    match parser::parse_function(input) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
