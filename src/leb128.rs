//! Emitters for the Little Endian Base 128 variable length
//! integer encoding, which is how WebAssembly stores integer
//! literals.
//!
//! The code in this file is heavily based in the [leb128](https://github.com/gimli-rs/leb128) crate by gimli-rs.

use std::{io, io::Write, ops::Not};

use crate::emitter::{Emittable, Emitter};

const CONTINUATION_BIT: u64 = 1 << 7;

/// LEB128 encoder for signed integers
pub struct SignedLeb128 {
    value: i64,
}

impl From<i64> for SignedLeb128 {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl<W: Write> Emittable<SignedLeb128> for Emitter<W> {
    fn emit_element(
        &mut self,
        element: SignedLeb128,
    ) -> io::Result<usize> {
        let mut bytes_written = 0;
        let SignedLeb128 { mut value } = element;
        let mut is_done = false;

        while is_done.not() {
            // Backup the current value
            let bkp = value;

            value >>= 6;

            is_done = matches!(value, 0 | -1);
            let byte = if is_done {
                bkp & !(CONTINUATION_BIT as i64)
            } else {
                // Remove the sign bit
                value >>= 1;

                // More bytes to come, so set the continuation
                // bit.
                bkp | (CONTINUATION_BIT as i64)
            } as u8;

            self.emit_byte(byte)?;
            bytes_written += 1;
        }

        Ok(bytes_written)
    }
}

// impl Emittable for SignedLeb128 {
//     fn emit_to<W: Write>(
//         &self,
//         writer: &mut W,
//     ) -> io::Result<usize> {
//         let mut bytes_written = 0;
//         let mut value = self.value;
//         let mut is_done = false;

//         while is_done.not() {
//             // Backup the current value
//             let bkp = value;

//             value >>= 6;

//             is_done = matches!(value, 0 | -1);
//             let byte = if is_done {
//                 bkp & !(CONTINUATION_BIT as i64)
//             } else {
//                 // Remove the sign bit
//                 value >>= 1;

//                 // More bytes to come, so set the continuation
//                 // bit.
//                 bkp | (CONTINUATION_BIT as i64)
//             } as u8;

//             writer.write_all(&[byte])?;
//             bytes_written += 1;
//         }

//         Ok(bytes_written)
//     }
// }

/// LEB128 encoder for unsigned integers
pub struct UnsignedLeb128 {
    value: u64,
}

impl From<u64> for UnsignedLeb128 {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl<W: Write> Emittable<UnsignedLeb128> for Emitter<W> {
    /// Encodes and emits an unsigned integer using the LEB128 variable-length encoding.
    ///
    /// This implementation follows the LEB128 specification for unsigned integers:
    /// - Values are encoded in 7-bit chunks
    /// - The high bit (continuation bit) of each byte indicates if more bytes follow
    /// - The continuation bit is set for all bytes except the last one
    /// - The value 0 is encoded as a single byte 0x00
    ///
    /// # Arguments
    ///
    /// * `self` - The emitter to write the encoded bytes to
    /// * `element` - The UnsignedLeb128 wrapper containing the value to encode
    ///
    /// # Returns
    ///
    /// The number of bytes written to encode the value
    ///
    /// # Errors
    ///
    /// Returns any I/O errors encountered during writing
    fn emit_element(
        &mut self,
        element: UnsignedLeb128,
    ) -> io::Result<usize> {
        let mut bytes_written = 0;
        let UnsignedLeb128 { mut value } = element;

        if value == 0 {
            self.emit_byte(0)?;

            return Ok(1);
        }

        while value != 0 {
            let mut byte = low_bits(value);
            value >>= 7;
            if value != 0 {
                // More bytes to come, so set the continuation
                // bit.
                byte |= CONTINUATION_BIT as u8;
            }

            bytes_written += 1;
            self.emit_byte(byte)?;
        }

        Ok(bytes_written)
    }
}

/// Extracts the lowest 7 bits from a u64 value for LEB128 encoding.
///
/// This function:
/// 1. Masks the input value to get only the lower 8 bits
/// 2. Clears the highest bit (continuation bit) as defined by LEB128 spec
/// 3. Returns the resulting 7-bit value as a u8
///
/// # Arguments
///
/// * `value` - The u64 value to extract bits from
///
/// # Returns
///
/// A u8 containing the lowest 7 bits (with the continuation bit cleared)
fn low_bits(value: u64) -> u8 {
    // This mask has all the lower 8 bits set
    const MASK: u64 = 0xFF;
    let lower_eight_bits = value & MASK;

    (lower_eight_bits & !CONTINUATION_BIT) as u8
}

#[cfg(test)]
mod tests {
    use crate::{
        emitter::{Emittable, Emitter},
        leb128::{SignedLeb128, UnsignedLeb128},
    };

    #[test]
    fn encodes_signed_leb_128() {
        let to_encode = [
            i64::MIN,
            0,
            36,
            128,
            156,
            256,
            512,
            50603,
            -85092,
            -9999999,
            -20312391039,
            i64::MAX,
        ];

        let expected_encoding: &[&[u8]] = &[
            &[128, 128, 128, 128, 128, 128, 128, 128, 128, 127],
            &[0],
            &[36],
            &[128, 1],
            &[156, 1],
            &[128, 2],
            &[128, 4],
            &[171, 139, 3],
            &[156, 231, 122],
            &[129, 211, 157, 123],
            &[129, 133, 166, 170, 180, 127],
            &[255, 255, 255, 255, 255, 255, 255, 255, 255, 0],
        ];

        for (value_to_encode, expected) in
            to_encode.into_iter().zip(expected_encoding)
        {
            let encoder = SignedLeb128::from(value_to_encode);
            let mut emitter = Emitter::new(Vec::new());

            emitter.emit_element(encoder).unwrap();
            // encoder.emit_to(&mut bytes).unwrap();
            assert_eq!(emitter.into_inner(), *expected);
        }
    }

    #[test]
    fn encodes_unsigned_leb_128() {
        let to_encode = [
            0_u64,
            15,
            97,
            128,
            225,
            256,
            512,
            900,
            9203,
            242962,
            u64::MAX,
        ];

        let expected_encoding: &[&[u8]] = &[
            &[0],
            &[15],
            &[97],
            &[128, 1],
            &[225, 1],
            &[128, 2],
            &[128, 4],
            &[132, 7],
            &[243, 71],
            &[146, 234, 14],
            &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1],
        ];

        for (value_to_encode, expected) in
            to_encode.into_iter().zip(expected_encoding)
        {
            let encoder = UnsignedLeb128::from(value_to_encode);
            let mut emitter = Emitter::new(Vec::new());

            emitter.emit_element(encoder).unwrap();
            // encoder.emit_to(&mut bytes).unwrap();
            assert_eq!(emitter.into_inner(), *expected);
        }
    }
}
