use std::error;
use std::fmt;

/// The error type for gemtext creation and parsing
#[derive(Debug)]
pub enum Error {
    InvalidContent,
}

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

impl error::Error for Error {}
