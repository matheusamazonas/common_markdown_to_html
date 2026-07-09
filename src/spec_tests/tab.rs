use crate::conversion::convert;

#[test]
fn example_1() {
	let markdown = "\tfoo\tbaz\t\tbim\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\tbaz\t\tbim\n</code></pre>\n");
}

#[test]
fn example_2() {
	let markdown = "  \tfoo\tbaz\t\tbim\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\tbaz\t\tbim\n</code></pre>\n");
}

#[test]
fn example_3() {
	let markdown = "    a\ta\n    ὐ\ta\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>a\ta\nὐ\ta\n</code></pre>\n");
}

#[test]
fn example_4() {
	let markdown = "  - foo\n\n\tbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n");
}

#[test]
fn example_5() {
	let markdown = "- foo\n\n\t\tbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>foo</p>\n<pre><code>  bar\n</code></pre>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_6() {
	let markdown = ">\t\tfoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<pre><code>  foo\n</code></pre>\n</blockquote>\n"
	);
}

#[test]
fn example_7() {
	let markdown = "-\t\tfoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<pre><code>  foo\n</code></pre>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_8() {
	let markdown = "    foo\n\tbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\nbar\n</code></pre>\n");
}

#[test]
fn example_9() {
	let markdown = " - foo\n   - bar\n\t - baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_10() {
	let markdown = "#\tFoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>Foo</h1>\n");
}

#[test]
fn example_11() {
	let markdown = "*\t*\t*\t\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}
