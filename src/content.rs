use std::fmt;

/// A [String] without newlines
#[derive(Debug, Clone)]
pub struct Content(String);

impl TryFrom<String> for Content {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_text(&value).map(|_| Content(value))
    }
}

impl TryFrom<&str> for Content {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_text(value).map(|_| Content(value.to_string()))
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// // TODO Error type would e custom--newline
// TODO handle /r variants too
// TODO cross platform \r combos as well
fn validate_text(text: &str) -> Result<(), ()> {
    if text.contains('\n') {
        Err(())
    } else {
        Ok(())
    }
}