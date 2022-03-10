use super::{Block, Level, Markup};

/// Gemtext formatter
pub struct Gemtext;

impl Markup for Gemtext {
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String {
        iter.map(|block| match block {
            Block::Text(text) => format!("{text}\n"),
            Block::Link(link) => match link.label() {
                Some(label) => format!("=> {uri} {label}\n", uri = link.uri()),
                None => format!("=> {uri}\n", uri = link.uri()),
            },
            Block::Heading(Level::One, text) => format!("# {text}\n"),
            Block::Heading(Level::Two, text) => format!("## {text}\n"),
            Block::Heading(Level::Three, text) => format!("### {text}\n"),
            Block::ListItem(text) => format!("* {text}\n"),
            Block::Quote(text) => format!("> {text}\n"),
            Block::Preformatted(pre) => match pre.alt() {
                Some(alt) => format!("```{alt}\n{text}\n```\n", text = pre.text()),
                None => format!("```\n{text}\n```\n", text = pre.text()),
            },
            Block::Empty => "\n".to_string(),
        })
        .collect::<String>()
    }
}
