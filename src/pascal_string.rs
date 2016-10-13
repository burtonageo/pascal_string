use ascii::{AsAsciiStrError, AsciiChar, AsciiStr, AsciiString, ToAsciiChar, ToAsciiCharError};
use std::ascii::AsciiExt;
use std::borrow::{Borrow, BorrowMut};
use std::cmp::{Eq, PartialEq};
use std::convert::{AsRef, AsMut, From, Into};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::iter::{ExactSizeIterator, FromIterator, IntoIterator};
use std::ops::{Deref, DerefMut};
use std::{fmt, mem, ptr, slice, str};
use ::{PascalStr, PASCAL_STRING_BUF_SIZE};

/// An owned `PascalString`. This string type stores its data the stack. It is always 256 bytes long, with
/// the first byte storing the length.
///
/// This string type uses Ascii encoding.
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

    /// Create a new `PascalString` from its constituent parts: `string_len` and `char_array`.
    ///
    /// Returns an `Err` if `char_array` is not valid Ascii.
    #[inline]
    pub fn from_fixed_ascii_array<C>(string_len: u8, char_array: [C; PASCAL_STRING_BUF_SIZE])
                                     -> Result<Self, PascalStringCreateError>
        where C: ToAsciiChar + Clone {
        let mut pstring = PascalString::new();
        pstring.len = string_len;
        for i in 0..(pstring.len as usize) {
            pstring[i] = try!(AsciiChar::from(char_array[i].clone()));
        }
        Ok(pstring)
    }

    /// Create a new `PascalString` using the contents of `bytes`.
    ///
    /// Returns an `Err` if `bytes` is longer than 255 characters, or it does not contain
    /// Ascii encoded characters.
    #[inline]
    pub fn from<B: AsRef<[u8]>>(bytes: B) -> Result<Self, PascalStringCreateError> {
        PascalString::_from(bytes.as_ref())
    }

    fn _from(bytes: &[u8]) -> Result<Self, PascalStringCreateError>  {
        let len = bytes.len();
        if len > PASCAL_STRING_BUF_SIZE {
            return Err(PascalStringCreateError::InputTooLong);
        }
        // Perform ascii check
        let ascii = try!(AsciiStr::from_ascii(bytes));

        let mut string = PascalString::new();
        string.len = len as u8;
        for i in 0..len {
            string[i] = ascii[i];
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
    pub fn try_push<C: ToAsciiChar>(&mut self, character: C) -> Result<(), PascalStringAppendError> {
        self._try_push(try!(AsciiChar::from(character)))
    }

    fn _try_push(&mut self, ch: AsciiChar) -> Result<(), PascalStringAppendError> {
        if self.is_full() {
            return Err(PascalStringAppendError::NoRoom)
        }
        self.len += 1;
        let idx = self.len - 1;
        self[idx] = ch;
        self.set_trailing_byte_to_null();
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
    pub fn try_push_str<S: AsRef<str>>(&mut self, s: S) -> Result<(), PascalStringAppendError> {
        self._try_push_str(s.as_ref())
    }

    fn _try_push_str(&mut self, s: &str) -> Result<(), PascalStringAppendError> {
        let ascii: &[AsciiChar] = try!(AsciiStr::from_ascii(s)).as_ref();
        let slen = self.len();
        let alen = ascii.len();
        if slen + alen > PASCAL_STRING_BUF_SIZE {
            return Err(PascalStringAppendError::NoRoom);
        }
        for i in 0..alen {
            self.chars[(i + slen)] = ascii[i];
        }
        self.len += alen as u8;
        Ok(())
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
        self.set_trailing_byte_to_null();
        Some(c)
    }

    /// Remove a character from the `AsciiString` at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is larger than `self.len()`.
    pub fn remove(&mut self, index: u8) -> AsciiChar {
        assert!(self.len < index);
        let len = self.len as usize;
        let index = index as isize;
        let c = self[len];
        // Shift everything to the right of the removed character to the left to cover up the hole
        // left.
        unsafe {
            let ptr = self.as_mut_ptr().offset(index);
            ptr::copy(ptr.offset(1), ptr, len - index.abs() as usize - 1);
        }
        self.len -= 1;
        self.set_trailing_byte_to_null();
        c
    }

    /// Insert a character into the `AsciiString` at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is larger than `self.len()`, or if the `PascalString` is full.
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
        self[len] = ch;
        self.len += 1;
        self.set_trailing_byte_to_null();
    }

    /// Truncates this String, removing all contents.
    ///
    /// Does not zero the values of the string.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        self.set_trailing_byte_to_null();
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

    /// Get a character in the string, without checking if the index is within the bounds of `len()`.
    ///
    /// This method cannot cause memory unsafety because of the size of `index`. However, it can give access
    /// to stale characters if `index` is greater than `len()`, and `len() < 255`.
    #[inline]
    pub fn get_unchecked(&self, index: u8) -> AsciiChar {
        self.chars[index as usize]
    }

    /// Sets the byte beyond the end of `len` to `AsciiChar::Null`, if this `PascalString` isn't full.
    ///
    /// Used to ensure that `PascalStr::as_cstr()` works correctly.
    #[inline]
    fn set_trailing_byte_to_null(&mut self) {
        if !self.is_full() {
            self.chars[self.len as usize] = AsciiChar::Null;
        }
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

impl Eq for PascalString { }

impl PartialEq for PascalString {
    fn eq(&self, other: &PascalString) -> bool {
        if self.len != other.len {
            return false;
        }
        self.chars.iter().zip(other.chars.iter()).all(|(c0, c1)| c0 == c1)
    }
}

impl Hash for PascalString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.len);
        state.write(self.as_ref());
    }
}

