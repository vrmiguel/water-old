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

    /// Creates a new WebAssembly emitter with the specified writer.
    ///
    /// This function initializes a new Emitter that will write WebAssembly 
    /// binary format data to the provided writer. The Emitter is responsible
    /// for encoding various WebAssembly structures into their binary representation.
    ///
    /// # Arguments
    ///
    /// * `writer` - Any type that implements the `Write` trait, which will receive
    ///   the encoded WebAssembly binary data. Common writers include `Vec<u8>`,
    ///   `File`, and other IO streams.
    ///
    /// # Returns
    ///
    /// A new Emitter instance that wraps the provided writer.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create an emitter that writes to a Vec<u8>
    /// let mut emitter = Emitter::new(Vec::new());
    ///
    /// // Create an emitter that writes to a file
    /// let file = std::fs::File::create("output.wasm").unwrap();
    /// let mut emitter = Emitter::new(file);
    /// ```
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Emit the given program to WASM
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
