use http::uri::Uri;
use std::fmt;

mod link;
pub use link::Link;

mod preformatted;
pub use preformatted::Preformatted;

mod gemtext;
pub use gemtext::Gemtext;

mod html;
pub use html::Html;

#[derive(Debug, Clone)]
pub enum Line {
    Text(Content),
    Link(Link),
    Heading(Level, Content),
    ListItem(Content),
    Quote(Content),
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
#[derive(Debug, Default, Clone)]
pub struct Lines(Vec<Line>);

macro_rules! setter {
    ($func:ident($($param:ident: $ty:ty),*), $line:block) => {
        pub fn $func(
            &mut self,
            $($param: $ty),*
        ) -> &mut Self {
            let line: Line = $line;
            self.push(line);
            self
        }
    };
}

macro_rules! try_into_setter {
    ($func:ident($($param:ident: $ty:ty),*), $line:block) => {
        pub fn $func<T: TryInto<Content>>(
            &mut self,
            $($param: $ty),+
        ) -> Result<&mut Self, <T as std::convert::TryInto<Content>>::Error> {
            let line: Line = $line;
            self.push(line);
            Ok(self)
        }
    };
}

// // TODO Error type would e custom--newline
// TODO handle /r variants too
fn validate_text(text: &str) -> Result<(), ()> {
    if text.contains("\n") {
        Err(())
    } else {
        Ok(())
    }
}

impl Lines {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, line: Line) -> &mut Self {
        self.0.push(line);
        self
    }

    try_into_setter!(text(text: T), {
        let content = text.try_into()?;
        Line::Text(content)
    });

    setter!(link(uri: Uri), { Line::Link(Link::new(uri, None)) });
    try_into_setter!(link_with_label(uri: Uri, label: T), {
        let label = label.try_into()?;
        Line::Link(Link::new(uri, Some(label)))
    });

    try_into_setter!(h1(text: T), {
        let content = text.try_into()?;
        Line::Heading(Level::One, content)
    });

    try_into_setter!(h2(text: T), {
        let content = text.try_into()?;
        Line::Heading(Level::Two, content)
    });

    try_into_setter!(h3(text: T), {
        let content = text.try_into()?;
        Line::Heading(Level::Three, content)
    });

    try_into_setter!(list_item(text: T), {
        let content = text.try_into()?;
        Line::ListItem(content)
    });

    try_into_setter!(quote(text: T), {
        let content = text.try_into()?;
        Line::Quote(content)
    });

    setter!(preformatted(text: String), {
        Line::Preformatted(Preformatted::new(text, None))
    });
    try_into_setter!(preformatted_with_alt(text: String, alt: T), {
        let alt = alt.try_into()?;
        Line::Preformatted(Preformatted::new(text, Some(alt)))
    });

    setter!(empty(), { Line::Empty });

    // iter
    // collect

    pub fn build(&self) -> Vec<Line> {
        self.0.clone()
    }

    //pub fn to_string<F>(&self) -> String
    //where
    //    F: FormatLine,
    //{
    //    <F>::format(self.0.iter())
    //}
}

pub fn to_string<F>(lines: &[Line]) -> String
where
    F: FormatLine,
{
    <F>::format(lines.iter())
}

/// Format `Lines` to
pub trait FormatLine {
    fn format<'a, I: Iterator<Item = &'a Line>>(iter: I) -> String;
}

/// A String without newlines
#[derive(Debug, Clone)]
pub struct Content(String);

impl TryFrom<String> for Content {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO cross platform \r combos as well
        if value.contains("\n") {
            Err(())
        } else {
            Ok(Content(value))
        }
    }
}

impl TryFrom<&str> for Content {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO cross platform \r combos as well
        if value.contains("\n") {
            Err(())
        } else {
            Ok(Content(value.to_string()))
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO
// ok, so validation.
// StringLine and try_from handle this before input
// but also the bulider could take in input, then validate separately,
//  maybe it is simply .validate() or .is_valid()
//  or that Lines really is just a builder and it validates on build,
//  then map and vec-like operations can be used on.. just a plain vec
//
//  but then what protects a plain vec from newlines?
//  should the builder take str and validate, outputting Lines with StringLines?
//  and if you want to make a vec w/o builder, you need to do the manual work to convert as well?
