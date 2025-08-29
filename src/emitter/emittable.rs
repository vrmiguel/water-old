use std::io::{self};

/// Trait for types that can emit WebAssembly binary data.
///
/// This trait provides a common interface for converting WebAssembly AST elements
/// into their binary representation. Implementations should write the binary data
/// to the underlying writer and return the number of bytes written.
///
/// # Examples
///
/// ```
/// use water::emitter::{Emitter, Emittable};
/// use water::leb128::UnsignedLeb128;
/// use std::io::Cursor;
///
/// let mut buffer = Vec::new();
/// let mut emitter = Emitter::new(&mut buffer);
/// let value = UnsignedLeb128::from(42u64);
///
/// let bytes_written = emitter.emit_element(value).unwrap();
/// assert!(bytes_written > 0);
/// ```
pub trait Emittable<T> {
    /// Emits a WebAssembly element to binary format.
    ///
    /// This method converts the given element to its WebAssembly binary representation
    /// and writes it to the underlying writer. The method returns the number of bytes
    /// written to the output.
    ///
    /// # Arguments
    ///
    /// * `element` - The WebAssembly AST element to emit
    ///
    /// # Returns
    ///
    /// Returns `Ok(usize)` with the number of bytes written on success,
    /// or `Err(io::Error)` if writing fails.
    ///
    /// # Errors
    ///
    /// This method can fail if the underlying writer encounters an I/O error
    /// during the write operation.
    fn emit_element(&mut self, element: T) -> io::Result<usize>;
}
