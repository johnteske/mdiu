use super::{Block, Level, Markup};
use crate::list_state::{update_list_state, ListState};
use std::fmt::Write;

/// HTML formatter
pub struct Html;

impl Markup for Html {
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String {
        let mut iter = iter.peekable();

        let mut b = String::new();
        let mut state = ListState::NotInList;

        while let Some(block) = iter.next() {
            write_block(&mut b, &mut state, block, iter.peek()).expect("TODO");
        }

        b
    }
}

fn write_block(
    b: &mut String,
    state: &mut ListState,
    block: &Block,
    next_block: Option<&&Block>,
) -> Result<(), std::fmt::Error> {
    match block {
        Block::Text(text) => writeln!(b, "<p>{}</p>", text),
        Block::Link(link) => {
            let next_block_is_link = matches!(next_block, Some(Block::Link(_)));
            update_list_state(state, next_block_is_link);

            if matches!(state, ListState::Entering) {
                writeln!(b, "<ul>")?;
            }

            let wrapper = list_item_wrapper(state);
            let uri = link.uri();
            let label = link
                .label()
                .as_ref()
                .map_or(uri.to_string(), |l| l.to_string());
            writeln!(b, "<{0}><a href=\"{1}\">{2}</a></{0}>", wrapper, uri, label)?;

            if matches!(state, ListState::Exiting) {
                writeln!(b, "</ul>")?;
            }

            Ok(())
        }
        Block::Heading(Level::One, text) => writeln!(b, "<h1>{}</h1>", text),
        Block::Heading(Level::Two, text) => writeln!(b, "<h2>{}</h2>", text),
        Block::Heading(Level::Three, text) => writeln!(b, "<h3>{}</h3>", text),
        Block::ListItem(text) => {
            let next_block_is_item = matches!(next_block, Some(Block::ListItem(_)));
            update_list_state(state, next_block_is_item);

            if matches!(state, ListState::Entering) {
                writeln!(b, "<ul>")?;
            }

            let wrapper = list_item_wrapper(state);
            writeln!(b, "<{0}>{1}</{0}>", wrapper, text)?;

            if matches!(state, ListState::Exiting) {
                writeln!(b, "</ul>")?;
            }

            Ok(())
        }
        Block::Quote(text) => writeln!(b, "<blockquote>{}</blockquote>", text),
        Block::Preformatted(pre) => writeln!(b, "<pre>\n{}\n</pre>", pre.text()),
        Block::Empty => Ok(()),
    }
}

fn list_item_wrapper(state: &ListState) -> &str {
    match &state {
        ListState::NotInList => "p",
        _ => "li",
    }
}
