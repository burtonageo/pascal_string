use ascii::{AsciiChar, AsciiStr};
use std::ascii::AsciiExt;
use std::borrow::{Cow, ToOwned};
use std::cmp::Ordering;
use std::convert::AsRef;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::iter::{ExactSizeIterator, Iterator};
use std::ops::{Index, IndexMut, Range, RangeFull, RangeFrom, RangeTo};
use std::slice::{Iter, IterMut};
use std::{fmt, isize, mem, slice};
use ::{PASCAL_STRING_BUF_SIZE, PascalString};

/// A borrowed slice from a `PascalString`. Does not own its data.
#[derive(Eq, Hash, Ord)]
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

    /// Get the `PascalStr` as an immutable `&str` reference.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    /// Get this string as a `CStr`.
    ///
    /// Returns `Err(InteriorNullError)` if the string contains any interior nulls. If this string is full,
    /// then a new `CString` will be allocated to hold the trailing null byte.
    #[inline]
    pub fn as_cstr(&self) -> Result<Cow<CStr>, InteriorNullError> {
        match self.chars().position(|&c| c == AsciiChar::Null) {
            Some(pos) if pos != (self.len() - 1) => Err(InteriorNullError(pos)),
            _ if self.is_full() && self.string[PASCAL_STRING_BUF_SIZE - 1] != AsciiChar::Null => {
                let str_clone = self.to_owned();
                Ok(Cow::Owned(CString::new(str_clone).unwrap()))
            }
            _ => Ok(Cow::Borrowed(CStr::from_bytes_with_nul(self.as_ref()).unwrap()))
        }
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

    /// Get an iterator over the lines of the internal character array.
    #[inline]
    pub fn lines(&self) -> Lines {
        Lines {
            current_index: 0,
            string: &self
        }
    }

    /// Get a character in the string, without checking if the index is within the bounds of `len()`.
    ///
    /// This method cannot cause memory unsafety because `index` is bounds checked within the maximum possible
    /// length of the `PascalStr`, which means that it cannot read uninitialised memory. However, it can give access
    /// to stale characters if `index` is greater than or equal to `self.len()` or `isize::MAX`, and `self.is_full()`
    /// is `false`.
    ///
    /// # Panics
    ///
    /// This method will panic if `index` is larger than `u8::MAX`(255).
    #[inline]
    pub fn get_unchecked(&self, index: usize) -> AsciiChar {
        assert!(index < PASCAL_STRING_BUF_SIZE);
        let ptr = self.as_ptr();
        unsafe {
            *ptr.offset(index as isize)
        }
    }
}

impl<S: AsRef<PascalStr> + ?Sized> PartialEq<S> for PascalStr {
    fn eq(&self, other: &S) -> bool {
        let other = other.as_ref();
        self.string.eq(&other.string)
    }
}

impl<S: AsRef<PascalStr> + ?Sized> PartialOrd<S> for PascalStr {
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        let other = other.as_ref();
        self.string.partial_cmp(&other.string)
    }
}

impl AsciiExt for PascalStr {
    type Owned = PascalString;

    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> Self::Owned {
        let bytes: &[u8] = self.as_ref();
        let mut upper = PascalString::from(bytes).unwrap();
        upper.make_ascii_uppercase();
        upper
    }

    fn to_ascii_lowercase(&self) -> Self::Owned {
        let bytes: &[u8] = self.as_ref();
        let mut lower = PascalString::from(bytes).unwrap();
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

impl fmt::Debug for PascalStr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad(self.as_ref())
    }
}

impl fmt::Display for PascalStr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad(self.as_ref())
    }
}

impl ToOwned for PascalStr {
    type Owned = PascalString;
    fn to_owned(&self) -> Self::Owned {
        PascalString::from(&self.string).unwrap()
    }
}

impl AsRef<PascalStr> for PascalStr {
    fn as_ref(&self) -> &Self {
        &self
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

impl Index<i32> for PascalStr {
    type Output = AsciiChar;
    fn index(&self, index: i32) -> &Self::Output {
        assert!(index >= 0);
        assert!((index as usize) < self.len());
        &self.string[index as usize]
    }
}

impl IndexMut<i32> for PascalStr {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        assert!(index >= 0);
        assert!((index as usize) < self.len());
        &mut self.string[index as usize]
    }
}

impl Index<RangeFull> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, _: RangeFull) -> &Self::Output {
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[..]
    }
}

impl IndexMut<RangeFull> for PascalStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[..]
    }
}

impl Index<Range<u8>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: Range<u8>) -> &Self::Output {
        assert!((range.end as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start as usize..range.end as usize]
    }
}

impl IndexMut<Range<u8>> for PascalStr {
    fn index_mut(&mut self, range: Range<u8>) -> &mut Self::Output {
        assert!((range.end as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start as usize..range.end as usize]
    }
}

impl Index<Range<usize>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: Range<usize>) -> &Self::Output {
        assert!(range.end < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start..range.end]
    }
}

impl IndexMut<Range<usize>> for PascalStr {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        assert!(range.end < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start..range.end]
    }
}

