use super::{FileExtension, FormatLine, Level, Line, Link};

pub struct Gemtext;

impl FormatLine for Gemtext {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String {
        iter.map(|line| match line {
            Line::Text(text) => format!("{}\n", text),
            Line::Link(Link(url, None)) => format!("=> {}\n", url),
            Line::Link(Link(url, Some(text))) => format!("=> {} {}\n", url, text),
            Line::Heading(Level::One, text) => format!("# {}\n", text),
            Line::Heading(Level::Two, text) => format!("## {}\n", text),
            Line::Heading(Level::Three, text) => format!("### {}\n", text),
            Line::ListItem(text) => format!("* {}\n", text),
        })
        .collect::<String>()
    }
}

impl FileExtension for Gemtext {
    const FILE_EXTENSION: &'static str = "gmi";
}
