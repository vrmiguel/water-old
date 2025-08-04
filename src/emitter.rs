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

    /// Emit the given program to WebAssembly binary format.
    ///
    /// This function takes a parsed WebAssembly program and emits it as binary WASM format
    /// to the underlying writer. Currently, this implementation only emits the WASM magic
    /// number and version header, serving as a foundation for a complete WASM compiler.
    ///
    /// The WebAssembly binary format starts with:
    /// - Magic number: `\0asm` (0x00 0x61 0x73 0x6d)
    /// - Version: `1000` in little-endian (0x01 0x00 0x00 0x00)
    ///
    /// # Arguments
    ///
    /// * `_program` - The parsed WebAssembly program to emit (currently unused in this early implementation)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful emission, or an `io::Error` if writing fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut emitter = Emitter::new(Vec::new());
    /// let program = Program::default(); // Assuming a default program
    /// emitter.emit_program(program)?;
    /// ```
    pub fn emit_program(
        &mut self,
        _program: Program,
    ) -> io::Result<()> {
        self.emit_magic()?;
        self.emit_version()?;

        Ok(())
    }

    #[cfg(test)]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W> Emitter<std::io::Cursor<W>> {
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
