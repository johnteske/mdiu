use http::uri::Uri;

#[derive(Clone, Debug)]
pub struct Link {
    uri: Uri,
    label: Option<String>,
}

impl Link {
    /// Constructs a new `Link`
    ///
    /// ```
    /// use mu_lines::Link;
    /// use http::uri::Uri;
    /// let link = Link::new(Uri::from_static("index.gmi"), Some("my gemlog".to_string()));
    /// ```
    pub fn new(uri: Uri, label: Option<String>) -> Self {
        Link { uri, label }
    }

    /// Constructs a new `Link` with a label
    ///
    /// ```
    /// use mu_lines::Link;
    /// use http::uri::Uri;
    /// let link = Link::with_label(Uri::from_static("index.gmi"), "my gemlog".to_string());
    /// ```
    pub fn with_label(uri: Uri, label: String) -> Self {
        Link {
            uri,
            label: Some(label),
        }
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
    pub fn label(&self) -> &Option<String> {
        &self.label
    }
    /// Returns a mutable reference to the label
    pub fn label_mut(&mut self) -> &mut Option<String> {
        &mut self.label
    }
}

/// ```
/// use mu_lines::Link;
/// use http::uri::Uri;
/// let uri = Uri::from_static("index.gmi");
/// let link = Link::from(uri);
/// ```
impl From<Uri> for Link {
    fn from(uri: Uri) -> Link {
        Link::new(uri, None)
    }
}
