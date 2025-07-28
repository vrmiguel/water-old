//! Water WebAssembly library
//!
//! This library provides functionality for parsing and emitting WebAssembly text format.

/// Abstract syntax tree definitions for WebAssembly
pub mod ast;
/// WebAssembly bytecode emission utilities
pub mod emitter;
/// LEB128 encoding and decoding utilities
pub mod leb128;
/// WebAssembly opcode definitions and conversions
pub mod opcode;
/// WebAssembly text format parsing utilities
pub mod parser;
/// Memory-efficient string implementation for small strings
pub mod small_string;
