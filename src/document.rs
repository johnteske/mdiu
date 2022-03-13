use crate::{Block, Content, Level, Link, Preformatted, Result};
use http::uri::Uri;

/// A builder to create a document with [`Block`]s
///
/// Where [`Content`] is expected, setters use `TryInto<Content>` for convenience.
///
/// ```
/// # use mdiu::*;
/// # fn main() -> Result<()> {
/// let text: Content = "some text".try_into()?;
/// let builder = Document::new().h1("my site").text(text);
/// let v: Vec<Block> = builder.build()?;
/// # Ok(())
/// # }
/// ```
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
    pub fn new() -> Self {
        Self::default()
    }

    setter! {
        /// Appends text, returns an error if text contains newlines
        pub fn text<T>(text: T) {
            let content = Content::new_unchecked(text);
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
            let label = Content::new_unchecked(label);
            Block::Link(Link::new(uri, Some(label)))
        }
    }

    setter! {
        pub fn h1<T>(text: T) {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::One, content)
        }
    }

    setter! {
        pub fn h2<T>(text: T) {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::Two, content)
        }
    }

    setter! {
        pub fn h3<T>(text: T) {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::Three, content)
        }
    }

    setter! {
        pub fn list_item<T>(text: T) {
            let content = Content::new_unchecked(text);
            Block::ListItem(content)
        }
    }

    setter! {
        pub fn quote<T>(text: T) {
            let content = Content::new_unchecked(text);
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
            let alt = Content::new_unchecked(alt);
            Block::Preformatted(Preformatted::new(text, Some(alt)))
        }
    }

    setter! {
        /// Appends an empty line
        pub fn empty() {
            Block::Empty
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.0.iter().try_for_each(|block| {
            let _ = match block {
                Block::Text(content) => content.validate(),
                Block::Link(link) => link.label().as_ref().map_or(Ok(""), |c| c.validate()),
                Block::Heading(_, content) => content.validate(),
                Block::ListItem(content) => content.validate(),
                Block::Quote(content) => content.validate(),
                Block::Preformatted(pre) => pre.alt().as_ref().map_or(Ok(""), |c| c.validate()),
                _ => Ok(""),
            };
            Ok(())
        })
    }

    /// Consumes the builder and return [Block]s
    ///
    /// ```
    /// # use std::error::Error;
    /// # use mdiu::*;
    /// # fn main() -> Result<()> {
    /// let mut builder = Document::new();
    /// builder = builder.h1("my site");
    /// // Explicitly clone the builder to reuse
    /// let homepage = builder.clone().build()?;
    /// let article = builder.h2("my article").build()?;
    /// assert_eq!(&homepage.to_markup::<Gemtext>(), "# my site\n");
    /// assert_eq!(&article.to_markup::<Gemtext>(), "# my site\n## my article\n");
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<Vec<Block>> {
        self.validate()?;
        Ok(self.0)
    }
}
