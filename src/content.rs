use crate::Error;
use std::fmt;

/// Text that is known to be non-empty and without newline characters
///
/// `Content` should be free of newline characters as
/// lines delineate blocks in Gemtext
/// (except within preformatted blocks).
/// To be meaningful, `Content` should not be empty.
///
/// # Example
///
/// ```
/// # fn main() -> mdiu::Result<()> {
/// use mdiu::{Block, Content};
///
/// let content: Content = "my gemlog".parse()?;
/// let text = Block::Text(content);
/// # Ok(())
/// # }
/// ```
///
/// [`Block`]: crate::Block
/// [`Block::Preformatted`]: crate::Block::Preformatted
#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    /// Constructs a new, checked `Content`
    ///
    /// # Errors
    /// If the input is empty or contains newline characters, an [`Error`] will be returned.
    ///
    /// # Example
    /// ```
    /// # fn main() -> mdiu::Result<()> {
    /// # use mdiu::Content;
    /// let content = Content::new("some text")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        value.as_ref().parse()
    }

    /// Constructs a new, unchecked `Content`
    ///
    /// This results in undefined behavior if the value is invalid.
    ///
    /// # Safety
    /// The value must not be empty and not contain newline characters.
    ///
    /// # Example
    ///
    /// ```
    /// # use mdiu::Content;
    /// let content = unsafe {
    ///     Content::new_unchecked("some text")
    /// };
    /// assert_eq!(Ok(()), content.validate());
    /// ```
    // This is unsafe/can cause undefined behavior in the same way core::num::NonZero* can
    pub unsafe fn new_unchecked<T: Into<String>>(value: T) -> Self {
        Content(value.into())
    }

    /// Validates `Content`
    ///
    /// # Errors
    /// If the input is empty or contains newline characters, an [`Error`] will be returned.
    ///
    /// # Example
    /// Check content after creating with [`new_unchecked`]
    /// ```
    /// # fn main() -> mdiu::Result<()> {
    /// # use mdiu::Content;
    /// let content = unsafe {
    ///     Content::new_unchecked("some text")
    /// };
    /// assert_eq!(Ok(()), content.validate());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`new_unchecked`]: #method.new_unchecked
    pub fn validate(&self) -> crate::Result<()> {
        validate(&self.0)
    }
}

use std::str::FromStr;
impl FromStr for Content {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate(s)?;
        Ok(Content(s.to_string()))
    }
}

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
    if text.is_empty() {
        return Err(Error::EmptyContent);
    }

    if text.contains(&['\n', '\r']) {
        return Err(Error::InvalidContent);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn errors() {
        assert_eq!(
            "my\r\ngemlog".parse::<Content>().err(),
            Some(Error::InvalidContent)
        );
        assert_eq!(
            "my\ngemlog".parse::<Content>().err(),
            Some(Error::InvalidContent)
        );
        assert_eq!("".parse::<Content>().err(), Some(Error::EmptyContent));
    }
}
