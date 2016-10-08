extern crate ascii;

use ascii::{AsAsciiStrError, AsciiChar, AsciiStr, AsciiString, ToAsciiChar, ToAsciiCharError};
use std::convert::{AsRef, AsMut, From, Into};
use std::{fmt, ptr, slice, str};

const PASCAL_STRING_BUF_SIZE: usize = 255;

/// An owned `PascalString`. This string type stores its data the stack. It is always 256 bytes long, with
/// the first byte storing the length.
///
/// This string uses the Ascii encoding.
#[repr(C)]
pub struct PascalString {
    /// The length of this string.
    len: u8,
    /// The characters of this string, encoded as an ascii array.
    chars: [AsciiChar; PASCAL_STRING_BUF_SIZE]
}

impl PascalString {
    /// Creates a new, empty `PascalString`.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, PascalStringError> {
        PascalString::_from_bytes(bytes.as_ref())
    }

    fn _from_bytes(bytes: &[u8]) -> Result<Self, PascalStringError>  {
        let len = bytes.len();
        if len > PASCAL_STRING_BUF_SIZE {
            return Err(PascalStringError::OutOfBounds);
        }
        // Perform ascii check
        let ascii = try!(AsciiStr::from_ascii(bytes));

        let mut string = PascalString::new();
        string.len = len as u8;
        for i in 0..len {
            string.chars[i] = ascii[i];
        }
        Ok(string)
    }

    /// Push an ascii convertible character onto this string.
    ///
    /// # Panics
    ///
    /// Panics if the string is full, of if the `character` is not a valid ascii character.
    #[inline]
    pub fn push<C: ToAsciiChar>(&mut self, character: C) {
        self.try_push(character).unwrap();
    }

    /// Attempt to push an ascii convertible character onto the end of this `PascalString`.
    ///
    /// Returns `Err(_)` if the character cannot be pushed because this `PascalString` is full, or if
    /// `character` is not a valid ascii character.
    #[inline]
    pub fn try_push<C: ToAsciiChar>(&mut self, character: C) -> Result<(), PascalStringError> {
        self._try_push(try!(AsciiChar::from(character)))
    }

    fn _try_push(&mut self, ch: AsciiChar) -> Result<(), PascalStringError> {
        if self.is_full() {
            return Err(PascalStringError::OutOfBounds)
        }
        self.chars[self.len as usize] = ch;
        self.len += 1;
        Ok(())
    }

    /// Append a given string slice onto the end of this `PascalString`.
    ///
    /// # Panics
    ///
    /// Panics if the string cannot be pushed to because this `PascalString` is full.
    #[inline]
    pub fn push_str<S: AsRef<str>>(&mut self, s: S) {
        self.try_push_str(s).unwrap();
    }

    /// Attempt to append a given string slice onto the end of this `PascalString`.
    ///
    /// Returns `Err(_)` if the string cannot be pushed because this `PascalString` is full.
    #[inline]
    pub fn try_push_str<S: AsRef<str>>(&mut self, s: S) -> Result<(), PascalStringError> {
        self._try_push_str(s.as_ref())
    }

    fn _try_push_str(&mut self, s: &str) -> Result<(), PascalStringError> {
        unimplemented!()
    }

    /// Removes the last character from the string buffer and returns it.
    ///
    /// Returns `None` if this `PascalString` is empty.
    pub fn pop(&mut self) -> Option<AsciiChar> {
        if self.is_empty() {
            return None;
        }
        let c = self.chars[self.len as usize];
        self.len -= 1;
        Some(c)
    }

    pub fn remove(&mut self, index: u8) -> AsciiChar {
        assert!(self.len < index);
        let len = self.len as usize;
        let index = index as isize;
        let c = self.chars[len];
        // Shift everything to the right of the removed character to the left to cover up the hole
        // left.
        unsafe {
            let ptr = self.as_mut_ptr().offset(index);
            ptr::copy(ptr.offset(1), ptr, len - index.abs() as usize - 1);
        }
        self.len -= 1;
        c
    }

    #[inline]
    pub fn insert<C: ToAsciiChar>(&mut self, ch: C, index: u8) {
        self._insert(AsciiChar::from(ch).unwrap(), index)
    }

    fn _insert(&mut self, ch: AsciiChar, index: u8) {
        assert!(self.len < index);
        assert!(!self.is_full());
        let len = self.len as usize;
        let index = index as isize;
        // Shift everything to the right of `index` 1 place to the right to make room for the
        // new character.
        unsafe {
            let ptr = self.as_mut_ptr().offset(index);
            ptr::copy(ptr, ptr.offset(1), len - index.abs() as usize - 1);
        }
        self.chars[len] = ch;
        self.len += 1;
    }

    /// Truncates this String, removing all contents.
    ///
    /// Does not zero the values of the string.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == PASCAL_STRING_BUF_SIZE
    }

    /// Consumes this `PascalString`, and returns its inner state as a `[u8; 256]`, where the first byte
    /// is the length.
    ///
    /// Note that if the string has been truncated, bytes beyond the end of the string will not have been
    /// zeroed.
    #[inline]
    pub fn to_array(self) -> [u8; PASCAL_STRING_BUF_SIZE + 1] {
        self.into()
    }

    /// Get a pointer to the first byte of the string buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        &self.chars as *const _ as *const u8
    }

    /// Get a mutable pointer to the first byte of the string buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        &mut self.chars as *mut _ as *mut u8
    }
}

