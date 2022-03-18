use super::{Block, Level};
use std::fmt::Display;
use std::ops::Deref;

/// A Gemtext formatter
#[derive(Debug)]
pub struct Gemtext(String);

impl<T> From<T> for Gemtext
where
    T: Deref<Target = [Block]>,
{
    //impl From<&[Block]> for Gemtext {
    fn from(blocks: T) -> Self {
        Gemtext(
            blocks
                .into_iter()
                .map(|block| match block {
                    Block::Text(text) => format!("{}\n", text),
                    Block::Link(link) => match link.label() {
                        Some(label) => format!("=> {} {}\n", link.uri(), label),
                        None => format!("=> {}\n", link.uri()),
                    },
                    Block::Heading(Level::One, text) => format!("# {}\n", text),
                    Block::Heading(Level::Two, text) => format!("## {}\n", text),
                    Block::Heading(Level::Three, text) => format!("### {}\n", text),
                    Block::ListItem(text) => format!("* {}\n", text),
                    Block::Quote(text) => format!("> {}\n", text),
                    Block::Preformatted(pre) => match pre.alt() {
                        Some(alt) => format!("```{}\n{}\n```\n", alt, pre.text()),
                        None => format!("```\n{}\n```\n", pre.text()),
                    },
                    Block::Empty => "\n".to_string(),
                })
                .collect::<String>(),
        )
    }
}

impl Display for Gemtext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
