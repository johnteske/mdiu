use crate::{Block, Content, Level, Link, Preformatted};
use http::uri::Uri;

/// A builder to create a document with [Block]s
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # use mu_lines::{DocBuilder, Block};
/// let builder = DocBuilder::new().h1("my site")?;
/// let v: Vec<Block> = builder.build();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct DocBuilder(Vec<Block>);

macro_rules! setter {
    ($func:ident($($param:ident: $ty:ty),*), $block:block) => {
        pub fn $func(
            mut self,
            $($param: $ty),*
        ) -> Self {
            let block: Block = $block;
            self.push(block);
            self
        }
    };
}

macro_rules! try_into_setter {
    ($func:ident($($param:ident: $ty:ty),*), $block:block) => {
        pub fn $func<T: TryInto<Content>>(
            mut self,
            $($param: $ty),+
        ) -> Result<Self, <T as std::convert::TryInto<Content>>::Error> {
            let block: Block = $block;
            self.push(block);
            Ok(self)
        }
    };
}

impl DocBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, block: Block) -> &mut Self {
        self.0.push(block);
        self
    }

    try_into_setter!(text(text: T), {
        let content = text.try_into()?;
        Block::Text(content)
    });

    setter!(link(uri: Uri), { Block::Link(Link::new(uri, None)) });
    try_into_setter!(link_with_label(uri: Uri, label: T), {
        let label = label.try_into()?;
        Block::Link(Link::new(uri, Some(label)))
    });

    try_into_setter!(h1(text: T), {
        let content = text.try_into()?;
        Block::Heading(Level::One, content)
    });

    try_into_setter!(h2(text: T), {
        let content = text.try_into()?;
        Block::Heading(Level::Two, content)
    });

    try_into_setter!(h3(text: T), {
        let content = text.try_into()?;
        Block::Heading(Level::Three, content)
    });

    try_into_setter!(list_item(text: T), {
        let content = text.try_into()?;
        Block::ListItem(content)
    });

    try_into_setter!(quote(text: T), {
        let content = text.try_into()?;
        Block::Quote(content)
    });

    setter!(preformatted(text: String), {
        Block::Preformatted(Preformatted::new(text, None))
    });
    try_into_setter!(preformatted_with_alt(text: String, alt: T), {
        let alt = alt.try_into()?;
        Block::Preformatted(Preformatted::new(text, Some(alt)))
    });

    setter!(empty(), { Block::Empty });

    /// Consumes the builder and return [Block]s
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # use mu_lines::{DocBuilder, format, Gemtext};
    /// let mut builder = DocBuilder::new();
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

//impl From<DocBuilder> for Vec<Block> {
//    fn from(builder: DocBuilder) -> Self {
//        // TODO
//        //vec![]
//        DocBuilder.0
//    }
//}
//impl From<Vec<Block>> for DocBuilder {
//    fn from(vec: Vec<Block>) -> Self {
//        DocBuilder(vec)
//    }
//}
