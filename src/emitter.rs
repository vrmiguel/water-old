use std::io::{self, Write};

mod arithmetic_operation;
mod constant;
pub mod emittable;
mod numerical_value;
mod unreachable;

pub use emittable::Emittable;

use crate::ast::Program;
use crate::calculate_padding;

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

    /// Emit padding bytes to align to the specified boundary.
    ///
    /// This is useful when emitting WebAssembly sections that require
    /// specific alignment (e.g., data segments, memory initialization).
    ///
    /// # Arguments
    ///
    /// * `current_offset` - The current position in the output
    /// * `alignment` - The required alignment boundary (must be a power of 2)
    ///
    /// # Returns
    ///
    /// The number of padding bytes written
    pub fn emit_padding(
        &mut self,
        current_offset: usize,
        alignment: usize,
    ) -> io::Result<usize> {
        let padding_needed =
            calculate_padding(current_offset, alignment);
        if padding_needed > 0 {
            let padding = vec![0u8; padding_needed];
            self.emit_bytes(&padding)?;
        }
        Ok(padding_needed)
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
    use super::{Emitter, MAGIC};

    #[test]
    fn assert_correct_magic() {
        assert_eq!(MAGIC, &[0x00, 0x61, 0x73, 0x6d])
    }

    #[test]
    fn test_emit_padding() {
        let mut emitter = Emitter::new(Vec::new());

        // Write 5 bytes
        emitter.emit_bytes(&[1, 2, 3, 4, 5]).unwrap();

        // Align to 8-byte boundary (should add 3 padding bytes)
        let padding_written =
            emitter.emit_padding(5, 8).unwrap();
        assert_eq!(padding_written, 3);

        let result = emitter.into_inner();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 0, 0, 0]);
    }

    #[test]
    fn test_emit_padding_already_aligned() {
        let mut emitter = Emitter::new(Vec::new());

        // Write 8 bytes (already aligned to 8-byte boundary)
        emitter.emit_bytes(&[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();

        // Should not add any padding
        let padding_written =
            emitter.emit_padding(8, 8).unwrap();
        assert_eq!(padding_written, 0);

        let result = emitter.into_inner();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
