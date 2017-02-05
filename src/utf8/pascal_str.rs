use std::borrow::{Cow, ToOwned};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ffi::{CStr, CString};
use std::str;
use ::utf8::PascalString;
use ::PASCAL_STRING_BUF_SIZE;

#[derive(Hash, Eq, Ord)]
pub struct PascalStr {
    string: str
}

impl PascalStr {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        (&self.string).as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        &mut self.string as *mut str as *mut u8
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.string
    }

    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.string
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn as_cstr(&self) -> Result<Cow<CStr>, InteriorNullError> {
        unimplemented!()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.string.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == PASCAL_STRING_BUF_SIZE
    }

    #[inline]
    pub fn chars(&self) -> Chars {
        self.string.chars()
    }

    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.string.bytes()
    }

    #[inline]
    pub fn lines(&self) -> Lines {
        self.string.lines()
    }
}

impl<S: AsRef<str> + ?Sized> PartialEq<S> for PascalStr {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        let other = other.as_ref();
        self.as_str() == other
    }
}

impl<S: AsRef<str> + ?Sized> PartialOrd<S> for PascalStr {
    #[inline]
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        let other = other.as_ref();
        self.as_str().partial_cmp(&other)
    }
}

impl ToOwned for PascalStr {
    type Owned = PascalString;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        PascalString::from_str(self.as_str()).unwrap()
    }
}

impl AsRef<str> for PascalStr {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.string
    }
}

pub type Chars<'a> = str::Chars<'a>;
pub type Bytes<'a> = str::Bytes<'a>;
pub type Lines<'a> = str::Lines<'a>;

pub struct InteriorNullError;
