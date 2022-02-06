mod link;
pub use link::Link;

mod preformatted;
pub use preformatted::Preformatted;

mod gemtext;
pub use gemtext::Gemtext;

mod html;
pub use html::Html;

#[derive(Debug)]
pub enum Line {
    Text(String),
    Link(Link),
    Heading(Level, String),
    ListItem(String),
    Quote(String),
    Preformatted(Preformatted),
    Empty,
}

#[derive(Clone, Debug)]
pub enum Level {
    One,
    Two,
    Three,
}

#[derive(Debug, Default)]
pub struct Lines(Vec<Line>);

impl Lines {
    pub fn new() -> Self {
        Self::default()
    }

    /// ```
    /// # use mu_lines::{Line, Lines};
    /// let mut lines = Lines::new();
    /// lines.add(Line::Text("hello".into()));
    /// ```
    pub fn add(&mut self, line: Line) {
        // TODO check no newlines
        self.0.push(line);
    }
    // TODO add_unchecked

    pub fn to_string<F>(&self) -> String
    where
        F: FormatLine,
    {
        <F>::format(self.0.iter())
    }
}

pub trait FormatLine {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String;
}
