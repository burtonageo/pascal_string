#![allow(missing_docs, unused_variables)]

mod pascal_str;
mod pascal_string;

pub use self::pascal_str::{Chars, Bytes, Lines, PascalStr};
pub use self::pascal_string::PascalString;

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{CStr, CString};
    use std::iter::IntoIterator;

    /*
    #[test]
    fn test_string_creation() {
        let test = "Hello, my world!".to_owned();
        let test_pascal = PascalString::from(&test).unwrap();
        assert_eq!(&test, test_pascal.as_str());

        let too_many_bytes = vec![12u8; 256];
        assert!(match PascalString::from(&too_many_bytes) {
            Err(PascalStringCreateError::InputTooLong) => true,
            _ => false
        });
    }
    */

    #[test]
    fn test_character_append() {
        let mut string = PascalString::new();
        assert!(string.try_push('h').is_ok());
        string.push('e');
        string.push(76u8 as char);
        string.push('l');
        string.push('L');
        string.push('o');
        string.push('\0');

        assert_eq!(string.as_str(), "heLlLo\0");
    }

    #[test]
    fn test_string_append() {
        let mut string = PascalString::new();
        string.push_str("Hola, ");
        string.push_str("Senor!");
        assert_eq!(string.as_str(), "Hola, Senor!");
    }
}
