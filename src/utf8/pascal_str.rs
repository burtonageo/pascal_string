use std::borrow::{Cow, ToOwned};
use std::ffi::{CStr, CString};

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
        (&self.string).as_mut_ptr()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.string
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unimplemented!();
    }

    #[inline]
    pub fn
}

pub struct Chars;

pub struct CharsMut;

pub struct Lines;
