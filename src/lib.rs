mod builder;
pub use builder::DocBuilder;

mod content;
pub use content::Content;

mod error;
pub use error::Error;

mod link;
pub use link::Link;

mod preformatted;
pub use preformatted::Preformatted;

mod gemtext;
pub use gemtext::Gemtext;

mod html;
pub use html::Html;

/// Gemtext elements
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
