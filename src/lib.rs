#![warn(missing_docs, trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications,
        unused_results)]

//! # Pascal strings in Rust.
//!
//! A `PascalString`, or `ShortString` is a String which stores its data on the stack. Because of this, it has
//! a fixed maximum size, which cannot be changed. Traditionally, the size of a `PascalString` is 256 bytes -
//! the first byte stores the length, which means that each remaining byte is indexable using only that byte.
//!
//! This is a very niche string type - generally, you are better off using `std::string::String`, or the
//! `AsciiString` type from the `ascii` crate if you need an ascii string. They have no upper size limit, and
//! are cheaper to pass around as they are only 64 bytes on the stack. Generally, you should only use `PascalString` if:
//!
//! * You know that you absolutely, certainly cannot do without heap allocation.
//! * You need to store your string data inline into your `struct` type - for example if you will allocate a bunch
//!   of these custom `struct` types into a pool allocator, and cannot afford the heap fragmentation.
//! * You will keep, allocate, and deallocate a *lot* of short strings in your program.

extern crate ascii as ascii_crate;
extern crate odds;

/// Ascii encoded pascal strings.
pub mod ascii;

/// Utf8 encoded pascal strings.
pub mod utf8;

const PASCAL_STRING_BUF_SIZE: usize = ::std::u8::MAX as usize;