impl Default for PascalString {
    #[inline]
    fn default() -> Self {
        PascalString {
            len: 0,
            chars: [AsciiChar::Null; PASCAL_STRING_BUF_SIZE]
        }
    }
}

impl Clone for PascalString {
    fn clone(&self) -> Self {
        let mut clone = PascalString::default();
        clone.len = self.len;
        unsafe {
            ptr::copy_nonoverlapping(&self.chars, &mut clone.chars, PASCAL_STRING_BUF_SIZE);
        }
        clone
    }
}

impl fmt::Debug for PascalString {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self)
    }
}

impl fmt::Display for PascalString {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad(self.as_ref())
    }
}

impl AsRef<str> for PascalString {
    fn as_ref(&self) -> &str {
        let bytes: &[u8] = self.as_ref();
        str::from_utf8(bytes).unwrap()
    }
}

impl AsRef<[u8]> for PascalString {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.chars.as_ptr() as *const u8, self.len as usize)
        }
    }
}

impl AsRef<AsciiStr> for PascalString {
    fn as_ref(&self) -> &AsciiStr {
        let bytes: &[u8] = self.as_ref();
        AsciiStr::from_ascii(bytes).unwrap()
    }
}

impl AsRef<[AsciiChar]> for PascalString {
    fn as_ref(&self) -> &[AsciiChar] {
        unsafe {
            slice::from_raw_parts(self.chars.as_ptr(), self.len as usize)
        }
    }
}

impl AsMut<[AsciiChar]> for PascalString {
    fn as_mut(&mut self) -> &mut [AsciiChar] {
        unsafe {
            slice::from_raw_parts_mut(self.chars.as_mut_ptr(), self.len as usize)
        }
    }
}

impl Into<[u8; PASCAL_STRING_BUF_SIZE + 1]> for PascalString {
    fn into(self) -> [u8; PASCAL_STRING_BUF_SIZE + 1] {
        let mut array = [0u8; PASCAL_STRING_BUF_SIZE + 1];
        array[0] = self.len;
        unsafe {
            let chars_ptr = &self.chars as *const _ as *const [u8; PASCAL_STRING_BUF_SIZE];
            let array_ptr = (&mut array as *mut _).offset(1) as *mut [u8; PASCAL_STRING_BUF_SIZE];
            ptr::copy_nonoverlapping(chars_ptr, array_ptr, PASCAL_STRING_BUF_SIZE);
        }
        array
    }
}

impl Into<String> for PascalString {
    fn into(self) -> String {
        String::from_utf8_lossy(self.as_ref()).into_owned()
    }
}

#[derive(Debug, PartialEq)]
pub enum PascalStringError {
    OutOfBounds,
    InvalidChar(ToAsciiCharError),
    InvalidString(AsAsciiStrError)
}

impl From<ToAsciiCharError> for PascalStringError {
    fn from(e: ToAsciiCharError) -> Self {
        PascalStringError::InvalidChar(e)
    }
}

impl From<AsAsciiStrError> for PascalStringError {
    fn from(e: AsAsciiStrError) -> Self {
        PascalStringError::InvalidString(e)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
