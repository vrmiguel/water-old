//! A cheaply-clonable String type
// Taken from github.com/vrmiguel/ceceio
use std::{
    borrow::Borrow, fmt, hash::Hash, ops::Deref, rc::Rc, str,
};

pub const INLINE_CAP: usize = 22;

#[derive(Clone, PartialEq, Eq)]
/// A cheaply-clonable String type
pub enum SmallString {
    Inlined { len: u8, buf: [u8; INLINE_CAP] },
    Heap(Rc<str>),
}

impl Hash for SmallString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SmallString::Inlined { len, buf } => {
                // Safety: len is guaranteed to be <= INLINE_CAP when
                // creating SmallString::Inlined, and INLINE_CAP equals
                // buf.len(), so 0..*len as usize is always within bounds
                unsafe { buf.get_unchecked(0..*len as usize) }
                    .hash(state);
            }
            SmallString::Heap(rc) => {
                // Cold branch since identifiers tend to be
                // smaller than 23 bytes
                cold();
                rc.hash(state);
            }
        }

        #[cold]
        fn cold() {}
    }
}

impl fmt::Debug for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `s#` prefix -to flag that this is a SmallString
        write!(f, "s#\"{self}\"")
    }
}

impl fmt::Display for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SmallString {
    /// Creates an inlined variant of SmallString for strings that fit within the inline capacity.
    ///
    /// This function constructs a `SmallString::Inlined` variant by copying the provided
    /// bytes into a fixed-size buffer. It's used internally when the string length is
    /// less than or equal to `INLINE_CAP` (22 bytes), allowing the string data to be
    /// stored directly within the enum variant without heap allocation.
    ///
    /// # Arguments
    /// * `bytes` - A byte slice containing the string data to be inlined
    ///
    /// # Returns
    /// * `Self` - A `SmallString::Inlined` variant containing the copied string data
    ///
    /// # Safety
    /// This function assumes that:
    /// - The input bytes represent valid UTF-8 (verified by caller)
    /// - The byte slice length is <= `INLINE_CAP` (asserted in debug builds)
    ///
    /// # Panics
    /// In debug builds, panics if `bytes.len() > INLINE_CAP`.
    #[inline(always)]
    fn inlined(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() <= INLINE_CAP);
        let mut buf = [0u8; INLINE_CAP];

        // Safety: This function is internal and only called after
        // verifying that bytes.len() <= INLINE_CAP, and INLINE_CAP
        // equals buf.len(). Therefore, 0..bytes.len() is guaranteed
        // to be within bounds of buf.
        unsafe { buf.get_unchecked_mut(0..bytes.len()) }
            .copy_from_slice(bytes);
        Self::Inlined {
            len: bytes.len() as u8,
            buf,
        }
    }

    #[must_use]
    pub fn is_in_heap(&self) -> bool {
        matches!(self, Self::Heap(_))
    }

    #[must_use]
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        let string = input.as_ref();
        let bytes = string.as_bytes();

        if bytes.len() > INLINE_CAP {
            Self::Heap(Rc::from(string))
        } else {
            Self::inlined(bytes)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            // Safety: SmallString::Inlined can only be created from
            // valid UTF-8 strings via `AsRef<str>`, and len is
            // guaranteed to be <= INLINE_CAP during construction.
            // The slice &buf[..*len as usize] represents the exact
            // bytes that were validated as UTF-8 when the SmallString
            // was created.
            SmallString::Inlined { buf, len } => unsafe {
                std::str::from_utf8_unchecked(
                    &buf[..*len as usize],
                )
            },
            SmallString::Heap(rc) => rc.as_ref(),
        }
    }
}

impl AsRef<str> for SmallString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for SmallString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for SmallString {
    fn borrow(&self) -> &str {
        self
    }
}

impl From<&str> for SmallString {
    fn from(slice: &str) -> Self {
        SmallString::new(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::SmallString;

    #[test]
    fn creates_inlined_small_strings_correctly() {
        let hey = SmallString::new("hey");
        assert_eq!(hey.as_str(), "hey");
        assert!(!hey.is_in_heap());

        let length_22 =
            SmallString::new("abcdefghijkabcdefghijk");
        assert_eq!(length_22.as_str(), "abcdefghijkabcdefghijk");
        assert!(!length_22.is_in_heap());

        let length_23 =
            SmallString::new("abcdefghijkabcdefghijkz");
        assert_eq!(
            length_23.as_str(),
            "abcdefghijkabcdefghijkz"
        );
        assert!(length_23.is_in_heap());
    }
}
