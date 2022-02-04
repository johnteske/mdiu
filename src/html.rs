use super::{FileExtension, FormatLine, Level, Line, Link};

pub struct Html;

impl FormatLine for Html {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String {
        let mut iter = iter.peekable();
        let mut s = String::new();

        let mut state = State::Normal;

        while let Some(line) = iter.next() {
            let l = match line {
                Line::Text(text) => format!("<p>{}</p>\n", text),
                Line::Link(link) => {
                    let next_line_is_link = matches!(iter.peek(), Some(Line::Link(_)));
                    wrap_list_item(&mut state, next_line_is_link, |wrapper| {
                        match link.label() {
                            Some(label) => {
                                format!(
                                    "<{0}><a href=\"{1}\">{2}</a></{0}>\n",
                                    wrapper,
                                    link.uri(),
                                    label
                                )
                            }
                            None => {
                                format!("<{0}><a href=\"{1}\">{1}</a></{0}>\n", wrapper, link.uri())
                            }
                        }
                    })
                }
                Line::Heading(Level::One, text) => format!("<h1>{}</h1>\n", text),
                Line::Heading(Level::Two, text) => format!("<h2>{}</h2>\n", text),
                Line::Heading(Level::Three, text) => format!("<h3>{}</h3>\n", text),
                Line::ListItem(text) => {
                    let next_line_is_item = matches!(iter.peek(), Some(Line::ListItem(_)));
                    wrap_list_item(&mut state, next_line_is_item, |wrapper| {
                        format!("<{0}>{1}</{0}>\n", wrapper, text)
                    })
                }
            };

            s += &l;
        }

        s
    }
}

enum State {
    InList,
    Normal,
}

fn wrap_list_item(
    state: &mut State,
    next_line_is_same: bool,
    format_item: impl Fn(&str) -> String,
) -> String {
    let mut b = String::new();

    if matches!(state, State::Normal) && next_line_is_same {
        *state = State::InList;
        b += "<ul>\n";
    }

    let wrapper = if matches!(state, State::InList) {
        "li"
    } else {
        "p"
    };

    b += &format_item(wrapper);

    if matches!(state, State::InList) && !next_line_is_same {
        *state = State::Normal;
        b += "</ul>\n";
    }

    b
}

impl FileExtension for Html {
    const FILE_EXTENSION: &'static str = "html";
}
