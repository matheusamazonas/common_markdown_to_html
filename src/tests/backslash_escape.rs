use crate::conversion::convert;

#[test]
fn example_12() {
	let markdown = "\\!\\\"\\#\\$\\%\\&\\'\\(\\)\\*\\+\\,\\-\\.\\/\\:\\;\\<\\=\\>\\?\\@\\[\\\\\\]\\^\\_\\`\\{\\|\\}\\~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\\]^_`{|}~</p>\n"
	);
}

#[test]
fn example_13() {
	let markdown = "\\\t\\A\\a\\ \\3\\φ\\«\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "\\\t\\A\\a\\ \\3\\φ\\«</p>\n");
}

#[test]
fn example_14() {
	let markdown = "\\*not emphasized*\n\\<br/> not a tag\n\\[not a link](/foo)\n\\`not code`\n1\\. not a list\n\\* not a list\n\\# not a heading\n\\[foo]: /url \"not a reference\"\n\\&ouml; not a character entity\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>*not emphasized*\n&lt;br/&gt; not a tag\n[not a link](/foo)\n`not code`\n1. not a list\n* not a list\n# not a heading\n[foo]: /url &quot;not a reference&quot;\n&amp;ouml; not a character entity</p>\n"
	);
}

#[test]
fn example_15() {
	let markdown = "\\\\*emphasis*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>\\<em>emphasis</em></p>\n");
}

#[test]
fn example_16() {
	let markdown = "foo\\\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbar</p>\n");
}

#[test]
fn example_17() {
	let markdown = "`` \\[\\` ``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>\\[\\`</code></p>\n");
}

#[test]
fn example_18() {
	let markdown = "    \\[\\]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>\\[\\]\n</code></pre>\n");
}

#[test]
fn example_19() {
	let markdown = "~~~\n\\[\\]\n~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>\\[\\]\n</code></pre>\n");
}

#[test]
fn example_20() {
	let markdown = "<https://example.com?find=\\*>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"https://example.com?find=%5C*\">https://example.com?find=\\*</a></p>\n"
	);
}

#[test]
fn example_21() {
	let markdown = "<a href=\"/bar\\/)\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<a href=\"/bar\\/)\">\n");
}

#[test]
fn example_22() {
	let markdown = "[foo](/bar\\* \"ti\\*tle\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n");
}

#[test]
fn example_23() {
	let markdown = "[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n");
}

#[test]
fn example_24() {
	let markdown = "``` foo\\+bar\nfoo\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code class=\"language-foo+bar\">foo\n</code></pre>\n"
	);
}
