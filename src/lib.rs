extern crate ascii;

mod pascal_str;
mod pascal_string;

const PASCAL_STRING_BUF_SIZE: usize = 255;

pub use pascal_str::{Chars, CharsMut, PascalStr};
pub use pascal_string::{PascalString, PascalStringAppendError, PascalStringCreateError};

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::*;
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
}
