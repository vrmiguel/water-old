//! Bit manipulation utilities for working with binary data
//!
//! This module provides helper functions for common bit manipulation
//! operations used in WebAssembly binary encoding and processing.

/// Counts the number of set bits (1s) in a 32-bit unsigned integer
///
/// This implements the population count (popcount) operation, which is
/// useful for analyzing binary data and optimization checks.
///
/// # Examples
///
/// ```
/// use water::bit_utils::count_set_bits_u32;
///
/// assert_eq!(count_set_bits_u32(0), 0);
/// assert_eq!(count_set_bits_u32(0b1010), 2);
/// assert_eq!(count_set_bits_u32(0xFF), 8);
/// assert_eq!(count_set_bits_u32(u32::MAX), 32);
/// ```
pub fn count_set_bits_u32(mut value: u32) -> u32 {
    let mut count = 0;
    while value != 0 {
        count += value & 1;
        value >>= 1;
    }
    count
}

/// Extracts a range of bits from a 64-bit unsigned integer
///
/// Returns the bits from position `start` to `end` (inclusive) as a new integer,
/// shifted to the least significant position. This is useful for decoding
/// packed binary formats.
///
/// # Panics
///
/// Panics if `start > end` or if `end >= 64`
///
/// # Examples
///
/// ```
/// use water::bit_utils::extract_bit_range;
///
/// // Extract bits 4-7 from 0b11110000 (240)
/// assert_eq!(extract_bit_range(240, 4, 7), 0b1111);
///
/// // Extract bits 0-3 from 0xFF
/// assert_eq!(extract_bit_range(0xFF, 0, 3), 0b1111);
/// ```
pub fn extract_bit_range(value: u64, start: u8, end: u8) -> u64 {
    assert!(start <= end, "start must be <= end");
    assert!(end < 64, "end must be < 64");

    let num_bits = end - start + 1;
    let mask = if num_bits == 64 {
        u64::MAX
    } else {
        (1u64 << num_bits) - 1
    };

    (value >> start) & mask
}

/// Reverses the byte order of a 32-bit unsigned integer
///
/// This is useful for converting between little-endian and big-endian
/// representations when working with binary formats.
///
/// # Examples
///
/// ```
/// use water::bit_utils::reverse_bytes_u32;
///
/// assert_eq!(reverse_bytes_u32(0x12345678), 0x78563412);
/// assert_eq!(reverse_bytes_u32(0xFF000000), 0x000000FF);
/// ```
pub fn reverse_bytes_u32(value: u32) -> u32 {
    value.swap_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_set_bits_u32() {
        assert_eq!(count_set_bits_u32(0), 0);
        assert_eq!(count_set_bits_u32(1), 1);
        assert_eq!(count_set_bits_u32(0b1010), 2);
        assert_eq!(count_set_bits_u32(0b1111), 4);
        assert_eq!(count_set_bits_u32(0xFF), 8);
        assert_eq!(count_set_bits_u32(0xFFFF), 16);
        assert_eq!(count_set_bits_u32(u32::MAX), 32);
    }

    #[test]
    fn test_extract_bit_range() {
        // Extract middle bits
        assert_eq!(extract_bit_range(240, 4, 7), 0b1111);

        // Extract lower bits
        assert_eq!(extract_bit_range(0xFF, 0, 3), 0b1111);

        // Extract single bit
        assert_eq!(extract_bit_range(0b1000, 3, 3), 1);

        // Extract all bits
        assert_eq!(extract_bit_range(0xFF, 0, 7), 0xFF);

        // Complex pattern
        assert_eq!(
            extract_bit_range(0b1010_1100_1111_0000, 4, 11),
            0b1100_1111
        );
    }

    #[test]
    #[should_panic(expected = "start must be <= end")]
    fn test_extract_bit_range_invalid_range() {
        extract_bit_range(100, 10, 5);
    }

    #[test]
    #[should_panic(expected = "end must be < 64")]
    fn test_extract_bit_range_out_of_bounds() {
        extract_bit_range(100, 0, 64);
    }

    #[test]
    fn test_reverse_bytes_u32() {
        assert_eq!(reverse_bytes_u32(0x12345678), 0x78563412);
        assert_eq!(reverse_bytes_u32(0xFF000000), 0x000000FF);
        assert_eq!(reverse_bytes_u32(0x00FF0000), 0x0000FF00);
        assert_eq!(reverse_bytes_u32(0), 0);
        assert_eq!(reverse_bytes_u32(u32::MAX), u32::MAX);
    }
}
