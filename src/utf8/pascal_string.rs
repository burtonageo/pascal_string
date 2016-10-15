use ::PASCAL_STRING_BUF_SIZE;

pub struct PascalString {
    /// The number of bytes used in the string.
    len: u8,
    /// The internal character buffer, encoded in utf8
    chars_buf: [u8; PASCAL_STRING_BUF_SIZE]
}
