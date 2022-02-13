use super::{Block, FormatBlocks, Level};

pub struct Html;

impl FormatBlocks for Html {
    fn format<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String {
        let mut iter = iter.peekable();
        let mut s = String::new();

        let mut state = State::Normal;

        while let Some(block) = iter.next() {
            let l = match block {
                Block::Text(text) => format!("<p>{}</p>\n", text),
                Block::Link(link) => {
                    let next_block_is_link = matches!(iter.peek(), Some(Block::Link(_)));
                    wrap_list_item(&mut state, next_block_is_link, |wrapper| {
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
                Block::Heading(Level::One, text) => format!("<h1>{}</h1>\n", text),
                Block::Heading(Level::Two, text) => format!("<h2>{}</h2>\n", text),
                Block::Heading(Level::Three, text) => format!("<h3>{}</h3>\n", text),
                Block::ListItem(text) => {
                    let next_block_is_item = matches!(iter.peek(), Some(Block::ListItem(_)));
                    wrap_list_item(&mut state, next_block_is_item, |wrapper| {
                        format!("<{0}>{1}</{0}>\n", wrapper, text)
                    })
                }
                Block::Quote(text) => format!("<blockquote>{}</blockquote>\n", text),
                Block::Preformatted(pre) => format!("<pre>\n{}\n</pre>\n", pre.text()),
                Block::Empty => "".to_string(),
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
    next_block_is_same: bool,
    format_item: impl Fn(&str) -> String,
) -> String {
    let mut b = String::new();

    if matches!(state, State::Normal) && next_block_is_same {
        *state = State::InList;
        b += "<ul>\n";
    }

    let wrapper = if matches!(state, State::InList) {
        "li"
    } else {
        "p"
    };

    b += &format_item(wrapper);

    if matches!(state, State::InList) && !next_block_is_same {
        *state = State::Normal;
        b += "</ul>\n";
    }

    b
}
