use std::error;
use std::fmt;

/// Gemtext creation errors
#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyContent,
    InvalidContent,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyContent => {
                write!(f, "invalid Content: contains newline characters")
            }
            Error::InvalidContent => {
                write!(f, "invalid Content: contains newline characters")
            }
        }
    }
}

impl error::Error for Error {}

/// A wrapper around [`Error`]
pub type Result<T> = std::result::Result<T, Error>;
