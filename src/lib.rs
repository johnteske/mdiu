//! Build documents with [gemtext]
//!
//! # Features
//! - `html`
//! - `markdown`
//!
//! [gemtext]: https://gemini.circumlunar.space/docs/gemtext.gmi

mod builder;
pub use builder::Document;

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

#[cfg(feature = "html")]
mod html;
#[cfg(feature = "html")]
pub use html::Html;

#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "markdown")]
pub use markdown::Markdown;

/// A Gemtext element
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

/// Heading level of [`Block::Heading`]
#[derive(Debug, Clone)]
pub enum Level {
    One,
    Two,
    Three,
}

/// Format an iterator of [`Block`]s
pub trait Markup {
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String;
}

// TODO create_markup?
/// Creates a [`Markup`]-formatted [`String`]
///
/// ```
/// use mu_lines::{Document, Gemtext, markup};
/// let doc = Document::new().empty().build();
/// let gemtext = markup::<Gemtext>(&doc);
/// # assert_eq!("\n".to_string(), gemtext);
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
/// *Note: this trait is sealed and cannot be implemented outside of mu_lines.*
///
/// [`markup`] is the function equivalent of this trait.
///
/// ```
/// use mu_lines::{Block, Gemtext, ToMarkup};
/// let slice = &[Block::Empty];
/// let gemtext = slice.to_markup::<Gemtext>();
/// # assert_eq!("\n".to_string(), gemtext);
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
