pub mod ast;
pub mod emitter;
pub mod leb128;
pub mod opcode;
pub mod parser;
pub mod small_string;
pub mod utils;

// Re-export utility functions for convenience
pub use utils::{align_to_power_of_two, calculate_leb128_size, is_valid_identifier};