impl AsciiExt for PascalString {
    type Owned = Self;

    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> Self::Owned {
        let mut upper = self.clone();
        upper.make_ascii_uppercase();
        upper
    }

    fn to_ascii_lowercase(&self) -> Self::Owned {
        let mut lower = self.clone();
        lower.make_ascii_lowercase();
        lower
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.chars().zip(other.chars()).all(|(c0, c1)| c0.eq_ignore_ascii_case(&c1))
    }

    #[inline]
    fn make_ascii_uppercase(&mut self) {
        for c in self.chars_mut() {
            c.make_ascii_uppercase();
        }
    }

    #[inline]
    fn make_ascii_lowercase(&mut self) {
        for c in self.chars_mut() {
            c.make_ascii_lowercase();
        }
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

impl Deref for PascalString {
    type Target = PascalStr;
    fn deref(&self) -> &Self::Target {
        let ascii_str: &[AsciiChar] = self.as_ref();
        unsafe {
            mem::transmute(ascii_str)
        }
    }
}

impl DerefMut for PascalString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ascii_str: &mut [AsciiChar] = self.as_mut();
        unsafe {
            mem::transmute(ascii_str)
        }
    }
}

impl AsRef<PascalStr> for PascalString {
    fn as_ref(&self) -> &PascalStr {
        self.deref()
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

impl Borrow<PascalStr> for PascalString {
    #[inline]
    fn borrow(&self) -> &PascalStr {
        self.deref()
    }
}

impl BorrowMut<PascalStr> for PascalString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut PascalStr {
        self.deref_mut()
    }
}

impl Borrow<str> for PascalString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl Borrow<[u8]> for PascalString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl Borrow<AsciiStr> for PascalString {
    #[inline]
    fn borrow(&self) -> &AsciiStr {
        self.as_ref()
    }
}

impl Borrow<[AsciiChar]> for PascalString {
    #[inline]
    fn borrow(&self) -> &[AsciiChar] {
        self.as_ref()
    }
}

impl BorrowMut<[AsciiChar]> for PascalString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [AsciiChar] {
        self.as_mut()
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
    #[inline]
    fn into(self) -> String {
        String::from_utf8_lossy(self.as_ref()).into_owned()
    }
}

impl Into<Vec<u8>> for PascalString {
    fn into(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.len());
        v.extend_from_slice(self.as_ref());
        v
    }
}

impl Into<Vec<AsciiChar>> for PascalString {
    fn into(self) -> Vec<AsciiChar> {
        let mut v = Vec::with_capacity(self.len());
        v.extend_from_slice(self.as_ref());
        v
    }
}

impl Into<AsciiString> for PascalString {
    fn into(self) -> AsciiString {
        AsciiString::from_ascii(self).unwrap()
    }
}

impl FromIterator<AsciiChar> for PascalString {
    fn from_iter<I: IntoIterator<Item = AsciiChar>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut pstring = PascalString::new();
        while let Some(ch) = iter.next() {
            // We know that the characters are valid ascii, and it's probably kinder to drop characters
            // past the 255th index than panic in this method.
            let _ = pstring.try_push(ch);
        }
        pstring
    }
}

