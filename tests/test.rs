use http::uri::Uri;
use mu_lines::*;

fn kitchen_sink() -> Lines {
    let mut lines = Lines::new();
    lines.add(Line::Heading(Level::One, "title".into()));
    lines.add(Line::Heading(Level::Two, "section".into()));
    lines.add(Line::Heading(Level::Three, "subsection".into()));
    lines.add(Line::Text("text".into()));
    lines.add(Line::Link(Link::new(
        Uri::from_static("one-link"),
        Some("one link".into()),
    )));
    lines.add(Line::Quote("quote".into()));
    lines.add(Line::Text("more text".into()));
    lines.add(Line::ListItem("one item".into()));
    lines.add(Line::Link(Link::new(Uri::from_static("no-text"), None)));
    lines.add(Line::Link(Link::new(
        Uri::from_static("with-text"),
        Some("with text".into()),
    )));
    lines.add(Line::ListItem("an item".into()));
    lines.add(Line::ListItem("another item".into()));
    lines
}

#[test]
fn gemtext() {
    let expected = r#"# title
## section
### subsection
text
=> one-link one link
> quote
more text
* one item
=> no-text
=> with-text with text
* an item
* another item
"#;

    assert_eq!(expected, kitchen_sink().to_string::<Gemtext>());
}

#[test]
fn html() {
    let expected = r#"<h1>title</h1>
<h2>section</h2>
<h3>subsection</h3>
<p>text</p>
<p><a href="one-link">one link</a></p>
<blockquote>quote</blockquote>
<p>more text</p>
<p>one item</p>
<ul>
<li><a href="no-text">no-text</a></li>
<li><a href="with-text">with text</a></li>
</ul>
<ul>
<li>an item</li>
<li>another item</li>
</ul>
"#;

    assert_eq!(expected, kitchen_sink().to_string::<Html>());
}
