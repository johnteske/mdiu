use crate::Error;
use std::fmt;

/// Text content without newline characters
///
/// Newline characters delineate [`Block`]s in gemtext. With the exception of
/// [`Block::Preformatted`], the contents of a [`Block`] should be free of newline
/// characters to guarantee the expected output.
///
/// [`Content`] should primarily be created through a conversion that checks for
/// `\n` and `\r` characters:
///
/// ```
/// # use mdiu::*;
/// # fn main() -> Result<()> {
/// let content: Content = "some text".try_into()?;
/// # Ok(())
/// # }
/// ```
///
/// If the text is known to be free of newline characters, [`Content`] can be
/// created without checking using [`new_unchecked`].
///
/// [`Block`]: crate::Block
/// [`Block::Preformatted`]: crate::Block::Preformatted
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
    /// # use mdiu::Content;
    /// let content = Content::new_unchecked("smth");
    /// ```
    /// [`Gemtext`]: crate::Gemtext
    pub fn new_unchecked<T: Into<String>>(value: T) -> Self {
        Content(value.into())
    }

    pub fn validate(&self) -> crate::Result<&str> {
        validate(&self.0)
    }
}

impl TryFrom<String> for Content {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _ = validate(&value)?;
        Ok(Content(value))
    }
}

impl TryFrom<&str> for Content {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value).map(|value| Content(value.to_string()))
    }
}

impl From<Content> for String {
    fn from(value: Content) -> String {
        value.0
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate(text: &str) -> crate::Result<&str> {
    if text.contains(&['\n', '\r']) {
        Err(Error::InvalidContent)
    } else {
        Ok(text)
    }
}
