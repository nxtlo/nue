use core::str::from_utf8_unchecked;

/// Parses a raw UID byte array into a colon-separated hex string.
#[inline]
pub(crate) const fn parse_uid<'a>(uid: &[u8], buf: &'a mut [u8; 11]) -> &'a str {
    const TABLE: &[u8] = b"0123456789ABCDEF";

    let mut i = 0;
    while i < 4 {
        let b = uid[i];
        let pos = i * 3;
        buf[pos] = TABLE[(b >> 4) as usize];
        buf[pos + 1] = TABLE[(b & 0xF) as usize];
        if i < 3 {
            buf[pos + 2] = b':';
        }
        i += 1;
    }
    // SAFETY: buf is always valid ASCII from TABLE
    unsafe { from_utf8_unchecked(buf) }
}
