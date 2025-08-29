use std::io::{self, Write};

mod arithmetic_operation;
mod constant;
pub mod emittable;
mod numerical_value;
mod unreachable;

pub use emittable::Emittable;

use crate::ast::Program;

const MAGIC: &[u8] = b"\0asm";
const VERSION: &[u8] = b"1000";

pub struct Emitter<W> {
    /// Where this Emitter will write to
    writer: W,
}

impl<W: Write> Emitter<W> {
    /// Emit a single byte to the writer
    pub fn emit_byte(&mut self, byte: u8) -> io::Result<usize> {
        self.emit_bytes(&[byte]).map(|()| 1)
    }

    /// Emit a sequence of bytes to the writer
    pub fn emit_bytes(
        &mut self,
        bytes: &[u8],
    ) -> io::Result<()> {
        self.writer.write_all(bytes)
    }

    /// Emits the WASM magic constant
    fn emit_magic(&mut self) -> io::Result<()> {
        self.emit_bytes(MAGIC)
    }

    /// Emits the WASM version tag
    fn emit_version(&mut self) -> io::Result<()> {
        self.emit_bytes(VERSION)
    }

    /// Builds a new emitter with the given writer
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Emits a WebAssembly program to the underlying writer.
    ///
    /// This method writes the WebAssembly binary format header (magic number
    /// and version) followed by the program's sections. Currently only emits
    /// the header as program emission is not yet fully implemented.
    ///
    /// # Arguments
    ///
    /// * `_program` - The WebAssembly program AST to emit (currently unused)
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if writing to the underlying writer fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use water::emitter::Emitter;
    /// use water::ast::Program;
    ///
    /// let mut buffer = Vec::new();
    /// let mut emitter = Emitter::new(&mut buffer);
    /// let program = Program { functions: vec![] };
    /// emitter.emit_program(program).unwrap();
    /// ```
    pub fn emit_program(
        &mut self,
        _program: Program,
    ) -> io::Result<()> {
        self.emit_magic()?;
        self.emit_version()?;

        Ok(())
    }

    /// Consumes the emitter and returns the underlying writer.
    ///
    /// This method is only available in test builds and is primarily used
    /// for testing to extract the written data from the emitter.
    ///
    /// # Returns
    ///
    /// The underlying writer that was passed to `new()`
    #[cfg(test)]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W> Emitter<std::io::Cursor<W>> {
    /// Creates a new emitter with a cursor-wrapped writer.
    ///
    /// This method is only available in test builds and wraps the given writer
    /// in a `std::io::Cursor` for convenient testing scenarios where you want
    /// to track position or seek within the written data.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to wrap in a cursor
    ///
    /// # Returns
    ///
    /// A new `Emitter` with the cursor-wrapped writer
    #[cfg(test)]
    pub fn new_cursored(writer: W) -> Self {
        use std::io::Cursor;

        Self {
            writer: Cursor::new(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MAGIC;

    #[test]
    fn assert_correct_magic() {
        assert_eq!(MAGIC, &[0x00, 0x61, 0x73, 0x6d])
    }
}
