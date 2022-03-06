use crate::{Block, Content, Error, Level, Link, Preformatted};
use http::uri::Uri;

/// A builder to create a document with [`Block`]s
///
/// Where [`Content`] is expected, setters use `TryInto<Content>` for convenience.
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # use mu_lines::{Document, Content, Block};
/// let text: Content = "some text".try_into()?;
/// let builder = Document::new().h1("my site").text(text);
/// let v: Vec<Block> = builder.build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Document(Vec<Block>);

macro_rules! setter {
    ($(#[doc = $doc:expr])* pub fn $func:ident($($param:ident: $ty:ty),*) $block:block) => {
        $(#[doc = $doc])*
        pub fn $func(
            mut self,
            $($param: $ty),*
        ) -> Self {
            self.push($block);
            self
        }
    };
    ($(#[doc = $doc:expr])* pub fn $func:ident($($param:ident: $ty:ty),*)? $block:block) => {
        $(#[doc = $doc])*
        pub fn $func<T>(
            mut self,
            $($param: $ty),*
        ) -> Self
        where T: TryInto<Content> + Into<String> {
            self.push($block);
            self
        }
    };
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, block: Block) -> &mut Self {
        self.0.push(block);
        self
    }

    setter! {
        /// Appends text, returns an error if text contains newlines
        pub fn text(text: T)? {
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
        pub fn link_with_label(uri: Uri, label: T)? {
            let label = Content::new_unchecked(label);
            Block::Link(Link::new(uri, Some(label)))
        }
    }

    setter! {
        pub fn h1(text: T)? {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::One, content)
        }
    }

    setter! {
        pub fn h2(text: T)? {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::Two, content)
        }
    }

    setter! {
        pub fn h3(text: T)? {
            let content = Content::new_unchecked(text);
            Block::Heading(Level::Three, content)
        }
    }

    setter! {
        pub fn list_item(text: T)? {
            let content = Content::new_unchecked(text);
            Block::ListItem(content)
        }
    }

    setter! {
        pub fn quote(text: T)? {
            let content = Content::new_unchecked(text);
            Block::Quote(content)
        }
    }

    setter! {
        /// Appends preformatted text
        pub fn preformatted(text: String) {
            Block::Preformatted(Preformatted::new(text, None))
        }
    }
    setter! {
        /// Appends preformatted text with alt text
        pub fn preformatted_with_alt(text: String, alt: T)? {
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

    pub fn validate(&self) -> Result<(), Error> {
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
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::{Document, Gemtext, ToMarkup};
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
    pub fn build(self) -> Result<Vec<Block>, Error> {
        self.validate()?;
        Ok(self.0)
    }
}
