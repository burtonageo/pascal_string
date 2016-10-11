extern crate ascii;

mod pascal_str;
mod pascal_string;

const PASCAL_STRING_BUF_SIZE: usize = 255;

pub use pascal_str::{Chars, CharsMut, PascalStr};
pub use pascal_string::{PascalString, PascalStringAppendError, PascalStringCreateError, AsciiError};

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::*;
    use std::borrow::Cow;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_string_creation() {
        let test = "Hello, my world!".to_owned();
        let test_pascal = PascalString::from(&test).unwrap();
        assert_eq!(&test, test_pascal.as_str());
    }

    #[test]
    fn test_character_append() {
        let mut string = PascalString::new();
        assert!(string.try_push('h').is_ok());
        string.push('e');
        string.push(76u8);
        string.push('l');
        string.push(AsciiChar::L);
        string.push('o');
        string.push(AsciiChar::Null);

        println!("{}", string);
        assert_eq!(string.as_str(), "heLlLo\0");
    }

    #[test]
    fn test_string_append() {
        let mut string = PascalString::new();
        string.push_str("Hola, ");
        string.push_str("Senor!");
        assert_eq!(string.as_str(), "Hola, Senor!");
    }

    #[test]
    fn test_string_indexing_and_char_iteration() {
        let mut string = PascalString::from("q").unwrap();
        assert_eq!(string[0u8], AsciiChar::q);

        let mut string2 = PascalString::from("WASD").unwrap();
        let mut iter = string2.chars();
        assert_eq!(iter.next(), Some(&AsciiChar::W));
        assert_eq!(iter.next(), Some(&AsciiChar::A));
        assert_eq!(iter.next(), Some(&AsciiChar::S));
        assert_eq!(iter.next(), Some(&AsciiChar::D));
    }

    #[test]
    fn test_as_cstr() {
        let msg = "I am your favourite cookie monster >:-)\0";
        let pstr = PascalString::from(&msg).unwrap();
        let cstr = CStr::from_bytes_with_nul(msg.as_bytes()).unwrap();
        let pstr_as_cstr = pstr.as_cstr().unwrap();
        assert!(match pstr_as_cstr {
            Cow::Borrowed(_) => true,
            _ => false
        });
        assert_eq!(&*pstr_as_cstr, cstr);

        let oversized = ['l'; 255];
        let mut string_oversized = {
            let mut s = String::new();
            for i in 0..oversized.len() {
                s.push(oversized[i]);
            }
            s
        };
        let pstr_oversized = PascalString::from_fixed_ascii_array(255, oversized).unwrap();
        println!("oversized: {}", pstr_oversized.is_full());
        let cstr_from_pstr_oversized = pstr_oversized.as_cstr().unwrap();
        let cstr_from_string_oversized = CString::new(string_oversized).unwrap();
        assert!(match cstr_from_pstr_oversized {
            Cow::Owned(_) => true,
            _ => false
        });
        assert_eq!(cstr_from_pstr_oversized.into_owned(), cstr_from_string_oversized);
    }
}
