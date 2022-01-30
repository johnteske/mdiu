use http::uri::Uri;
use mu_lines::*;

fn kitchen_sink() -> Lines {
    let mut lines = Lines::new();
    lines.add(Line::H1("title".into()));
    lines.add(Line::H2("section".into()));
    lines.add(Line::H3("subsection".into()));
    lines.add(Line::Text("text".into()));
    lines.add(Line::Link(Link(
        Uri::from_static("a-link"),
        Some("a link".into()),
    )));
    lines.add(Line::Text("more text".into()));
    lines.add(Line::Link(Link(Uri::from_static("no-text"), None)));
    lines.add(Line::Link(Link(
        Uri::from_static("with-text"),
        Some("with text".into()),
    )));
    lines
}

#[test]
fn gemtext() {
    let expected = r#"# title
## section
### subsection
text
=> a-link a link
more text
=> no-text
=> with-text with text
"#;

    assert_eq!(expected, kitchen_sink().to_string::<Gemtext>());
}

#[test]
fn html() {
    let expected = r#"<h1>title</h1>
<h2>section</h2>
<h3>subsection</h3>
<p>text</p>
<p><a href="a-link">a link</a></p>
<p>more text</p>
<ul>
<li><a href="no-text">no-text</a></li>
<li><a href="with-text">with text</a></li>
</ul>
"#;

    assert_eq!(expected, kitchen_sink().to_string::<Html>());
}
