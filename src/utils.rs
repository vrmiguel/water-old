/// Calculates the alignment padding needed to align an offset to a given boundary.
///
/// This is useful when working with binary formats that require specific alignment
/// (e.g., WebAssembly memory alignment, struct padding).
///
/// # Arguments
///
/// * `offset` - The current offset/position
/// * `alignment` - The required alignment boundary (must be a power of 2)
///
/// # Returns
///
/// The number of padding bytes needed to reach the next aligned position.
///
/// # Examples
///
/// ```
/// # use water::calculate_padding;
/// // Aligning to 4-byte boundary
/// assert_eq!(calculate_padding(5, 4), 3);  // 5 + 3 = 8 (next multiple of 4)
/// assert_eq!(calculate_padding(8, 4), 0);  // already aligned
/// assert_eq!(calculate_padding(1, 4), 3);  // 1 + 3 = 4
///
/// // Aligning to 8-byte boundary
/// assert_eq!(calculate_padding(10, 8), 6); // 10 + 6 = 16 (next multiple of 8)
/// ```
pub fn calculate_padding(offset: usize, alignment: usize) -> usize {
    debug_assert!(alignment > 0 && (alignment & (alignment - 1)) == 0,
                  "alignment must be a power of 2");

    let mask = alignment - 1;
    let remainder = offset & mask;

    if remainder == 0 {
        0
    } else {
        alignment - remainder
    }
}

/// Converts a byte size to a human-readable string representation.
///
/// This utility formats byte counts into a more readable format using
/// appropriate unit prefixes (B, KB, MB, GB, TB).
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format
///
/// # Returns
///
/// A formatted string with the size and appropriate unit.
///
/// # Examples
///
/// ```
/// # use water::format_byte_size;
/// assert_eq!(format_byte_size(512), "512 B");
/// assert_eq!(format_byte_size(1024), "1.00 KB");
/// assert_eq!(format_byte_size(1536), "1.50 KB");
/// assert_eq!(format_byte_size(1048576), "1.00 MB");
/// assert_eq!(format_byte_size(1536000), "1.46 MB");
/// assert_eq!(format_byte_size(1073741824), "1.00 GB");
/// ```
pub fn format_byte_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_padding() {
        // Test 4-byte alignment
        assert_eq!(calculate_padding(0, 4), 0);
        assert_eq!(calculate_padding(1, 4), 3);
        assert_eq!(calculate_padding(2, 4), 2);
        assert_eq!(calculate_padding(3, 4), 1);
        assert_eq!(calculate_padding(4, 4), 0);
        assert_eq!(calculate_padding(5, 4), 3);

        // Test 8-byte alignment
        assert_eq!(calculate_padding(0, 8), 0);
        assert_eq!(calculate_padding(7, 8), 1);
        assert_eq!(calculate_padding(8, 8), 0);
        assert_eq!(calculate_padding(10, 8), 6);

        // Test 1-byte alignment (always 0)
        assert_eq!(calculate_padding(0, 1), 0);
        assert_eq!(calculate_padding(100, 1), 0);

        // Test 2-byte alignment
        assert_eq!(calculate_padding(1, 2), 1);
        assert_eq!(calculate_padding(2, 2), 0);
        assert_eq!(calculate_padding(3, 2), 1);
    }

    #[test]
    fn test_format_byte_size() {
        // Bytes
        assert_eq!(format_byte_size(0), "0 B");
        assert_eq!(format_byte_size(1), "1 B");
        assert_eq!(format_byte_size(512), "512 B");
        assert_eq!(format_byte_size(1023), "1023 B");

        // Kilobytes
        assert_eq!(format_byte_size(1024), "1.00 KB");
        assert_eq!(format_byte_size(1536), "1.50 KB");
        assert_eq!(format_byte_size(2048), "2.00 KB");

        // Megabytes
        assert_eq!(format_byte_size(1048576), "1.00 MB");
        assert_eq!(format_byte_size(1572864), "1.50 MB");

        // Gigabytes
        assert_eq!(format_byte_size(1073741824), "1.00 GB");
        assert_eq!(format_byte_size(1610612736), "1.50 GB");

        // Terabytes
        assert_eq!(format_byte_size(1099511627776), "1.00 TB");
    }
}
