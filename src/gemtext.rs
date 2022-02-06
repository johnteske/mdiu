use super::{FormatLine, Level, Line};

pub struct Gemtext;

impl FormatLine for Gemtext {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String {
        iter.map(|line| match line {
            Line::Text(text) => format!("{}\n", text),
            Line::Link(link) => match link.label() {
                Some(label) => format!("=> {} {}\n", link.uri(), label),
                None => format!("=> {}\n", link.uri()),
            },
            Line::Heading(Level::One, text) => format!("# {}\n", text),
            Line::Heading(Level::Two, text) => format!("## {}\n", text),
            Line::Heading(Level::Three, text) => format!("### {}\n", text),
            Line::ListItem(text) => format!("* {}\n", text),
            Line::Quote(text) => format!("> {}\n", text),
            Line::Preformatted(pre) => match pre.alt() {
                Some(alt) => format!("```{}\n{}\n```\n", alt, pre.text()),
                None => format!("```\n{}\n```\n", pre.text()),
            },
            Line::Empty => "\n".to_string(),
        })
        .collect::<String>()
    }
}
