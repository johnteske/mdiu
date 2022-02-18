use crate::{Block, Content, Level, Link, Preformatted};
use http::uri::Uri;

/// A builder to create a document with [`Block`]s
///
/// TODO..that tries to convert input into [`Content`]
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # use mu_lines::{Document, Block};
/// let builder = Document::new().h1("my site")?;
/// let v: Vec<Block> = builder.build();
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
        pub fn $func<T: TryInto<Content>>(
            mut self,
            $($param: $ty),*
        ) -> Result<Self, <T as std::convert::TryInto<Content>>::Error> {
            self.push($block);
            Ok(self)
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
            let content = text.try_into()?;
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
            Block::Link(Link::new(uri, Some(label.try_into()?)))
        }
    }

    setter! {
        pub fn h1(text: T)? {
            Block::Heading(Level::One, text.try_into()?)
        }
    }

    setter! {pub fn h2(text: T)? {
        let content = text.try_into()?;
        Block::Heading(Level::Two, content)
    }}

    setter! {pub fn h3(text: T)? {
        Block::Heading(Level::Three, text.try_into()?)
    }}

    setter! {
        pub fn list_item(text: T)? {
            Block::ListItem(text.try_into()?)
        }
    }

    setter! {
        pub fn quote(text: T)? {
            let content = text.try_into()?;
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
            let alt = alt.try_into()?;
            Block::Preformatted(Preformatted::new(text, Some(alt)))
        }
    }

    setter! {
        /// Appends an empty line
        pub fn empty() {
            Block::Empty
        }
    }

    /// Consumes the builder and return [Block]s
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::{Document, format, Gemtext};
    /// let mut builder = Document::new();
    /// builder = builder.h1("my site")?;
    /// let homepage = builder.clone().build();
    /// let article = builder.h2("my article")?.build();
    /// assert_eq!(format::<Gemtext>(&homepage), "# my site\n");
    /// assert_eq!(format::<Gemtext>(&article), "# my site\n## my article\n");
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Vec<Block> {
        self.0
    }
}

//impl From<Document> for Vec<Block> {
//    fn from(builder: Document) -> Self {
//        // TODO
//        //vec![]
//        Document.0
//    }
//}
//impl From<Vec<Block>> for Document {
//    fn from(vec: Vec<Block>) -> Self {
//        Document(vec)
//    }
//}
