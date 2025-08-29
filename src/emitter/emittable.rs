use std::io::{self};

/// Core trait for emitting WebAssembly elements to binary format.
/// 
/// This trait defines the interface for converting high-level WebAssembly
/// constructs (like constants, instructions, etc.) into the binary WebAssembly
/// format. Each implementor knows how to serialize a specific type `T`.
/// 
/// # Type Parameters
/// * `T` - The type of element that can be emitted
/// 
/// # Purpose
/// This trait enables a modular approach to WebAssembly binary generation,
/// where different types can implement their own emission logic while
/// maintaining a consistent interface.
pub trait Emittable<T> {
    /// Emits a WebAssembly element to binary format.
    /// 
    /// This method converts the given element into its WebAssembly binary
    /// representation and writes it to the underlying writer.
    /// 
    /// # Arguments
    /// * `element` - The element to emit
    /// 
    /// # Returns
    /// * `io::Result<usize>` - Number of bytes written or an IO error
    fn emit_element(&mut self, element: T) -> io::Result<usize>;
}
