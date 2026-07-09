use crate::conversion::convert;

#[test]
fn example_80() {
	let markdown = "Foo *bar*\n=========\n\nFoo *bar*\n---------\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h1>Foo <em>bar</em></h1>\n<h2>Foo <em>bar</em></h2>\n"
	);
}

#[test]
fn example_81() {
	let markdown = "Foo *bar\nbaz*\n====\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>Foo <em>bar\nbaz</em></h1>\n");
}

#[test]
fn example_82() {
	let markdown = "  Foo *bar\nbaz*\t\n====\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>Foo <em>bar\nbaz</em></h1>\n");
}

#[test]
fn example_83() {
	let markdown = "Foo\n-------------------------\n\nFoo\n=\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo</h2>\n<h1>Foo</h1>\n");
}

#[test]
fn example_84() {
	let markdown = "   Foo\n---\n\n  Foo\n-----\n\n  Foo\n  ===\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo</h2>\n<h2>Foo</h2>\n<h1>Foo</h1>\n");
}

#[test]
fn example_85() {
	let markdown = "    Foo\n    ---\n\n    Foo\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>Foo\n---\n\nFoo\n</code></pre>\n<hr />\n");
}

#[test]
fn example_86() {
	let markdown = "Foo\n   ----      \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo</h2>\n");
}

#[test]
fn example_87() {
	let markdown = "Foo\n    ---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\n---</p>\n");
}

#[test]
fn example_88() {
	let markdown = "Foo\n= =\n\nFoo\n--- -\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\n= =</p>\n<p>Foo</p>\n<hr />\n");
}

#[test]
fn example_89() {
	let markdown = "Foo  \n-----\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo</h2>\n");
}

#[test]
fn example_90() {
	let markdown = "Foo\\\n----\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo\\</h2>\n");
}

#[test]
fn example_91() {
	let markdown = "`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h2>`Foo</h2>\n<p>`</p>\n<h2>&lt;a title=&quot;a lot</h2>\n<p>of dashes&quot;/&gt;</p>\n"
	);
}

#[test]
fn example_92() {
	let markdown = "> Foo\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>Foo</p>\n</blockquote>\n<hr />\n");
}

#[test]
fn example_93() {
	let markdown = "> foo\nbar\n===\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>\n");
}

#[test]
fn example_94() {
	let markdown = "- Foo\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>Foo</li>\n</ul>\n<hr />\n");
}

#[test]
fn example_95() {
	let markdown = "Foo\nBar\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo\nBar</h2>\n");
}

#[test]
fn example_96() {
	let markdown = "---\nFoo\n---\nBar\n---\nBaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n<h2>Foo</h2>\n<h2>Bar</h2>\n<p>Baz</p>\n");
}

#[test]
fn example_97() {
	let markdown = "\n====\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>====</p>\n");
}

#[test]
fn example_98() {
	let markdown = "---\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n<hr />\n");
}

#[test]
fn example_99() {
	let markdown = "- foo\n-----\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>foo</li>\n</ul>\n<hr />\n");
}

#[test]
fn example_100() {
	let markdown = "    foo\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>foo\n</code></pre>\n<hr />\n");
}

#[test]
fn example_101() {
	let markdown = "> foo\n-----\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n");
}

#[test]
fn example_102() {
	let markdown = "\\> foo\n------\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>&gt; foo</h2>\n");
}

#[test]
fn example_103() {
	let markdown = "Foo\n\nbar\n---\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo</p>\n<h2>bar</h2>\n<p>baz</p>\n");
}

#[test]
fn example_104() {
	let markdown = "Foo\nbar\n\n---\n\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n");
}

#[test]
fn example_105() {
	let markdown = "Foo\nbar\n* * *\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n");
}

#[test]
fn example_106() {
	let markdown = "Foo\nbar\n\\---\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\nbar\n---\nbaz</p>\n");
}
