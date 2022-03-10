use super::{Block, Level, Markup};
use crate::list_state::{update_list_state, ListState};
use std::fmt::Write;

/// Markdown formatter
///
/// Supports [Markdown 1.0.1](https://daringfireball.net/projects/markdown/)
pub struct Markdown;

impl Markup for Markdown {
    fn markup<'a, I: Iterator<Item = &'a Block>>(iter: I) -> String {
        let mut iter = iter.peekable();

        let mut b = String::new();
        let mut state = ListState::NotInList;

        while let Some(block) = iter.next() {
            write_block(&mut b, &mut state, block, iter.peek())
                //https://doc.rust-lang.org/std/macro.format.html#panics
                .expect("BUG: fmt::Write for String does not return an error");
        }

        // Remove trailing newline
        b.pop();

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
        Block::Text(text) => write!(b, "{}\n\n", text),
        Block::Link(link) => {
            let next_block_is_link = matches!(next_block, Some(Block::Link(_)));
            update_list_state(state, next_block_is_link);

            let prefix = match &state {
                ListState::NotInList => "",
                _ => "* ",
            };

            // Markdown 1.0.1 autolink syntax doesn't work for relative URIs
            // https://daringfireball.net/projects/markdown/syntax#autolink
            let uri = link.uri();
            let label = match link.label() {
                Some(link) => link.to_string(),
                None => uri.to_string(),
            };
            writeln!(b, "{}[{}]({})", prefix, label, link.uri())?;

            list_trailing_newline(b, state)
        }
        Block::Heading(Level::One, text) => write!(b, "# {}\n\n", text),
        Block::Heading(Level::Two, text) => write!(b, "## {}\n\n", text),
        Block::Heading(Level::Three, text) => write!(b, "### {}\n\n", text),
        Block::ListItem(text) => {
            let next_block_is_item = matches!(next_block, Some(Block::ListItem(_)));
            update_list_state(state, next_block_is_item);
            writeln!(b, "* {}", text)?;
            list_trailing_newline(b, state)
        }
        Block::Quote(text) => write!(b, "> {}\n\n", text),
        Block::Preformatted(pre) => {
            for line in pre.text().lines() {
                write!(b, "    {}", line)?;
            }
            write!(b, "\n\n")
        }
        Block::Empty => Ok(()),
    }
}

// After a list or single list item/link, add a newline
fn list_trailing_newline(b: &mut String, state: &ListState) -> Result<(), std::fmt::Error> {
    match &state {
        ListState::NotInList | ListState::Exiting => writeln!(b),
        _ => Ok(()),
    }
}
