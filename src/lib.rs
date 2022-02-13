use http::uri::Uri;

mod content;
pub use content::Content;

mod link;
pub use link::Link;

mod preformatted;
pub use preformatted::Preformatted;

mod gemtext;
pub use gemtext::Gemtext;

mod html;
pub use html::Html;

#[derive(Debug, Clone)]
pub enum Block {
    Text(Content),
    Link(Link),
    Heading(Level, Content),
    ListItem(Content),
    Quote(Content),
    Preformatted(Preformatted),
    Empty,
}

/// Heading level
#[derive(Clone, Debug)]
pub enum Level {
    One,
    Two,
    Three,
}

/// A builder to create a document by [Block]
#[derive(Debug, Default, Clone)]
pub struct DocBuilder(Vec<Block>);

macro_rules! setter {
    ($func:ident($($param:ident: $ty:ty),*), $block:block) => {
        pub fn $func(
            &mut self,
            $($param: $ty),*
        ) -> &mut Self {
            let block: Block = $block;
            self.push(block);
            self
        }
    };
}

macro_rules! try_into_setter {
    ($func:ident($($param:ident: $ty:ty),*), $block:block) => {
        pub fn $func<T: TryInto<Content>>(
            &mut self,
            $($param: $ty),+
        ) -> Result<&mut Self, <T as std::convert::TryInto<Content>>::Error> {
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

    // TODO what is a more practical example?
    /// Returns [Block]s without consuming the builder
    ///
    /// ```
    /// # use mu_lines::DocBuilder;
    /// let mut builder = DocBuilder::new();
    /// let homepage = builder.h1("my site").unwrap().build();
    /// let article = builder.h2("my article").unwrap().build();
    /// # assert_eq!(homepage.len(), 1);
    /// # assert_eq!(article.len(), 2);
    /// ```
    pub fn build(&self) -> Vec<Block> {
        self.0.clone()
    }
}

// TODO is format() too similar to format! macro
/// Format [Block]s
pub fn format<F>(blocks: &[Block]) -> String
where
    F: FormatBlocks,
{
    <F>::format(blocks.iter())
}

/// Format an iterator of [Block]s
pub trait FormatBlocks {
    fn format<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String;
}
