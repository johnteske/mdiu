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
/// If you are sure that the text is free of newline characters,
/// [`Content`] can be created without checking using [`new_unchecked`].
///
/// [`Block`]: crate::Block
/// [`Block::Preformatted`]: crate::Block::Preformatted
/// [`new_unchecked`]: #method.new_unchecked
#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    pub fn new<T: TryInto<Content>>(value: T) -> crate::Result<Self> {
        value.try_into().map_err(|_| Error::InvalidContent)
    }
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

    pub fn validate(&self) -> crate::Result<()> {
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
        let _ = validate(value)?;
        Ok(Content(value.to_string()))
    }
}

impl From<Content> for String {
    fn from(value: Content) -> String {
        value.0
    }
}

/// ```
/// # use mdiu::*;
/// let content = Content::new("yes").unwrap();
/// let mut b = String::new();
/// b += content.as_ref();
/// assert_eq!(b, "yes");
/// ```
impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn validate(text: &str) -> crate::Result<()> {
    if text.contains(&['\n', '\r']) {
        Err(Error::InvalidContent)
    } else {
        Ok(())
    }
}
