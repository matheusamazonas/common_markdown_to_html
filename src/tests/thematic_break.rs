use crate::conversion::convert;

#[test]
fn example_43() {
	let markdown = "***\n---\n___\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n<hr />\n<hr />\n");
}

#[test]
fn example_44() {
	let markdown = "+++\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>+++</p>\n");
}

#[test]
fn example_45() {
	let markdown = "===\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>===</p>\n");
}

#[test]
fn example_46() {
	let markdown = "--\n**\n__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>--\n**\n__</p>\n");
}

#[test]
fn example_47() {
	let markdown = " ***\n  ***\n   ***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n<hr />\n<hr />\n");
}

#[test]
fn example_48() {
	let markdown = "    ***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>***\n</code></pre>\n");
}

#[test]
fn example_49() {
	let markdown = "Foo\n    ***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\n***</p>\n");
}

#[test]
fn example_50() {
	let markdown = "_____________________________________\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}

#[test]
fn example_51() {
	let markdown = " - - -\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}

#[test]
fn example_52() {
	let markdown = " **  * ** * ** * **\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}

#[test]
fn example_53() {
	let markdown = "-     -      -      -\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}

#[test]
fn example_54() {
	let markdown = "- - - -    \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n");
}

#[test]
fn example_55() {
	let markdown = "_ _ _ _ a\n\na------\n\n---a---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n");
}

#[test]
fn example_56() {
	let markdown = " *-*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>-</em></p>\n");
}

#[test]
fn example_57() {
	let markdown = "- foo\n***\n- bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo</li>\n</ul>\n<hr />\n<ul>\n<li>bar</li>\n</ul>\n"
	);
}

#[test]
fn example_58() {
	let markdown = "Foo\n***\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo</p>\n<hr />\n<p>bar</p>\n");
}

#[test]
fn example_59() {
	let markdown = "Foo\n---\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>Foo</h2>\n<p>bar</p>\n");
}

#[test]
fn example_60() {
	let markdown = "* Foo\n* * *\n* Bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>Foo</li>\n</ul>\n<hr />\n<ul>\n<li>Bar</li>\n</ul>\n"
	);
}

#[test]
fn example_61() {
	let markdown = "- Foo\n- * * *\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n");
}
