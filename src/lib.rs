use http::uri::Uri;

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

/// A builder to create a document by `Line`
#[derive(Debug, Default)]
pub struct Lines(Vec<Line>);

impl Lines {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(mut self, line: Line) -> Self {
        self.0.push(line);
        self
    }

    // TODO check no newlines
    // TODO add_unchecked

    pub fn text(self, text: String) -> Self {
        self.push(Line::Text(text))
    }

    pub fn link(self, uri: Uri, label: Option<String>) -> Self {
        self.push(Line::Link(Link::new(uri, label)))
    }

    pub fn h1(self, text: String) -> Self {
        self.push(Line::Heading(Level::One, text))
    }

    pub fn h2(self, text: String) -> Self {
        self.push(Line::Heading(Level::Two, text))
    }

    pub fn h3(self, text: String) -> Self {
        self.push(Line::Heading(Level::Three, text))
    }

    pub fn list_item(self, text: String) -> Self {
        self.push(Line::ListItem(text))
    }

    pub fn quote(self, text: String) -> Self {
        self.push(Line::Quote(text))
    }

    pub fn preformatted(self, text: String, alt: Option<String>) -> Self {
        self.push(Line::Preformatted(Preformatted::new(text, alt)))
    }

    pub fn empty(self) -> Self {
        self.push(Line::Empty)
    }

    // iter
    // collect

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
