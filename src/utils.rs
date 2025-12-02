//! General utility functions for the water compiler.
//!
//! This module contains utility functions that are not
//! specific to parsing or code generation.

/// Checks if a given number is a power of two.
///
/// This is useful for alignment checks and memory
/// calculations in WebAssembly, where many values must
/// be powers of two.
///
/// # Examples
///
/// ```
/// use water::utils::is_power_of_two;
///
/// assert_eq!(is_power_of_two(1), true);
/// assert_eq!(is_power_of_two(2), true);
/// assert_eq!(is_power_of_two(4), true);
/// assert_eq!(is_power_of_two(8), true);
/// assert_eq!(is_power_of_two(16), true);
/// assert_eq!(is_power_of_two(0), false);
/// assert_eq!(is_power_of_two(3), false);
/// assert_eq!(is_power_of_two(5), false);
/// assert_eq!(is_power_of_two(15), false);
/// ```
pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

/// Calculates the next aligned value for a given
/// alignment requirement.
///
/// This is commonly used in WebAssembly memory layout
/// calculations where values need to be aligned to
/// specific byte boundaries.
///
/// # Arguments
///
/// * `value` - The current value to align
/// * `alignment` - The alignment requirement (must be a
///   power of two)
///
/// # Panics
///
/// Panics if `alignment` is not a power of two.
///
/// # Examples
///
/// ```
/// use water::utils::align_to;
///
/// assert_eq!(align_to(0, 4), 0);
/// assert_eq!(align_to(1, 4), 4);
/// assert_eq!(align_to(3, 4), 4);
/// assert_eq!(align_to(4, 4), 4);
/// assert_eq!(align_to(5, 4), 8);
/// assert_eq!(align_to(7, 8), 8);
/// assert_eq!(align_to(10, 16), 16);
/// ```
pub fn align_to(value: u32, alignment: u32) -> u32 {
    assert!(
        is_power_of_two(alignment),
        "Alignment must be a power of two"
    );

    if value == 0 {
        return 0;
    }

    let mask = alignment - 1;
    (value + mask) & !mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        // Powers of two
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(8));
        assert!(is_power_of_two(16));
        assert!(is_power_of_two(32));
        assert!(is_power_of_two(64));
        assert!(is_power_of_two(128));
        assert!(is_power_of_two(256));
        assert!(is_power_of_two(1024));

        // Not powers of two
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(7));
        assert!(!is_power_of_two(9));
        assert!(!is_power_of_two(15));
        assert!(!is_power_of_two(100));
    }

    #[test]
    fn test_align_to() {
        // Alignment to 4
        assert_eq!(align_to(0, 4), 0);
        assert_eq!(align_to(1, 4), 4);
        assert_eq!(align_to(2, 4), 4);
        assert_eq!(align_to(3, 4), 4);
        assert_eq!(align_to(4, 4), 4);
        assert_eq!(align_to(5, 4), 8);

        // Alignment to 8
        assert_eq!(align_to(7, 8), 8);
        assert_eq!(align_to(8, 8), 8);
        assert_eq!(align_to(9, 8), 16);

        // Alignment to 16
        assert_eq!(align_to(10, 16), 16);
        assert_eq!(align_to(16, 16), 16);
        assert_eq!(align_to(17, 16), 32);
    }

    #[test]
    #[should_panic(
        expected = "Alignment must be a power of two"
    )]
    fn test_align_to_invalid_alignment() {
        align_to(10, 3);
    }
}
