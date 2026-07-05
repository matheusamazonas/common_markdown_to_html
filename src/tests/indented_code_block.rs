use crate::conversion::convert;

#[test]
fn example_107() {
	let markdown = "    a simple\n      indented code block\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>a simple\n  indented code block\n</code></pre>\n"
	);
}

#[test]
fn example_108() {
	let markdown = "  - foo\n\n    bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n");
}

#[test]
fn example_109() {
	let markdown = "1.  foo\n\n    - bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_110() {
	let markdown = "    <a/>\n    *hi*\n\n    - one\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>&lt;a/&gt;\n*hi*\n\n- one\n</code></pre>\n"
	);
}

#[test]
fn example_111() {
	let markdown = "    chunk1\n\n    chunk2\n  \n \n \n    chunk3\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3\n</code></pre>\n"
	);
}

#[test]
fn example_112() {
	let markdown = "    chunk1\n      \n      chunk2\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>chunk1\n  \n  chunk2\n</code></pre>\n");
}

#[test]
fn example_113() {
	let markdown = "Foo\n    bar\n\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\nbar</p>\n");
}

#[test]
fn example_114() {
	let markdown = "    foo\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\n</code></pre>\n<p>bar</p>\n");
}

#[test]
fn example_115() {
	let markdown = "# Heading\n    foo\nHeading\n------\n    foo\n----\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h1>Heading</h1>\n<pre><code>foo\n</code></pre>\n<h2>Heading</h2>\n<pre><code>foo\n</code></pre>\n<hr />\n"
	);
}

#[test]
fn example_116() {
	let markdown = "        foo\n    bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>    foo\nbar\n</code></pre>\n");
}

#[test]
fn example_117() {
	let markdown = "\n    \n    foo\n    \n\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\n</code></pre>\n");
}

#[test]
fn example_118() {
	let markdown = "    foo  \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo  \n</code></pre>\n");
}
