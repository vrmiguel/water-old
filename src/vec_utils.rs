//! Vector manipulation utilities
//!
//! This module provides helper functions for working with vectors,
//! particularly useful for binary data processing and buffer management.

/// Pads a vector to a specific length with a given value
///
/// If the vector is shorter than the target length, appends the pad value
/// until the target length is reached. If the vector is already at or beyond
/// the target length, no modification is made.
///
/// # Examples
///
/// ```
/// use water::vec_utils::pad_to_length;
///
/// let mut vec = vec![1, 2, 3];
/// pad_to_length(&mut vec, 6, 0);
/// assert_eq!(vec, vec![1, 2, 3, 0, 0, 0]);
///
/// let mut vec2 = vec![1, 2, 3, 4, 5];
/// pad_to_length(&mut vec2, 3, 0);
/// assert_eq!(vec2, vec![1, 2, 3, 4, 5]); // No change
/// ```
pub fn pad_to_length<T: Clone>(vec: &mut Vec<T>, target_len: usize, pad_value: T) {
    if vec.len() < target_len {
        vec.resize(target_len, pad_value);
    }
}

/// Splits a vector into chunks of a specified size
///
/// Returns a vector of vectors, where each inner vector contains at most
/// `chunk_size` elements. The last chunk may contain fewer elements if
/// the original vector's length is not evenly divisible by `chunk_size`.
///
/// # Panics
///
/// Panics if `chunk_size` is 0.
///
/// # Examples
///
/// ```
/// use water::vec_utils::chunk_vec;
///
/// let vec = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunks = chunk_vec(&vec, 3);
/// assert_eq!(chunks, vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7]
/// ]);
///
/// let empty: Vec<u8> = vec![];
/// assert_eq!(chunk_vec(&empty, 5), Vec::<Vec<u8>>::new());
/// ```
pub fn chunk_vec<T: Clone>(vec: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    assert!(chunk_size > 0, "chunk_size must be greater than 0");

    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Aligns a vector's length to a multiple of the given alignment
///
/// Pads the vector with the specified value until its length is a multiple
/// of `alignment`. This is useful for ensuring data structures meet
/// alignment requirements in binary formats.
///
/// # Panics
///
/// Panics if `alignment` is 0.
///
/// # Examples
///
/// ```
/// use water::vec_utils::align_to_multiple;
///
/// let mut vec = vec![1, 2, 3];
/// align_to_multiple(&mut vec, 4, 0);
/// assert_eq!(vec, vec![1, 2, 3, 0]);
///
/// let mut vec2 = vec![1, 2, 3, 4];
/// align_to_multiple(&mut vec2, 4, 0);
/// assert_eq!(vec2, vec![1, 2, 3, 4]); // Already aligned
///
/// let mut vec3 = vec![1, 2, 3, 4, 5];
/// align_to_multiple(&mut vec3, 4, 0xFF);
/// assert_eq!(vec3, vec![1, 2, 3, 4, 5, 0xFF, 0xFF, 0xFF]);
/// ```
pub fn align_to_multiple<T: Clone>(vec: &mut Vec<T>, alignment: usize, pad_value: T) {
    assert!(alignment > 0, "alignment must be greater than 0");

    let remainder = vec.len() % alignment;
    if remainder != 0 {
        let padding_needed = alignment - remainder;
        vec.resize(vec.len() + padding_needed, pad_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_to_length() {
        let mut vec = vec![1, 2, 3];
        pad_to_length(&mut vec, 6, 0);
        assert_eq!(vec, vec![1, 2, 3, 0, 0, 0]);

        let mut vec2 = vec![1, 2, 3, 4, 5];
        pad_to_length(&mut vec2, 3, 0);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);

        let mut vec3: Vec<u8> = vec![];
        pad_to_length(&mut vec3, 3, 255);
        assert_eq!(vec3, vec![255, 255, 255]);
    }

    #[test]
    fn test_chunk_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = chunk_vec(&vec, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);

        let vec2 = vec![1, 2, 3, 4];
        let chunks2 = chunk_vec(&vec2, 2);
        assert_eq!(chunks2, vec![vec![1, 2], vec![3, 4]]);

        let empty: Vec<u8> = vec![];
        assert_eq!(chunk_vec(&empty, 5), Vec::<Vec<u8>>::new());

        let single = vec![42];
        assert_eq!(chunk_vec(&single, 10), vec![vec![42]]);
    }

    #[test]
    #[should_panic(expected = "chunk_size must be greater than 0")]
    fn test_chunk_vec_zero_size() {
        let vec = vec![1, 2, 3];
        chunk_vec(&vec, 0);
    }

    #[test]
    fn test_align_to_multiple() {
        let mut vec = vec![1, 2, 3];
        align_to_multiple(&mut vec, 4, 0);
        assert_eq!(vec, vec![1, 2, 3, 0]);

        let mut vec2 = vec![1, 2, 3, 4];
        align_to_multiple(&mut vec2, 4, 0);
        assert_eq!(vec2, vec![1, 2, 3, 4]);

        let mut vec3 = vec![1, 2, 3, 4, 5];
        align_to_multiple(&mut vec3, 4, 0xFF);
        assert_eq!(vec3, vec![1, 2, 3, 4, 5, 0xFF, 0xFF, 0xFF]);

        let mut vec4 = vec![1];
        align_to_multiple(&mut vec4, 8, 0);
        assert_eq!(vec4, vec![1, 0, 0, 0, 0, 0, 0, 0]);

        let mut empty: Vec<u8> = vec![];
        align_to_multiple(&mut empty, 4, 0);
        assert_eq!(empty, Vec::<u8>::new());
    }

    #[test]
    #[should_panic(expected = "alignment must be greater than 0")]
    fn test_align_to_multiple_zero_alignment() {
        let mut vec = vec![1, 2, 3];
        align_to_multiple(&mut vec, 0, 0);
    }
}
