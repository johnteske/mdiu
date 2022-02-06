#[derive(Clone, Debug)]
pub struct Preformatted {
    text: String,
    alt: Option<String>,
}

impl Preformatted {
    /// Constructs a new `Preformatted`
    ///
    /// ```
    /// use mu_lines::Preformatted;
    /// let pre = Preformatted::new("@_@".to_string(), Some("emoticon".to_string()));
    /// ```
    pub fn new(text: String, alt: Option<String>) -> Self {
        Preformatted { text, alt }
    }

    /// Constructs a new `Preformatted` with alt text
    ///
    /// ```
    /// use mu_lines::Preformatted;
    /// let pre = Preformatted::with_alt("@_@".to_string(), "emoticon".to_string());
    /// ```
    pub fn with_alt(text: String, alt: String) -> Self {
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
    pub fn alt(&self) -> &Option<String> {
        &self.alt
    }
    /// Returns a mutable reference to the alt text
    pub fn alt_mut(&mut self) -> &mut Option<String> {
        &mut self.alt
    }
}
