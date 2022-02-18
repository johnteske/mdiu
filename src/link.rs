use crate::Content;
use http::uri::Uri;

/// A URI with optional label
#[derive(Clone, Debug)]
pub struct Link {
    uri: Uri,
    label: Option<Content>,
}

impl Link {
    /// Constructs a new [`Link`]
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::Link;
    /// use http::uri::Uri;
    /// let link = Link::new(Uri::from_static("index.gmi"), Some("my gemlog".try_into()?));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(uri: Uri, label: Option<Content>) -> Self {
        Link { uri, label }
    }

    /// Returns a reference to the URI
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
    /// Returns a mutable reference to the URI
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    /// Returns a reference to the label
    pub fn label(&self) -> &Option<Content> {
        &self.label
    }
    /// Returns a mutable reference to the label
    pub fn label_mut(&mut self) -> &mut Option<Content> {
        &mut self.label
    }
}

/// ```
/// # use mu_lines::Link;
/// use http::uri::Uri;
/// let uri = Uri::from_static("index.gmi");
/// let link = Link::from(uri);
/// ```
impl From<Uri> for Link {
    fn from(uri: Uri) -> Link {
        Link::new(uri, None)
    }
}
