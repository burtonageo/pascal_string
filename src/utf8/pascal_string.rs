use ::PASCAL_STRING_BUF_SIZE;

pub struct PascalString {
    len: u8,
    chars_buf: [u8; PASCAL_STRING_BUF_SIZE]
}
