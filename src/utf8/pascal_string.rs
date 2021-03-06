use std::borrow::{Borrow, BorrowMut, Cow};
use std::cmp::{Eq, PartialEq, Ord, Ordering, PartialOrd};
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::{fmt, mem, ptr, str};
use odds::char::{encode_utf8, EncodeUtf8Error};
use ::utf8::PascalStr;
use ::PASCAL_STRING_BUF_SIZE;

/// An owned `PascalString`. This string type stores its data the stack. It is always 256 bytes long, with
/// the first byte storing the length *of the number of bytes used*.
///
/// Note that because this string type is utf8 encoded, the first byte will not store the number of characters
/// in the string.
pub struct PascalString {
    /// The number of bytes used in the string.
    len: u8,
    /// The internal character buffer, encoded in utf8
    chars_buf: [u8; PASCAL_STRING_BUF_SIZE]
}

impl PascalString {
    /// Creates a new, empty `PascalString`.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `PascalString` with the contents of `s`.
    ///
    /// # Returns
    ///
    /// If the contents of `s` can be stored in the buffer of the `PascalString`, then it returns
    /// `Ok`. Otherwise, returns `Err`.
    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<Self, PascalStringCreateError> {
        let s = s.as_ref();
        if s.len() > PASCAL_STRING_BUF_SIZE {
            return Err(PascalStringCreateError::InputTooLong);
        }
        let mut pstring = PascalString::new();
        for ch in s.chars() {
            pstring.push(ch)
        }
        Ok(pstring)
    }

    /// Push a character onto the end of the string's internal buffer.
    ///
    /// # Panics
    ///
    /// Panics if there is no room to store the `char`.
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.try_push(ch).unwrap()
    }

    /// Attempt to push a `char` onto the end of this string's internal buffer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation succeeded, otherwise an error is returned.
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), EncodeUtf8Error> {
        let num_bytes_extended = try!(encode_utf8(ch, &mut self.chars_buf[self.len as usize..]));
        self.len += num_bytes_extended as u8;
        Ok(())
    }

    /// Push a string onto the end of this string's internal buffer.
    ///
    /// # Panics
    ///
    /// Panics if there is no room to store the `str`.
    #[inline]
    pub fn push_str<S: AsRef<str>>(&mut self, s: S) {
        self.try_push_str(s).unwrap()
    }

    /// Attempt to push a string onto the end of this string's internal buffer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation succeeded, otherwise an error is returned.
    #[inline]
    pub fn try_push_str<S: AsRef<str>>(&mut self, s: S) -> Result<(), PascalStringAppendError> {
        self._try_push_str(s.as_ref())
    }

    fn _try_push_str(&mut self, s: &str) -> Result<(), PascalStringAppendError> {
        if self.len() + s.len() > PASCAL_STRING_BUF_SIZE {
            return Err(PascalStringAppendError::NoRoom);
        }
        for ch in s.chars() {
            try!(self.try_push(ch))
        }
        Ok(())
    }
}

impl fmt::Debug for PascalString {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("PascalString")
            .field("len", &self.len)
            .field("chars_buf", &self.as_str())
            .finish()
    }
}

impl fmt::Display for PascalString {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad(self.as_str())
    }
}

impl Default for PascalString {
    #[inline]
    fn default() -> Self {
        PascalString {
            len: 0,
            chars_buf: [0u8; PASCAL_STRING_BUF_SIZE]
        }
    }
}

impl Clone for PascalString {
    #[inline]
    fn clone(&self) -> Self {
        let mut clone = PascalString::default();
        clone.len = self.len;
        unsafe {
            ptr::copy_nonoverlapping(&self.chars_buf, &mut clone.chars_buf, PASCAL_STRING_BUF_SIZE);
        }
        clone
    }
}

impl Hash for PascalString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.len);
        state.write(&self.chars_buf);
    }
}

impl<S: AsRef<str> + ?Sized> PartialEq<S> for PascalString {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        let other = other.as_ref();
        self.as_str() == other
    }
}

impl Eq for PascalString {}

impl<S: AsRef<str> + ?Sized> PartialOrd<S> for PascalString {
    #[inline]
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        let other = other.as_ref();
        self.as_str().partial_cmp(&other)
    }
}

impl Ord for PascalString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Deref for PascalString {
    type Target = PascalStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(&self.chars_buf[0..(self.len as usize)]) }
    }
}

impl DerefMut for PascalString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(&mut self.chars_buf[0..(self.len as usize)]) }
    }
}

impl AsRef<PascalStr> for PascalString {
    #[inline]
    fn as_ref(&self) -> &PascalStr {
        self.deref()
    }
}

impl AsRef<str> for PascalString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
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
        self.as_str()
    }
}

impl BorrowMut<str> for PascalString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PascalStringCreateError {
    InputTooLong
}

#[derive(Debug, Clone, Copy)]
pub enum PascalStringAppendError {
    NoRoom,
    EncodeError(EncodeUtf8Error)
}

impl fmt::Display for PascalStringAppendError {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let desc = self.description();
        match *self {
            PascalStringAppendError::NoRoom => fmtr.pad(desc),
            PascalStringAppendError::EncodeError(ref e) => write!(fmtr, "{}: {:?}", desc, e)
        }
    }
}

impl Error for PascalStringAppendError {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            PascalStringAppendError::NoRoom => "there is no room for the string to be appended",
            PascalStringAppendError::EncodeError(_) => "there was a problem encoding the character as utf8"
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        // std::error::Error is not implemented for EncodeUtf8Error
        /*
        if let PascalStringAppendError::EncodeError(ref e) = *self {
            Some(e)
        } else {
            None
        }
        */
        None
    }
}

impl From<EncodeUtf8Error> for PascalStringAppendError {
    #[inline]
    fn from(e: EncodeUtf8Error) -> Self {
        PascalStringAppendError::EncodeError(e)
    }
}
