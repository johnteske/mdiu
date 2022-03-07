use std::error;
use std::fmt;

/// Error type that can be returned during gemtext creation
#[derive(Debug)]
pub enum Error {
    InvalidContent,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidContent => write!(
                f,
                "Conversion into Content failed: input contains newline characters"
            ),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
