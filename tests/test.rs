use http::uri::Uri;
use mdiu::*;

fn kitchen_sink() -> Result<Vec<Block>> {
    let lines = Document::new()
        .h1("title")
        .h2("section")
        .h3("subsection")
        .empty()
        .text("text")
        .link_with_label(Uri::from_static("one-link"), "one link")
        .quote("quote")
        .preformatted("@_@".to_string())
        .text("more text")
        .preformatted_with_alt("@_@".into(), "emoticon")
        .list_item("one item")
        .link(Uri::from_static("no-text"))
        .link_with_label(Uri::from_static("with-text"), "with text")
        .list_item("an item")
        .list_item("another item")
        .build()?;

    Ok(lines)
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

    assert_eq!(
        expected,
        &(kitchen_sink().unwrap()).into::<Gemtext>().to_string()
    );
}

#[cfg(feature = "html")]
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

    assert_eq!(expected, &kitchen_sink().unwrap().into::<Html>());
}

#[cfg(feature = "markdown")]
#[test]
fn markdown() {
    let expected = r#"# title

## section

### subsection

text

[one link](one-link)

> quote

    @_@

more text

    @_@

* one item

* [no-text](no-text)
* [with text](with-text)

* an item
* another item
"#;

    assert_eq!(expected, &kitchen_sink().unwrap().into::<Markdown>());
}
