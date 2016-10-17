use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr, str};
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
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn from(s: &str) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.try_push(ch).unwrap()
    }

    pub fn try_push(&mut self, ch: char) -> Result<(), PascalStringAppendError> {
        let unich = UniChar::from(ch);
        unimplemented!();
    }

    #[inline]
    pub fn push_str<S: AsRef<str>>(&mut self, s: &S) {
        self.try_push_str(s).unwrap()
    }

    #[inline]
    pub fn try_push_str<S: AsRef<str>>(&mut self, s: &S) -> Result<(), PascalStringAppendError> {
        self._try_push_str(s.as_ref())
    }

    fn _try_push_str(&mut self, s: &str) -> Result<(), PascalStringAppendError> {
        unimplemented!()
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

#[derive(Debug)]
pub enum PascalStringAppendError {
    NoRoom,
}

/// A view into the individual bytes that make up a `char`.
///
/// Use the `From` trait to convert a `char` to a `UniChar` without using `unsafe`.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct UniChar {
    /// First byte.
    b0: u8,
    /// Second byte.
    b1: u8,
    /// Third byte.
    b2: u8,
    /// Fourth byte.
    b3: u8
}

impl UniChar {
    fn num_bytes(&self) -> u8 {
        unimplemented!()
    }
}

impl From<char> for UniChar {
    fn from(ch: char) -> Self {
        let chn: u32 = ch as u32;
        UniChar {
            b0: ((chn & 0xff000000u32) >> 24) as u8,
            b1: ((chn & 0x00ff0000u32) >> 16) as u8,
            b2: ((chn & 0x0000ff00u32) >> 8) as u8,
            b3:  (chn & 0x000000ffu32) as u8
        }
    }
}

