use std::fmt;

#[derive(Debug)]
pub enum ItemsDatError {
    Io(std::io::Error),
    UnexpectedEof,
    ParseError(String),
    DecompressionError(String),
}

impl fmt::Display for ItemsDatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemsDatError::Io(e) => write!(f, "IO error: {e}"),
            ItemsDatError::UnexpectedEof => write!(f, "unexpected end of file"),
            ItemsDatError::ParseError(msg) => write!(f, "parse error: {msg}"),
            ItemsDatError::DecompressionError(msg) => write!(f, "decompression error: {msg}"),
        }
    }
}

impl std::error::Error for ItemsDatError {}

impl From<std::io::Error> for ItemsDatError {
    fn from(e: std::io::Error) -> Self {
        ItemsDatError::Io(e)
    }
}
