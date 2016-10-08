extern crate ascii;

mod pascal_str;
mod pascal_string;

const PASCAL_STRING_BUF_SIZE: usize = 255;

pub use pascal_str::PascalStr;
pub use pascal_string::PascalString;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
