//! Mathematical utility functions

/// Calculates the greatest common divisor (GCD) of two unsigned integers
/// using the Euclidean algorithm.
///
/// # Examples
///
/// ```
/// use water::math::gcd;
///
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(17, 19), 1);
/// assert_eq!(gcd(0, 5), 5);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_common_cases() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(13, 7), 1);
    }

    #[test]
    fn test_gcd_with_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_same_numbers() {
        assert_eq!(gcd(42, 42), 42);
        assert_eq!(gcd(1, 1), 1);
    }

    #[test]
    fn test_gcd_large_numbers() {
        assert_eq!(gcd(1071, 462), 21);
        assert_eq!(gcd(123456, 789012), 12);
    }
}
