use std::fmt;

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
