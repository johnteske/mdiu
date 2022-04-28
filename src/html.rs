use super::{Block, Level, Markup};

/// An HTML formatter, available with the `html` feature
///
/// Links are formatted as lists of links.
pub struct Html;

impl Markup for Html {
    fn markup(blocks: &[Block]) -> String {
        let mut iter = blocks.iter();
        let mut block = iter.next();
        let mut in_list: Option<List> = None;

        std::iter::from_fn(|| {
            let intermediate = match (in_list.as_ref(), block) {
                (None, Some(Block::Link(_))) => {
                    in_list = Some(List::Link);
                    Intermediate::OpenList
                }
                (None, Some(Block::ListItem(_))) => {
                    in_list = Some(List::Item);
                    Intermediate::OpenList
                }
                (Some(_), None) => {
                    in_list = None;
                    Intermediate::CloseList
                }
                (Some(List::Link), Some(b)) if !matches!(b, Block::Link(_)) => {
                    in_list = None;
                    Intermediate::CloseList
                }
                (Some(List::Item), Some(b)) if !matches!(b, Block::ListItem(_)) => {
                    in_list = None;
                    Intermediate::CloseList
                }
                (_, Some(block)) => Intermediate::Block(block),
                (None, None) => return None,
            };

            if let Intermediate::Block(_) = intermediate {
                block = iter.next();
            }

            Some(intermediate)
        })
        .map(|intermediate| match intermediate {
            Intermediate::Block(block) => generate(block),
            Intermediate::OpenList => "<ul>\n".to_string(),
            Intermediate::CloseList => "</ul>\n".to_string(),
        })
        .collect::<String>()
    }
}

enum List {
    Link,
    Item,
}

enum Intermediate<'a> {
    Block(&'a Block),
    OpenList,
    CloseList,
}

fn generate(block: &Block) -> String {
    match block {
        Block::Text(text) => format!("<p>{}</p>\n", text),
        Block::Link(link) => match link.label() {
            Some(label) => {
                format!("<li><a href=\"{}\">{}</a></li>\n", link.uri(), label)
            }
            None => {
                format!("<li><a href=\"{0}\">{0}</a></li>\n", link.uri())
            }
        },
        Block::Heading(Level::One, text) => format!("<h1>{}</h1>\n", text),
        Block::Heading(Level::Two, text) => format!("<h2>{}</h2>\n", text),
        Block::Heading(Level::Three, text) => format!("<h3>{}</h3>\n", text),
        Block::ListItem(text) => {
            format!("<li>{}</li>\n", text)
        }
        Block::Quote(text) => format!("<blockquote>{}</blockquote>\n", text),
        Block::Preformatted(pre) => format!("<pre>\n{}\n</pre>\n", pre.text()),
        // TODO
        Block::Empty => "".to_string(),
    }
}
