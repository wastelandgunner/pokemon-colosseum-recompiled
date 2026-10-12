//! `.msg` string tables (and the same layout embedded in other files).
//!
//! u16 count @0x04 (bytes 6..8 hold a region tag such as "US"), then `count` × {u32 id, u32 offset}
//! starting at 0x10. Strings are UTF-16BE, terminated by 0x0000, and may start at odd offsets.
//! `FF FF cc` introduces control code `cc`, followed by 0, 1 or 4 argument bytes (GoD Tool
//! `XGSpecialStringCharacters.swift`); text then continues as UTF-16BE from the next byte.

use crate::be::{u16_at, u32_at};
use crate::Result;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Text(String),
    Control { code: u8, args: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u32,
    pub offset: usize,
    pub tokens: Vec<Token>,
}

/// Number of argument bytes after control code `code`.
pub fn control_arg_len(code: u8) -> usize {
    match code {
        0x07 | 0x09 | 0x38 | 0x52 | 0x53 | 0x54 | 0x5B | 0x5C => 1,
        0x08 => 4,
        _ => 0,
    }
}

/// Human-readable name for common control codes (GoD Tool naming).
pub fn control_name(code: u8) -> Option<&'static str> {
    Some(match code {
        0x00 => "newline",
        0x02 => "end",
        0x03 => "clear",
        0x04 => "kanji",
        0x05 => "furigana",
        0x06 => "furigana_end",
        0x07 => "font",
        0x08 => "rgba",
        0x09 => "pause",
        0x13 => "player_battle",
        0x22 => "foe_class",
        0x23 => "foe_name",
        0x2B => "player",
        0x2C => "rui",
        0x38 => "color",
        0x4D => "msg",
        0x59 => "speaker",
        0x6A => "set_speaker",
        0x6D => "wait_key",
        _ => return None,
    })
}

pub fn parse_string(data: &[u8], mut p: usize) -> Vec<Token> {
    let mut toks = Vec::new();
    let mut text = String::new();
    while p + 1 < data.len() {
        let c = u16::from_be_bytes([data[p], data[p + 1]]);
        p += 2;
        match c {
            0x0000 => break,
            0xFFFF => {
                let Some(&code) = data.get(p) else { break };
                p += 1;
                let n = control_arg_len(code);
                let args = data.get(p..p + n).map(|s| s.to_vec()).unwrap_or_default();
                p += n;
                if !text.is_empty() { toks.push(Token::Text(std::mem::take(&mut text))); }
                toks.push(Token::Control { code, args });
            }
            _ => text.push(char::from_u32(c as u32).unwrap_or('\u{FFFD}')),
        }
    }
    if !text.is_empty() { toks.push(Token::Text(text)); }
    toks
}

pub fn parse_table(data: &[u8]) -> Result<Vec<Message>> {
    let count = u16_at(data, 4, "msg header")? as usize;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let id = u32_at(data, 0x10 + 8 * i, "msg table")?;
        let offset = u32_at(data, 0x14 + 8 * i, "msg table")? as usize;
        out.push(Message { id, offset, tokens: parse_string(data, offset) });
    }
    Ok(out)
}

impl Message {
    /// Render with control codes as `{name:args}` / `{xx:args}`; newline codes become '\n'.
    pub fn render(&self) -> String {
        let mut s = String::new();
        for t in &self.tokens {
            match t {
                Token::Text(x) => s.push_str(x),
                Token::Control { code: 0x00, .. } => s.push('\n'),
                Token::Control { code, args } => {
                    match control_name(*code) {
                        Some(n) => { let _ = write!(s, "{{{n}"); }
                        None => { let _ = write!(s, "{{{code:02x}"); }
                    }
                    for (i, a) in args.iter().enumerate() {
                        let _ = write!(s, "{}{a:02x}", if i == 0 { ':' } else { ' ' });
                    }
                    s.push('}');
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn control_then_odd_aligned_text() {
        let mut d = vec![0xFF, 0xFF, 0x59];
        for c in "Hi".encode_utf16() { d.extend_from_slice(&c.to_be_bytes()); }
        d.extend_from_slice(&[0, 0]);
        let t = parse_string(&d, 0);
        assert_eq!(t, vec![Token::Control { code: 0x59, args: vec![] }, Token::Text("Hi".into())]);
    }
}
