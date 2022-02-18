use crate::Error;
use std::fmt;

/// A piece of text without newline characters
///
/// As newline characters delineate [`Block`]s in gemtext, content should be free of them.
///
/// [`Content`] should primarily be created through a conversion that checks for
/// `\n` and `\r` characters:
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # use mu_lines::Content;
/// let content: Content = "some text".try_into()?;
/// # Ok(())
/// # }
/// ```
///
/// If the text is known to be free of newline characters, [`Content`] can be created without
/// checking using [`new_unchecked`].
///
/// [`Block`]: crate::Block
/// [`new_unchecked`]: #method.new_unchecked
#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    /// Constructs a new [`Content`] without checking the input
    ///
    /// This method is not unsafe but its correctness is not guaranteed,
    /// especially in the case of formatting as [`Gemtext`].
    ///
    /// ```
    /// # use mu_lines::Content;
    /// let content = Content::new_unchecked("smth");
    /// ```
    /// [`Gemtext`]: crate::Gemtext
    pub fn new_unchecked<T: Into<String>>(value: T) -> Self {
        Content(value.into())
    }
}

impl TryFrom<String> for Content {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_text(&value).map(|_| Content(value))
    }
}

impl TryFrom<&str> for Content {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_text(value).map(|_| Content(value.to_string()))
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate_text(text: &str) -> Result<(), Error> {
    if text.contains(&['\n', '\r']) {
        Err(Error::InvalidContent)
    } else {
        Ok(())
    }
}
