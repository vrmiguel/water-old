/// Version information for the water compiler
pub const MAJOR: u32 = 0;
pub const MINOR: u32 = 1;
pub const PATCH: u32 = 0;

/// Returns the full version string
pub fn version() -> String {
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}

/// Returns the version with a prefix (e.g., "water v0.1.0")
pub fn version_string(name: &str) -> String {
    format!("{} v{}", name, version())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_format() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn test_version_string() {
        assert_eq!(version_string("water"), "water v0.1.0");
    }
}
