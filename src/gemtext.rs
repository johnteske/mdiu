use super::{Block, Level, Markup};

/// A Gemtext formatter
pub struct Gemtext;

impl Markup for Gemtext {
    fn markup(blocks: &[Block]) -> String {
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
            .collect::<String>()
    }
}
