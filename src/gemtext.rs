use super::{Block, Level, Markup};
use std::fmt::Write;

/// Gemtext formatter
pub struct Gemtext;

impl Markup for Gemtext {
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String {
        let mut b = String::new();

        for block in iter {
            let _ = match block {
                Block::Text(text) => writeln!(b, "{text}"),
                Block::Link(link) => match link.label() {
                    Some(label) => writeln!(b, "=> {uri} {label}", uri = link.uri()),
                    None => writeln!(b, "=> {uri}", uri = link.uri()),
                },
                Block::Heading(Level::One, text) => writeln!(b, "# {text}"),
                Block::Heading(Level::Two, text) => writeln!(b, "## {text}"),
                Block::Heading(Level::Three, text) => writeln!(b, "### {text}"),
                Block::ListItem(text) => writeln!(b, "* {text}"),
                Block::Quote(text) => writeln!(b, "> {text}"),
                Block::Preformatted(pre) => match pre.alt() {
                    Some(alt) => writeln!(b, "```{alt}\n{text}\n```", text = pre.text()),
                    None => writeln!(b, "```\n{text}\n```", text = pre.text()),
                },
                Block::Empty => writeln!(b),
            }
            .expect(crate::FMT_STRING_ERR);
        }

        b
    }
}
