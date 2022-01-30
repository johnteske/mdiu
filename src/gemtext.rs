use super::{FileExtension, FormatLine, Line, Link};

pub struct Gemtext;

impl FormatLine for Gemtext {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String {
        iter.map(|line| match line {
            Line::Text(text) => format!("{}\n", text),
            Line::Link(Link(url, text)) => match text {
                Some(text) => format!("=> {} {}\n", url, text),
                None => format!("=> {}\n", url),
            },
            Line::H1(text) => format!("# {}\n", text),
            Line::H2(text) => format!("## {}\n", text),
            Line::H3(text) => format!("### {}\n", text),
        })
        .collect::<String>()
    }
}

impl FileExtension for Gemtext {
    const FILE_EXTENSION: &'static str = "gmi";
}
