use ascii::{AsciiChar, AsciiStr};
use std::borrow::ToOwned;
use std::convert::AsRef;
use std::ops::{Index, IndexMut, Range, RangeFull, RangeFrom, RangeTo};
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

