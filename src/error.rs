use std::error;
use std::fmt;

/// Error type that can be returned during gemtext creation
#[derive(Debug)]
pub enum Error {
    InvalidContent,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidContent => {
                write!(f, "invalid Content: contains newline characters")
            }
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
