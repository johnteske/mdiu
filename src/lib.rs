//! Build documents with [gemtext]
//!
//! Named after the [Manual Data Insertion Unit], part of Gemini's on-board computer.
//!
//! # Features
//! - `html`
//! - `markdown`
//!
//! [gemtext]: https://gemini.circumlunar.space/docs/gemtext.gmi
//! [Manual Data Insertion Unit]: https://web.archive.org/web/20220201083102/https://www.ibiblio.org/apollo/Gemini.html

mod content;
pub use content::Content;

mod document;
pub use document::Document;

mod error;
pub use error::{Error, Result};

mod link;
pub use link::Link;

mod list_state;

mod preformatted;
pub use preformatted::Preformatted;

mod gemtext;
pub use gemtext::Gemtext;

#[cfg(feature = "html")]
mod html;
#[cfg(feature = "html")]
pub use html::Html;

#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "markdown")]
pub use markdown::Markdown;

/// A Gemtext element
#[derive(Clone, Debug)]
pub enum Block {
    Text(Content),
    Link(Link),
    Heading(Level, Content),
    ListItem(Content),
    Quote(Content),
    Preformatted(Preformatted),
    Empty,
}

/// Heading level of [`Block::Heading`]
#[derive(Clone, Debug)]
pub enum Level {
    One,
    Two,
    Three,
}

/// Format an iterator of [`Block`]s
pub trait Markup {
    // This takes an iter of Blocks so the formatter can handle adjacent Blocks,
    // for example wrapping lists with <ul> in HTML
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String;
}

// TODO create_markup?
/// Creates a [`Markup`]-formatted [`String`]
///
/// ```
/// # fn main() -> mdiu::Result<()> {
/// use mdiu::{Document, Gemtext, markup};
/// let doc = Document::new().empty().build()?;
/// let gemtext = markup::<Gemtext>(&doc);
/// # assert_eq!("\n".to_string(), gemtext);
/// # Ok(())
/// # }
/// ```
///
/// [`ToMarkup`] is the trait equivalent of this function.
pub fn markup<F>(blocks: &[Block]) -> String
where
    F: Markup,
{
    <F>::markup(blocks.iter())
}

/// Create [`Markup`]-formatted strings
///
/// *Note: this trait is sealed and cannot be implemented outside of mdiu.*
///
/// [`markup`] is the function equivalent of this trait.
///
/// ```
/// # fn main() -> mdiu::Result<()> {
/// use mdiu::{Block, Gemtext, ToMarkup};
/// let slice = &[Block::Empty];
/// let gemtext = slice.to_markup::<Gemtext>();
/// # assert_eq!("\n".to_string(), gemtext);
/// # Ok(())
/// # }
///
/// ```
pub trait ToMarkup: private::Sealed {
    fn to_markup<F>(self) -> String
    where
        F: Markup;
}

impl<'a, T> ToMarkup for T
where
    T: IntoIterator<Item = &'a Block>,
{
    fn to_markup<F>(self) -> String
    where
        F: Markup,
    {
        <F>::markup(self.into_iter())
    }
}

mod private {
    use super::Block;

    pub trait Sealed {}

    impl<'a, T> Sealed for T where T: IntoIterator<Item = &'a Block> {}
}
