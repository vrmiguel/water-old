//! Type conversion utilities for numeric representations
//!
//! This module provides helper functions for converting between different
//! numeric representations, particularly useful when working with WebAssembly's
//! type system and binary encoding.

use std::convert::TryFrom;

/// Converts an f32 to its IEEE 754 binary representation as u32
///
/// This function returns the raw binary representation of a 32-bit floating
/// point number, which is useful for encoding floats in WebAssembly binary format.
///
/// # Examples
///
/// ```
/// use water::type_utils::f32_to_bits;
///
/// assert_eq!(f32_to_bits(1.0), 0x3F800000);
/// assert_eq!(f32_to_bits(0.0), 0x00000000);
/// assert_eq!(f32_to_bits(-0.0), 0x80000000);
/// ```
pub fn f32_to_bits(value: f32) -> u32 {
    value.to_bits()
}

/// Converts an f64 to its IEEE 754 binary representation as u64
///
/// This function returns the raw binary representation of a 64-bit floating
/// point number, which is useful for encoding doubles in WebAssembly binary format.
///
/// # Examples
///
/// ```
/// use water::type_utils::f64_to_bits;
///
/// assert_eq!(f64_to_bits(1.0), 0x3FF0000000000000);
/// assert_eq!(f64_to_bits(0.0), 0x0000000000000000);
/// ```
pub fn f64_to_bits(value: f64) -> u64 {
    value.to_bits()
}

/// Safely converts an i64 to i32, returning None if the value doesn't fit
///
/// WebAssembly has both i32 and i64 types, and sometimes we need to safely
/// downcast from i64 to i32. This function returns None if the value is
/// outside the i32 range.
///
/// # Examples
///
/// ```
/// use water::type_utils::i64_to_i32_checked;
///
/// assert_eq!(i64_to_i32_checked(100), Some(100));
/// assert_eq!(i64_to_i32_checked(i32::MAX as i64), Some(i32::MAX));
/// assert_eq!(i64_to_i32_checked(i32::MIN as i64), Some(i32::MIN));
/// assert_eq!(i64_to_i32_checked(i32::MAX as i64 + 1), None);
/// assert_eq!(i64_to_i32_checked(i32::MIN as i64 - 1), None);
/// ```
pub fn i64_to_i32_checked(value: i64) -> Option<i32> {
    i32::try_from(value).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_to_bits() {
        assert_eq!(f32_to_bits(1.0), 0x3F800000);
        assert_eq!(f32_to_bits(0.0), 0x00000000);
        assert_eq!(f32_to_bits(-0.0), 0x80000000);
        assert_eq!(f32_to_bits(2.0), 0x40000000);
        assert_eq!(f32_to_bits(-1.0), 0xBF800000);

        // Test special values
        assert_eq!(f32_to_bits(f32::INFINITY), 0x7F800000);
        assert_eq!(f32_to_bits(f32::NEG_INFINITY), 0xFF800000);

        // NaN can have multiple representations, just verify it's in the NaN range
        let nan_bits = f32_to_bits(f32::NAN);
        assert!((nan_bits & 0x7F800000) == 0x7F800000); // Exponent is all 1s
        assert!((nan_bits & 0x007FFFFF) != 0); // Mantissa is non-zero
    }

    #[test]
    fn test_f64_to_bits() {
        assert_eq!(f64_to_bits(1.0), 0x3FF0000000000000);
        assert_eq!(f64_to_bits(0.0), 0x0000000000000000);
        assert_eq!(f64_to_bits(-0.0), 0x8000000000000000);
        assert_eq!(f64_to_bits(2.0), 0x4000000000000000);
        assert_eq!(f64_to_bits(-1.0), 0xBFF0000000000000);

        // Test special values
        assert_eq!(f64_to_bits(f64::INFINITY), 0x7FF0000000000000);
        assert_eq!(f64_to_bits(f64::NEG_INFINITY), 0xFFF0000000000000);
    }

    #[test]
    fn test_i64_to_i32_checked() {
        // Values that fit
        assert_eq!(i64_to_i32_checked(0), Some(0));
        assert_eq!(i64_to_i32_checked(100), Some(100));
        assert_eq!(i64_to_i32_checked(-100), Some(-100));
        assert_eq!(i64_to_i32_checked(i32::MAX as i64), Some(i32::MAX));
        assert_eq!(i64_to_i32_checked(i32::MIN as i64), Some(i32::MIN));

        // Values that don't fit
        assert_eq!(i64_to_i32_checked(i32::MAX as i64 + 1), None);
        assert_eq!(i64_to_i32_checked(i32::MIN as i64 - 1), None);
        assert_eq!(i64_to_i32_checked(i64::MAX), None);
        assert_eq!(i64_to_i32_checked(i64::MIN), None);
        assert_eq!(i64_to_i32_checked(10_000_000_000), None);
        assert_eq!(i64_to_i32_checked(-10_000_000_000), None);
    }

    #[test]
    fn test_roundtrip_f32() {
        // Test that we can roundtrip through bits
        let test_values = [0.0, 1.0, -1.0, 3.14159, -2.71828, 1e10, 1e-10];

        for value in test_values {
            let bits = f32_to_bits(value);
            let recovered = f32::from_bits(bits);
            assert_eq!(value, recovered);
        }
    }

    #[test]
    fn test_roundtrip_f64() {
        // Test that we can roundtrip through bits
        let test_values = [
            0.0,
            1.0,
            -1.0,
            3.141592653589793,
            -2.718281828459045,
            1e100,
            1e-100,
        ];

        for value in test_values {
            let bits = f64_to_bits(value);
            let recovered = f64::from_bits(bits);
            assert_eq!(value, recovered);
        }
    }
}
