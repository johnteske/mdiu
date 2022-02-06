use http::uri::Uri;
use mu_lines::*;

fn kitchen_sink() -> Lines {
    Lines::new()
        .h1("title".to_string())
        .h2("section".to_string())
        .h3("subsection".into())
        .empty()
        .text("text".into())
        .link(Uri::from_static("one-link"), Some("one link".into()))
        .quote("quote".into())
        .preformatted("@_@".into(), None)
        .text("more text".into())
        .preformatted("@_@".into(), Some("emoticon".into()))
        .list_item("one item".into())
        .link(Uri::from_static("no-text"), None)
        .link(Uri::from_static("with-text"), Some("with text".into()))
        .list_item("an item".into())
        .list_item("another item".into())
}

#[test]
fn gemtext() {
    let expected = r#"# title
## section
### subsection

text
=> one-link one link
> quote
```
@_@
```
more text
```emoticon
@_@
```
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
<pre>
@_@
</pre>
<p>more text</p>
<pre>
@_@
</pre>
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
