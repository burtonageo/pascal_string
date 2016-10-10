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
}
