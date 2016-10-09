use ascii::{AsciiChar, AsciiStr};
use std::ascii::AsciiExt;
use std::borrow::ToOwned;
use std::convert::AsRef;
use std::iter::{ExactSizeIterator, Iterator};
use std::ops::{Index, IndexMut, Range, RangeFull, RangeFrom, RangeTo};
use std::slice::{Iter, IterMut};
use ::{PASCAL_STRING_BUF_SIZE, PascalString};

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

    /// Returns the number of characters used in the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.string.len()
    }

    /// Returns true if the string has a length of 0
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the string has a length of 255.
    ///
    /// When this value is true, no more elements can be pushed onto the string.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == PASCAL_STRING_BUF_SIZE
    }

    /// Get an immutable iterator to the internal character array.
    #[inline]
    pub fn chars<'a>(&'a self) -> Chars<'a> {
        Chars(self.string.as_slice().iter())
    }

    /// Get a mutable iterator to the internal character array.
    #[inline]
    pub fn chars_mut<'a>(&'a mut self) -> CharsMut<'a> {
        CharsMut(self.string.as_mut_slice().iter_mut())
    }
}

impl AsciiExt for PascalStr {
    type Owned = PascalString;

    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> Self::Owned {
        let bytes: &[u8] = self.as_ref();
        let mut upper = PascalString::from_bytes(bytes).unwrap();
        upper.make_ascii_uppercase();
        upper
    }

    fn to_ascii_lowercase(&self) -> Self::Owned {
        let bytes: &[u8] = self.as_ref();
        let mut lower = PascalString::from_bytes(bytes).unwrap();
        lower.make_ascii_lowercase();
        lower
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.string.eq_ignore_ascii_case(&other.string)
    }

    fn make_ascii_uppercase(&mut self) {
        self.string.make_ascii_uppercase()
    }

    fn make_ascii_lowercase(&mut self) {
        self.string.make_ascii_lowercase()
    }
}

impl ToOwned for PascalStr {
    type Owned = PascalString;
    fn to_owned(&self) -> Self::Owned {
        PascalString::from_bytes(&self.string).unwrap()
    }
}

impl AsRef<str> for PascalStr {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}

impl AsRef<[u8]> for PascalStr {
    fn as_ref(&self) -> &[u8] {
        self.string.as_ref()
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

impl Index<u8> for PascalStr {
    type Output = AsciiChar;
    fn index(&self, index: u8) -> &Self::Output {
        let index = index as usize;
        assert!(index < self.len());
        &self.string[index]
    }
}

impl IndexMut<u8> for PascalStr {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let index = index as usize;
        assert!(index < self.len());
        &mut self.string[index]
    }
}

impl Index<usize> for PascalStr {
    type Output = AsciiChar;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        &self.string[index]
    }
}

impl IndexMut<usize> for PascalStr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len());
        &mut self.string[index]
    }
}

impl Index<RangeFull> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.string[..]
    }
}

impl IndexMut<RangeFull> for PascalStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
        &mut self.string[..]
    }
}

impl Index<Range<u8>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: Range<u8>) -> &Self::Output {
        assert!((range.end as usize) < self.len());
        &self.string[range.start as usize..range.end as usize]
    }
}

impl IndexMut<Range<u8>> for PascalStr {
    fn index_mut(&mut self, range: Range<u8>) -> &mut Self::Output {
        assert!((range.end as usize) < self.len());
        &mut self.string[range.start as usize..range.end as usize]
    }
}

impl Index<Range<usize>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: Range<usize>) -> &Self::Output {
        assert!(range.end < self.len());
        &self.string[range.start..range.end]
    }
}

impl IndexMut<Range<usize>> for PascalStr {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        assert!(range.end < self.len());
        &mut self.string[range.start..range.end]
    }
}

impl Index<RangeFrom<u8>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: RangeFrom<u8>) -> &Self::Output {
        &self.string[range.start as usize..]
    }
}

impl IndexMut<RangeFrom<u8>> for PascalStr {
    fn index_mut(&mut self, range: RangeFrom<u8>) -> &mut Self::Output {
        &mut self.string[range.start as usize..]
    }
}

impl Index<RangeFrom<usize>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.string[range.start..]
    }
}

impl IndexMut<RangeFrom<usize>> for PascalStr {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.string[range.start..]
    }
}

impl Index<RangeTo<u8>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: RangeTo<u8>) -> &Self::Output {
        assert!((range.end as usize) < self.len());
        &self.string[..range.end as usize]
    }
}

impl IndexMut<RangeTo<u8>> for PascalStr {
    fn index_mut(&mut self, range: RangeTo<u8>) -> &mut Self::Output {
        assert!((range.end as usize) < self.len());
        &mut self.string[..range.end as usize]
    }
}

impl Index<RangeTo<usize>> for PascalStr {
    type Output = AsciiStr;
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        assert!(range.end < self.len());
        &self.string[..range.end]
    }
}

impl IndexMut<RangeTo<usize>> for PascalStr {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        assert!(range.end < self.len());
        &mut self.string[..range.end]
    }
}

/// An immutable iterator over the buffer of a `PascalStr`.
pub struct Chars<'a>(Iter<'a, AsciiChar>);

impl<'a> Iterator for Chars<'a> {
    type Item = &'a AsciiChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> ExactSizeIterator for Chars<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// A mutable iterator over the buffer of a `PascalStr`.
pub struct CharsMut<'a>(IterMut<'a, AsciiChar>);

impl<'a> Iterator for CharsMut<'a> {
    type Item = &'a mut AsciiChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> ExactSizeIterator for CharsMut<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
