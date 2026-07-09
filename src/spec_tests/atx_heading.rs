use crate::conversion::convert;

#[test]
fn example_62() {
	let markdown = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</h4>\n<h5>foo</h5>\n<h6>foo</h6>\n"
	);
}

#[test]
fn example_63() {
	let markdown = "####### foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>####### foo</p>\n");
}

#[test]
fn example_64() {
	let markdown = "#5 bolt\n\n#hashtag\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>#5 bolt</p>\n<p>#hashtag</p>\n");
}

#[test]
fn example_65() {
	let markdown = "\\## foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>## foo</p>\n");
}

#[test]
fn example_66() {
	let markdown = "# foo *bar* \\*baz\\*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>foo <em>bar</em> *baz*</h1>\n");
}

#[test]
fn example_67() {
	let markdown = "#                  foo                     \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>foo</h1>\n");
}

#[test]
fn example_68() {
	let markdown = " ### foo\n  ## foo\n   # foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>\n");
}

#[test]
fn example_69() {
	let markdown = "    # foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code># foo\n</code></pre>\n");
}

#[test]
fn example_70() {
	let markdown = "foo\n    # bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\n# bar</p>\n");
}

#[test]
fn example_71() {
	let markdown = "## foo ##\n  ###   bar    ###\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2>foo</h2>\n<h3>bar</h3>\n");
}

#[test]
fn example_72() {
	let markdown = "# foo ##################################\n##### foo ##\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>foo</h1>\n<h5>foo</h5>\n");
}

#[test]
fn example_73() {
	let markdown = "### foo ###     \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo</h3>\n");
}

#[test]
fn example_74() {
	let markdown = "### foo ### b\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo ### b</h3>\n");
}

#[test]
fn example_75() {
	let markdown = "# foo#\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>foo#</h1>\n");
}

#[test]
fn example_76() {
	let markdown = "### foo \\###\n## foo #\\##\n# foo \\#\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo ###</h3>\n<h2>foo ###</h2>\n<h1>foo #</h1>\n");
}

#[test]
fn example_77() {
	let markdown = "****\n## foo\n****\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<hr />\n<h2>foo</h2>\n<hr />\n");
}

#[test]
fn example_78() {
	let markdown = "Foo bar\n# baz\nBar foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo bar</p>\n<h1>baz</h1>\n<p>Bar foo</p>\n");
}

#[test]
fn example_79() {
	let markdown = "## \n#\n### ###\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h2></h2>\n<h1></h1>\n<h3></h3>\n");
}
