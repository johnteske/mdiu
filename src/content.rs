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
/// Constructs a document with checked `Content`
/// ```
/// # fn main() -> mdiu::Result<()> {
/// use mdiu::{Block, Content, Gemtext, Level, Markup};
///
/// let content = Content::new("my gemlog")?;
/// let h1 = Block::Heading(Level::One, content);
///
/// // Content can also be created with try_into
/// let text = Block::Text("welcome".try_into()?);
///
/// let blocks = vec![h1, text];
/// let gemlog = <Gemtext>::markup(blocks.iter());
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
    pub fn new<T: TryInto<Content>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
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

// Due to
// https://github.com/rust-lang/rust/issues/50133#issuecomment-64690839
// impl<T> TryFrom<T> for Content where T: Into<String>,

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

// Allow Content to be passed in to builder,
// although that process is inefficient,
// converting Content to String to Content again.
impl From<Content> for String {
    fn from(value: Content) -> String {
        value.0
    }
}

/// ```
/// # use mdiu::*;
/// let content = Content::new("gemblog").unwrap();
/// let mut b = String::from("my ");
/// b += content.as_ref();
/// assert_eq!(b, "my gemblog");
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
    if text.is_empty() {
        return Err(Error::EmptyContent);
    }

    if text.contains(&['\n', '\r']) {
        return Err(Error::InvalidContent);
    }

    Ok(())
}
