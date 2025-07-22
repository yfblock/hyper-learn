/// Console function to write a character.
#[inline]
#[allow(deprecated)]
pub fn putchar(ch: u8) {
    sbi_rt::legacy::console_putchar(ch as _);
}

/// Console function to read a character.
#[inline]
#[allow(deprecated)]
pub fn getchar() -> Option<u8> {
    let c = sbi_rt::legacy::console_getchar() as u8;
    match c == u8::MAX {
        true => None,
        _ => Some(c),
    }
}
