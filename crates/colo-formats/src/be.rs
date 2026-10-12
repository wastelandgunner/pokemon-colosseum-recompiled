//! Bounds-checked big-endian readers over byte slices.

use crate::{Error, Result};

#[inline]
fn slice<'a>(d: &'a [u8], off: usize, n: usize, what: &'static str) -> Result<&'a [u8]> {
    d.get(off..off.checked_add(n).ok_or_else(|| Error::Invalid(format!("{what}: offset overflow")))?)
        .ok_or(Error::Truncated { what, offset: off, need: n, have: d.len().saturating_sub(off) })
}

pub fn u8_at(d: &[u8], off: usize, what: &'static str) -> Result<u8> {
    Ok(slice(d, off, 1, what)?[0])
}
pub fn u16_at(d: &[u8], off: usize, what: &'static str) -> Result<u16> {
    let s = slice(d, off, 2, what)?;
    Ok(u16::from_be_bytes([s[0], s[1]]))
}
pub fn u32_at(d: &[u8], off: usize, what: &'static str) -> Result<u32> {
    let s = slice(d, off, 4, what)?;
    Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}
pub fn bytes_at<'a>(d: &'a [u8], off: usize, n: usize, what: &'static str) -> Result<&'a [u8]> {
    slice(d, off, n, what)
}
/// NUL-terminated byte string starting at `off` (lossy Latin-1/ASCII decode).
pub fn cstr_at(d: &[u8], off: usize, what: &'static str) -> Result<String> {
    let tail = d.get(off..).ok_or(Error::Truncated { what, offset: off, need: 1, have: 0 })?;
    let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
    Ok(tail[..end].iter().map(|&b| b as char).collect())
}
