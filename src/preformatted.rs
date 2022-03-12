use crate::Content;

/// Preformatted text with optional alt text
#[derive(Clone, Debug)]
pub struct Preformatted {
    text: String,
    alt: Option<Content>,
}

impl Preformatted {
    /// Constructs a new `Preformatted`
    ///
    /// ```
    /// # use mdiu::*;
    /// # fn main() -> Result<()> {
    /// let pre = Preformatted::new("@_@".to_string(), Some("emoticon".try_into()?));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: Into<String>>(text: T, alt: Option<Content>) -> Self {
        Preformatted {
            text: text.into(),
            alt,
        }
    }

    /// Returns a reference to the text
    pub fn text(&self) -> &String {
        &self.text
    }
    /// Returns a mutable reference to the text
    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    /// Returns a reference to the alt text
    pub fn alt(&self) -> &Option<Content> {
        &self.alt
    }
    /// Returns a mutable reference to the alt text
    pub fn alt_mut(&mut self) -> &mut Option<Content> {
        &mut self.alt
    }
}
