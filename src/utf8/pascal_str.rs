use std::borrow::{Cow, ToOwned};
use std::ffi::{CStr, CString};
use std::str;
use ::utf8::PascalString;

#[derive(Hash)]
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

impl ToOwned for PascalStr {
    type Owned = PascalString;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        PascalString::from_str(self.as_str()).unwrap()
    }
}

pub type Chars<'a> = str::Chars<'a>;
pub type Bytes<'a> = str::Bytes<'a>;
pub type Lines<'a> = str::Lines<'a>;

pub struct InteriorNullError;
