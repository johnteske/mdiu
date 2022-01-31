use super::{FileExtension, FormatLine, Level, Line, Link};

pub struct Html;

impl FormatLine for Html {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String {
        let mut iter = iter.peekable();
        let mut s = String::new();

        let mut in_link_list = false;
        while let Some(line) = iter.next() {
            let l = match line {
                Line::Text(text) => format!("<p>{}</p>\n", text),
                Line::Link(Link(url, text)) => {
                    let mut b = String::new();
                    let next_line_is_link = matches!(iter.peek(), Some(Line::Link(_)));

                    if !in_link_list && next_line_is_link {
                        in_link_list = true;
                        b += "<ul>\n";
                    }

                    let wrapper = if in_link_list { "li" } else { "p" };

                    let link = match text {
                        Some(text) => {
                            format!("<{0}><a href=\"{1}\">{2}</a></{0}>\n", wrapper, url, text)
                        }
                        None => format!("<{0}><a href=\"{1}\">{1}</a></{0}>\n", wrapper, url),
                    };
                    b += &link;

                    if in_link_list && !next_line_is_link {
                        in_link_list = false;
                        b += "</ul>\n";
                    }

                    b
                }
                Line::Heading(Level::One, text) => format!("<h1>{}</h1>\n", text),
                Line::Heading(Level::Two, text) => format!("<h2>{}</h2>\n", text),
                Line::Heading(Level::Three, text) => format!("<h3>{}</h3>\n", text),
            };

            s += &l;
        }

        s
    }
}

impl FileExtension for Html {
    const FILE_EXTENSION: &'static str = "html";
}
