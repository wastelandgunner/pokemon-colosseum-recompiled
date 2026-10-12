use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// Data is shorter than a structure requires.
    Truncated { what: &'static str, offset: usize, need: usize, have: usize },
    /// Magic bytes or a header field did not match.
    BadMagic { what: &'static str, found: [u8; 4] },
    Invalid(String),
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Truncated { what, offset, need, have } => {
                write!(f, "{what}: need {need} bytes at {offset:#x}, have {have}")
            }
            Error::BadMagic { what, found } => write!(f, "{what}: bad magic {found:02X?}"),
            Error::Invalid(s) => write!(f, "invalid data: {s}"),
            Error::NotFound(s) => write!(f, "not found: {s}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
