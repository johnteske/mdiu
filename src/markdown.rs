use super::{Block, Level, Markup};

/// A [Markdown 1.0.1] formatter, available with the `markdown` feature
///
/// [Markdown 1.0.1]: https://daringfireball.net/projects/markdown/
pub struct Markdown;

impl Markup for Markdown {
    fn markup<'a, T: IntoIterator<Item = &'a Block>>(collection: T) -> String {
        let mut iter = collection.into_iter().peekable();
        let mut s = String::new();

        let mut state = State::Normal;

        while let Some(block) = iter.next() {
            let l = match block {
                Block::Text(text) => format!("{}\n\n", text),
                Block::Link(link) => {
                    let next_block_is_link = matches!(iter.peek(), Some(Block::Link(_)));
                    wrap_list_item(&mut state, next_block_is_link, |prefix| {
                        match link.label() {
                            Some(label) => {
                                format!("{}[{}]({})\n", prefix, label, link.uri())
                            }
                            None => {
                                // Markdown 1.0.1 autolink syntax doesn't work for relative URIs
                                // https://daringfireball.net/projects/markdown/syntax#autolink
                                format!("{}[{1}]({1})\n", prefix, link.uri())
                            }
                        }
                    })
                }
                Block::Heading(Level::One, text) => format!("# {}\n\n", text),
                Block::Heading(Level::Two, text) => format!("## {}\n\n", text),
                Block::Heading(Level::Three, text) => format!("### {}\n\n", text),
                Block::ListItem(text) => {
                    let next_block_is_item = matches!(iter.peek(), Some(Block::ListItem(_)));
                    wrap_list_item(&mut state, next_block_is_item, |_| format!("* {}\n", text))
                }
                Block::Quote(text) => format!("> {}\n\n", text),
                Block::Preformatted(pre) => {
                    let lines: String = pre
                        .text()
                        .lines()
                        .map(|line| format!("    {}", line))
                        .collect();
                    format!("{}\n\n", lines)
                }
                Block::Empty => "".to_string(),
            };

            s += &l;
        }

        // Remove trailing newline
        s.pop();

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
    }

    let prefix = if matches!(state, State::InList) {
        "* "
    } else {
        ""
    };

    // TODO one usage of this always ignores the input
    b += &format_item(prefix);

    match (&state, next_block_is_same) {
        (State::InList, false) => {
            *state = State::Normal;
            b += "\n"
        }
        (State::Normal, _) => b += "\n",
        _ => {}
    }

    b
}
