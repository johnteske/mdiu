use crate::{Block, Content, Level, Link, Preformatted, Result};
use http::uri::Uri;

/// A document builder
///
/// # Example
///
/// See [crate documentation](./index.html#examples).
#[derive(Debug, Default, Clone)]
pub struct Document(Vec<Block>);

macro_rules! setter {
    (
        $(#[$attr:meta])*
        pub fn $func:ident($($param:ident: $ty:ty),*) $block:block
    ) => {
        $(#[$attr])*
        pub fn $func(mut self, $($param: $ty),*) -> Self {
            self.0.push($block);
            self
        }
    };
    (
        $(#[$attr:meta])*
        pub fn $func:ident<T>($($param:ident: $ty:ty),*) $block:block
    ) => {
        $(#[$attr])*
        pub fn $func<T>(mut self, $($param: $ty),*) -> Self
        where T: Into<String> {
            self.0.push($block);
            self
        }
    };
}

impl Document {
    /// Constructs a new `Document`
    pub fn new() -> Self {
        Self::default()
    }

    setter! {
        /// Appends text
        pub fn text<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::Text(content)
        }
    }

    setter! {
        /// Appends a link
        pub fn link(uri: Uri) {
            Block::Link(Link::new(uri, None))
        }
    }
    setter! {
        /// Appends a link with label
        pub fn link_with_label<T>(uri: Uri, label: T) {
            let label = unsafe { Content::new_unchecked(label) };
            Block::Link(Link::new(uri, Some(label)))
        }
    }

    setter! {
        /// Appends a heading
        pub fn h1<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::Heading(Level::One, content)
        }
    }

    setter! {
        /// Appends a subheading
        pub fn h2<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::Heading(Level::Two, content)
        }
    }

    setter! {
        /// Appends a sub-subheading
        pub fn h3<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::Heading(Level::Three, content)
        }
    }

    setter! {
        /// Appends a list item
        pub fn list_item<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::ListItem(content)
        }
    }

    setter! {
        /// Appends a blockquote
        pub fn quote<T>(text: T) {
            let content = unsafe { Content::new_unchecked(text) };
            Block::Quote(content)
        }
    }

    setter! {
        /// Appends preformatted text
        pub fn preformatted<T>(text: T) {
            Block::Preformatted(Preformatted::new(text, None))
        }
    }
    setter! {
        /// Appends preformatted text with alt text
        pub fn preformatted_with_alt<T>(text: T, alt: T) {
            let alt = unsafe { Content::new_unchecked(alt) };
            Block::Preformatted(Preformatted::new(text, Some(alt)))
        }
    }

    setter! {
        /// Appends an empty line
        pub fn empty() {
            Block::Empty
        }
    }

    /// Validates the [`Content`] in a `Document`
    ///
    /// # Errors
    /// If any [`Content`] is empty or contains newline characters, an [`Error`] will be returned.
    ///
    /// [`Error`]: crate::Error
    pub fn validate(&self) -> Result<()> {
        self.0.iter().try_for_each(|block| match block {
            Block::Text(content) => content.validate(),
            Block::Link(link) => link.label().as_ref().map_or(Ok(()), |c| c.validate()),
            Block::Heading(_, content) => content.validate(),
            Block::ListItem(content) => content.validate(),
            Block::Quote(content) => content.validate(),
            Block::Preformatted(pre) => pre.alt().as_ref().map_or(Ok(()), |c| c.validate()),
            Block::Empty => Ok(()),
        })
    }

    /// Consumes the builder, returning [`Block`]s if valid
    ///
    /// # Errors
    /// Returns an [`Error`] if the internal [`validate`] call failed.
    ///
    /// # Example
    /// Clone the builder to reuse
    /// ```
    /// # fn main() -> mdiu::Result<()> {
    /// # use mdiu::*;
    /// let mut builder = Document::new();
    /// builder = builder.h1("my site");
    ///
    /// let homepage = builder.clone().build()?;
    /// assert_eq!(&homepage.to_markup::<Gemtext>(), "# my site\n");
    ///
    /// let article = builder.h2("my article").build()?;
    /// assert_eq!(&article.to_markup::<Gemtext>(), "# my site\n## my article\n");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error`]: crate::Error
    /// [`validate`]: #method.validate
    pub fn build(self) -> Result<Vec<Block>> {
        self.validate()?;
        Ok(self.0)
    }
}
