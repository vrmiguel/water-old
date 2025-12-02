//! Utility functions for the water compiler

/// Validates if a given identifier is a valid WebAssembly identifier
///
/// WebAssembly identifiers can contain alphanumeric characters, underscores,
/// and certain special characters
///
/// # Examples
///
/// ```
/// use water::is_valid_identifier;
///
/// assert!(is_valid_identifier("$myFunc"));
/// assert!(is_valid_identifier("_start"));
/// assert!(!is_valid_identifier("123invalid"));
/// ```
pub fn is_valid_identifier(identifier: &str) -> bool {
    if identifier.is_empty() {
        return false;
    }

    let first_char = identifier.chars().next().unwrap();

    // First character must be a letter, underscore, or dollar sign
    if !first_char.is_alphabetic() && first_char != '_' && first_char != '$' {
        return false;
    }

    // Remaining characters can be alphanumeric, underscore, or dollar sign
    identifier.chars().all(|c| {
        c.is_alphanumeric() || c == '_' || c == '$' || c == '.' || c == ':'
    })
}

/// Aligns a value to the next power of two
///
/// This is useful for memory alignment operations in WebAssembly
///
/// # Examples
///
/// ```
/// use water::align_to_power_of_two;
///
/// assert_eq!(align_to_power_of_two(5), 8);
/// assert_eq!(align_to_power_of_two(8), 8);
/// assert_eq!(align_to_power_of_two(9), 16);
/// ```
pub const fn align_to_power_of_two(value: usize) -> usize {
    if value == 0 {
        return 0;
    }

    if value.is_power_of_two() {
        return value;
    }

    value.next_power_of_two()
}

/// Calculates the byte size needed to encode a value in LEB128 format
///
/// This is useful for determining buffer sizes when emitting LEB128-encoded values
///
/// # Examples
///
/// ```
/// use water::calculate_leb128_size;
///
/// assert_eq!(calculate_leb128_size(0), 1);
/// assert_eq!(calculate_leb128_size(127), 1);
/// assert_eq!(calculate_leb128_size(128), 2);
/// assert_eq!(calculate_leb128_size(16383), 2);
/// assert_eq!(calculate_leb128_size(16384), 3);
/// ```
pub fn calculate_leb128_size(mut value: u64) -> usize {
    let mut size = 0;

    loop {
        value >>= 7;
        size += 1;

        if value == 0 {
            break;
        }
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier() {
        // Valid identifiers
        assert!(is_valid_identifier("myFunc"));
        assert!(is_valid_identifier("_start"));
        assert!(is_valid_identifier("$local0"));
        assert!(is_valid_identifier("func_name"));
        assert!(is_valid_identifier("my.namespace:func"));

        // Invalid identifiers
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier("-invalid"));
        assert!(!is_valid_identifier("@invalid"));
    }

    #[test]
    fn test_align_to_power_of_two() {
        assert_eq!(align_to_power_of_two(0), 0);
        assert_eq!(align_to_power_of_two(1), 1);
        assert_eq!(align_to_power_of_two(2), 2);
        assert_eq!(align_to_power_of_two(3), 4);
        assert_eq!(align_to_power_of_two(5), 8);
        assert_eq!(align_to_power_of_two(8), 8);
        assert_eq!(align_to_power_of_two(9), 16);
        assert_eq!(align_to_power_of_two(15), 16);
        assert_eq!(align_to_power_of_two(16), 16);
        assert_eq!(align_to_power_of_two(17), 32);
    }

    #[test]
    fn test_calculate_leb128_size() {
        // Single byte values (0-127)
        assert_eq!(calculate_leb128_size(0), 1);
        assert_eq!(calculate_leb128_size(1), 1);
        assert_eq!(calculate_leb128_size(127), 1);

        // Two byte values (128-16383)
        assert_eq!(calculate_leb128_size(128), 2);
        assert_eq!(calculate_leb128_size(255), 2);
        assert_eq!(calculate_leb128_size(16383), 2);

        // Three byte values (16384-2097151)
        assert_eq!(calculate_leb128_size(16384), 3);
        assert_eq!(calculate_leb128_size(2097151), 3);

        // Four byte values
        assert_eq!(calculate_leb128_size(2097152), 4);
    }
}
