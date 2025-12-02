pub mod ast;
pub mod emitter;
pub mod leb128;
pub mod opcode;
pub mod parser;
pub mod small_string;

pub use leb128::leb128_encoded_size;
