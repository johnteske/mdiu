use http::uri::Uri;

mod gemtext;
pub use gemtext::Gemtext;

mod html;
pub use html::Html;

#[derive(Debug)]
pub enum Line {
    Text(String),
    Link(Link),
    H1(String),
    H2(String),
    H3(String),
    // TODO unordered list
    // TODO blockquote
    // TODO preformatted
    // TODO blank line
}

#[derive(Clone, Debug)]
pub struct Link(pub Uri, pub Option<String>);

#[derive(Default)]
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

pub trait FileExtension {
    const FILE_EXTENSION: &'static str;
}
