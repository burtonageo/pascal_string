use ascii::{AsciiChar, AsciiStr};
use std::convert::AsRef;
use ::PASCAL_STRING_BUF_SIZE;

/// A borrowed slice from a `PascalString`. Does not own its data.
#[derive(Eq, Hash, PartialEq, PartialOrd)]
pub struct PascalStr {
    /// The `AsciiStr`, borrowed from the original `PascalString`
    string: AsciiStr
}

impl PascalStr {
    /// Get a pointer to the first byte of the string buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const AsciiChar {
        self.string.as_ptr()
    }

    /// Get a mutable pointer to the first byte of the string buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut AsciiChar {
        self.string.as_mut_ptr()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.string.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == PASCAL_STRING_BUF_SIZE
    }
}

impl AsRef<AsciiStr> for PascalStr {
    fn as_ref(&self) -> &AsciiStr {
        &self.string
    }
}

impl AsMut<AsciiStr> for PascalStr {
    fn as_mut(&mut self) -> &mut AsciiStr {
        &mut self.string
    }
}

impl AsRef<[AsciiChar]> for PascalStr {
    fn as_ref(&self) -> &[AsciiChar] {
        self.string.as_ref()
    }
}

impl AsMut<[AsciiChar]> for PascalStr {
    fn as_mut(&mut self) -> &mut [AsciiChar] {
        self.string.as_mut()
    }
}
