#![allow(missing_docs, unused_variables)]

mod pascal_str;
mod pascal_string;

pub use self::pascal_str::{Chars, Bytes, Lines, PascalStr};
pub use self::pascal_string::PascalString;

#[cfg(test)]
mod tests {
}
