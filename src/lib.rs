//! Build documents with [Gemtext]
//!
//! `mdiu` provides a correct and flexible approach to creating small documents with Gemtext.
//!
//! Named after the [Manual Data Insertion Unit], part of Gemini's on-board computer.
//!
//! # Examples
//!
//! Create a document with a builder
//! ```
//! # fn main() -> mdiu::Result<()> {
//! use mdiu::{Document, Gemtext, ToMarkup};
//!
//! let gemtext = Document::new()
//!     .h1("my gemlog")
//!     .text("welcome")
//!     .build()?
//!     .to_markup::<Gemtext>();
//!
//! assert_eq!(gemtext, "# my gemlog\nwelcome\n");
//! # Ok(())
//! # }
//! ```
//!
//! Create a document block by block
//! ```
//! # fn main() -> mdiu::Result<()> {
//! use mdiu::{Block, Content, Gemtext, Level, Markup};
//!
//! let h1 = Block::Heading(Level::One, "my gemlog".parse()?);
//! let text = Block::Text(Content::new("welcome")?);
//! let doc = vec![h1, text];
//!
//! let gemtext = <Gemtext>::markup(&doc);
//!
//! assert_eq!(gemtext, "# my gemlog\nwelcome\n");
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! Formatting to [`Gemtext`] is supported by default.
//! Additional features are available for the following formats:
//!
//! * `html`
//! * `markdown`
//!
//! A Gemtext `parsing` feature is planned but not yet implemented.
//!
//! # Alternatives
//!
//! While `mdiu` only covers Gemtext, the following crates cover the full Gemini protocol:
//!
//! * [gmi](https://crates.io/crates/gmi)
//! * [gemini](https://crates.io/crates/gemini)
//!
//! [Gemtext]: https://gemini.circumlunar.space/docs/gemtext.gmi
//! [Manual Data Insertion Unit]: https://web.archive.org/web/20220201083102/https://www.ibiblio.org/apollo/Gemini.html

mod content;
pub use content::Content;

mod document;
pub use document::Document;

mod error;
pub use error::{Error, Result};

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

/// Heading level of a [`Block::Heading`]
#[derive(Clone, Debug)]
pub enum Level {
    One,
    Two,
    Three,
}

/// Format an iterator of [`Block`]s
///
/// # Example
///
/// See example usage in [crate documentation](./index.html#examples).
pub trait Markup {
    // This takes an iter of Blocks so the formatter can handle adjacent Blocks,
    // for example wrapping lists with <ul> in HTML
    fn markup(blocks: &[Block]) -> String;
}

/// Create [`Markup`]-formatted strings
///
/// This trait is sealed and cannot be implemented for types outside this crate.
///
/// # Example
///
/// See example usage in [crate documentation](./index.html#examples).
pub trait ToMarkup: private::Sealed {
    fn to_markup<F>(self) -> String
    where
        F: Markup;
}

impl<T> ToMarkup for T
where
    T: std::ops::Deref<Target = [Block]>,
{
    fn to_markup<F>(self) -> String
    where
        F: Markup,
    {
        <F>::markup(&self)
    }
}

mod private {
    use super::Block;

    pub trait Sealed {}

    impl<T> Sealed for T where T: std::ops::Deref<Target = [Block]> {}
}