impl IntoIterator for PascalString {
    type Item = AsciiChar;
    type IntoIter = IntoChars;
    fn into_iter(self) -> Self::IntoIter {
        IntoChars(self)
    }
}

/// An iterator from a `PascalString`. Has ownership of the iterated `PascalString`.
#[derive(Debug)]
pub struct IntoChars(PascalString);

impl Iterator for IntoChars {
    type Item = AsciiChar;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl ExactSizeIterator for IntoChars {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Indicates the range of errors which can occur from creating a new `PascalString`.
#[derive(Debug, PartialEq)]
pub enum PascalStringCreateError {
    /// The data provided to the constructor was larger than the `PascalString` could store.
    InputTooLong,
    /// The data provided was not correctly encoded as ascii.
    NotValidAscii(AsciiError)
}

impl fmt::Display for PascalStringCreateError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PascalStringCreateError::InputTooLong => fmtr.pad(self.description()),
            PascalStringCreateError::NotValidAscii(ref e) => write!(fmtr, "{}: {}", self.description(), e)
        }
    }
}

impl Error for PascalStringCreateError {
    fn description(&self) -> &str {
        match *self {
            PascalStringCreateError::InputTooLong => "the input data is longer than what a PascalString can store",
            PascalStringCreateError::NotValidAscii(_) =>"could not convert input data to ascii"
        }
    }

    fn cause(&self) -> Option<&Error> {
        if let PascalStringCreateError::NotValidAscii(ref e) = *self {
            Some(e)
        } else {
            None
        }
    }
}

impl<E: Into<AsciiError>> From<E> for PascalStringCreateError {
    #[inline]
    fn from(e: E) -> Self {
        PascalStringCreateError::NotValidAscii(e.into())
    }
}

/// Indicates the range of errors which can occur from appending string data to a `PascalString`.
#[derive(Debug, PartialEq)]
pub enum PascalStringAppendError {
    /// There is no room to store the appended data.
    NoRoom,
    /// The data provided was not correctly encoded as ascii.
    NotValidAscii(AsciiError)
}

impl fmt::Display for PascalStringAppendError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PascalStringAppendError::NoRoom => fmtr.pad(self.description()),
            PascalStringAppendError::NotValidAscii(ref e) => write!(fmtr, "{}: {}", self.description(), e)
        }
    }
}

impl Error for PascalStringAppendError {
    fn description(&self) -> &str {
        match *self {
            PascalStringAppendError::NoRoom => "there is no space left in the string to append the data",
            PascalStringAppendError::NotValidAscii(_) =>"could not convert string to ascii"
        }
    }

    fn cause(&self) -> Option<&Error> {
        if let PascalStringAppendError::NotValidAscii(ref e) = *self {
            Some(e)
        } else {
            None
        }
    }
}

impl<E: Into<AsciiError>> From<E> for PascalStringAppendError {
    #[inline]
    fn from(e: E) -> Self {
        PascalStringAppendError::NotValidAscii(e.into())
    }
}

/// An error type which abstracts over ascii conversion errors.
#[derive(Debug, PartialEq)]
pub enum AsciiError {
    /// A character was not encoded as ascii.
    Char(ToAsciiCharError),
    /// A string was not encoded as ascii.
    Str(AsAsciiStrError)
}

impl fmt::Display for AsciiError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsciiError::Char(ref e) => write!(fmtr, "{}: {}", self.description(), e),
            AsciiError::Str(ref e) => write!(fmtr, "{}: {}", self.description(), e)
        }
    }
}

impl Error for AsciiError {
    fn description(&self) -> &str {
        match *self {
            AsciiError::Char(_) => "could not convert character to ascii: {}",
            AsciiError::Str(_) =>"could not convert string to ascii: {}"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AsciiError::Char(ref e) => Some(e),
            AsciiError::Str(ref e) => Some(e)
        }
    }
}

impl From<ToAsciiCharError> for AsciiError {
    #[inline]
    fn from(e: ToAsciiCharError) -> Self {
        AsciiError::Char(e)
    }
}

impl From<AsAsciiStrError> for AsciiError {
    #[inline]
    fn from(e: AsAsciiStrError) -> Self {
        AsciiError::Str(e)
    }
}
