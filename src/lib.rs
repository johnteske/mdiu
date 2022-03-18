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
//! use mdiu::{Document, Gemtext};
//!
//! let gemtext: Gemtext = Document::new()
//!     .h1("my gemlog")
//!     .text("welcome")
//!     .build()?
//!     .into();
//!
//! assert_eq!(gemtext.to_string(), "# my gemlog\nwelcome\n");
//! # Ok(())
//! # }
//! ```
//!
//! Create a document block by block
//! ```
//! # fn main() -> mdiu::Result<()> {
//! use mdiu::{Block, Content, Gemtext, Level};
//!
//! let h1 = Block::Heading(Level::One, "my gemlog".parse()?);
//! let text = Block::Text(Content::new("welcome")?);
//! let doc = vec![h1, text];
//!
//! let gemtext: Gemtext = doc.into();
//!
//! assert_eq!(gemtext.to_string(), "# my gemlog\nwelcome\n");
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

// TODO I still might want a Markup trait
// so format output is a String, not a newtype
// and ToMarkup can still be a helper for &[Block]
