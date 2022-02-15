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
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::Preformatted;
    /// let pre = Preformatted::new("@_@".to_string(), Some("emoticon".try_into()?));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(text: String, alt: Option<Content>) -> Self {
        Preformatted { text, alt }
    }

    /// Constructs a new `Preformatted` with alt text
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::Preformatted;
    /// let pre = Preformatted::with_alt("@_@".to_string(), "emoticon".try_into()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_alt(text: String, alt: Content) -> Self {
        Preformatted::new(text, Some(alt))
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