impl Index<Range<i32>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: Range<i32>) -> &Self::Output {
        assert!(range.start >= 0);
        assert!((range.start as usize) < self.len());
        assert!(range.end >= 0);
        assert!((range.end as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start as usize..]
    }
}

impl IndexMut<Range<i32>> for PascalStr {
    fn index_mut(&mut self, range: Range<i32>) -> &mut Self::Output {
        assert!(range.start >= 0);
        assert!((range.start as usize) < self.len());
        assert!(range.end >= 0);
        assert!((range.end as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start as usize..]
    }
}

impl Index<RangeFrom<u8>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeFrom<u8>) -> &Self::Output {
        assert!((range.start as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start as usize..]
    }
}

impl IndexMut<RangeFrom<u8>> for PascalStr {
    fn index_mut(&mut self, range: RangeFrom<u8>) -> &mut Self::Output {
        assert!((range.start as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start as usize..]
    }
}

impl Index<RangeFrom<usize>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        assert!(range.start < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start..]
    }
}

impl IndexMut<RangeFrom<usize>> for PascalStr {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        assert!(range.start < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start..]
    }
}

impl Index<RangeFrom<i32>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeFrom<i32>) -> &Self::Output {
        assert!(range.start >= 0);
        assert!((range.start as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[range.start as usize..]
    }
}

impl IndexMut<RangeFrom<i32>> for PascalStr {
    fn index_mut(&mut self, range: RangeFrom<i32>) -> &mut Self::Output {
        assert!(range.start >= 0);
        assert!((range.start as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[range.start as usize..]
    }
}

impl Index<RangeTo<u8>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeTo<u8>) -> &Self::Output {
        assert!((range.end as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[..range.end as usize]
    }
}

impl IndexMut<RangeTo<u8>> for PascalStr {
    fn index_mut(&mut self, range: RangeTo<u8>) -> &mut Self::Output {
        assert!((range.end as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[..range.end as usize]
    }
}

impl Index<RangeTo<usize>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        assert!(range.end < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[..range.end]
    }
}

impl IndexMut<RangeTo<usize>> for PascalStr {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        assert!(range.end < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[..range.end]
    }
}

impl Index<RangeTo<i32>> for PascalStr {
    type Output = [AsciiChar];
    fn index(&self, range: RangeTo<i32>) -> &Self::Output {
        assert!(range.end >= 0);
        assert!((range.end as usize) < self.len());
        let char_array: &[AsciiChar] = self.string.as_ref();
        &char_array[..range.end as usize]
    }
}

impl IndexMut<RangeTo<i32>> for PascalStr {
    fn index_mut(&mut self, range: RangeTo<i32>) -> &mut Self::Output {
        assert!(range.end >= 0);
        assert!((range.end as usize) < self.len());
        let char_array: &mut [AsciiChar] = self.string.as_mut();
        &mut char_array[..range.end as usize]
    }
}

impl<'a> IntoIterator for &'a PascalStr {
    type Item = &'a AsciiChar;
    type IntoIter = Chars<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

impl<'a> IntoIterator for &'a mut PascalStr {
    type Item = &'a mut AsciiChar;
    type IntoIter = CharsMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.chars_mut()
    }
}

/// An immutable iterator over the buffer of a `PascalStr`.
#[derive(Debug)]
pub struct Chars<'a>(Iter<'a, AsciiChar>);

impl<'a> Iterator for Chars<'a> {
    type Item = &'a AsciiChar;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> ExactSizeIterator for Chars<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// A mutable iterator over the buffer of a `PascalStr`.
#[derive(Debug)]
pub struct CharsMut<'a>(IterMut<'a, AsciiChar>);

impl<'a> Iterator for CharsMut<'a> {
    type Item = &'a mut AsciiChar;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> ExactSizeIterator for CharsMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// An iterator over the lines of the internal character array.
#[derive(Debug)]
pub struct Lines<'a> {
    current_index: usize,
    string: &'a PascalStr
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a PascalStr;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_idx = self.current_index;
        let len = self.string.len();
        if curr_idx >= len {
            return None;
        }

        let mut next_idx = None;
        for i in curr_idx..len {
            if self.string[i] == AsciiChar::LineFeed {
                next_idx = Some(i);
                break;
            }
        }
        let next_idx = match next_idx {
            Some(i) => i,
            None => return None
        };
        let line: &PascalStr = unsafe {
            let ptr = self.string.as_ptr().offset(curr_idx as isize);
            let len = next_idx - curr_idx;
            let slice = slice::from_raw_parts(ptr, len);
            mem::transmute(slice)
        };
        self.current_index = next_idx + 1; // skip the linefeed
        Some(line)
    }
}

impl<'a> ExactSizeIterator for Lines<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.string.chars().skip(self.current_index).filter(|&&c| c == AsciiChar::LineFeed).count()
    }
}

#[derive(Clone, Debug, Hash)]
pub struct InteriorNullError(usize);

impl InteriorNullError {
    #[inline]
    pub fn interior_null_index(&self) -> usize {
        self.0
    }
}

impl fmt::Display for InteriorNullError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "interior null at {}", self.0)
    }
}

impl Error for InteriorNullError {
    fn description(&self) -> &str {
        "an interior null was found when creating a CStr from a pascal string"
    }
}
