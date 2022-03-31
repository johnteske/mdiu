use super::{Block, Level, Markup};

/// A [Markdown 1.0.1] formatter, available with the `markdown` feature
///
/// Links are formatted as list items
/// [Markdown 1.0.1]: https://daringfireball.net/projects/markdown/
pub struct Markdown;

impl Markup for Markdown {
    fn markup(blocks: &[Block]) -> String {
        let mut s = String::new();
        let mut last_block: Option<&Block> = None;

        for block in blocks {
            match (last_block, block) {
                (Some(Block::Link(_)), b) if !matches!(b, Block::Link(_)) => s += "\n",
                (Some(Block::ListItem(_)), b) if !matches!(b, Block::ListItem(_)) => s += "\n",
                _ => {}
            }

            let l = match block {
                Block::Text(text) => format!("{}\n\n", text),
                Block::Link(link) => {
                    match link.label() {
                        Some(label) => {
                            format!("* [{}]({})\n", label, link.uri())
                        }
                        None => {
                            // Markdown 1.0.1 autolink syntax doesn't work for relative URIs
                            // https://daringfireball.net/projects/markdown/syntax#autolink
                            format!("* [{0}]({0})\n", link.uri())
                        }
                    }
                }
                Block::Heading(Level::One, text) => format!("# {}\n\n", text),
                Block::Heading(Level::Two, text) => format!("## {}\n\n", text),
                Block::Heading(Level::Three, text) => format!("### {}\n\n", text),
                Block::ListItem(text) => format!("* {}\n", text),
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

            last_block = Some(block);
            s += &l;
        }

        // Remove trailing newline
        if s.ends_with("\n\n") {
            s.pop();
        }

        s
    }
}
