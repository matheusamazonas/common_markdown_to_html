use crate::conversion::convert;

#[test]
fn example_594() {
	let markdown = "<http://foo.bar.baz>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"http://foo.bar.baz\">http://foo.bar.baz</a></p>\n"
	);
}

#[test]
fn example_595() {
	let markdown = "<https://foo.bar.baz/test?q=hello&id=22&boolean>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean\">https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>\n"
	);
}

#[test]
fn example_596() {
	let markdown = "<irc://foo.bar:2233/baz>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"irc://foo.bar:2233/baz\">irc://foo.bar:2233/baz</a></p>\n"
	);
}

#[test]
fn example_597() {
	let markdown = "<MAILTO:FOO@BAR.BAZ>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"MAILTO:FOO@BAR.BAZ\">MAILTO:FOO@BAR.BAZ</a></p>\n"
	);
}

#[test]
fn example_598() {
	let markdown = "<a+b+c:d>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"a+b+c:d\">a+b+c:d</a></p>\n");
}

#[test]
fn example_599() {
	let markdown = "<made-up-scheme://foo,bar>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"made-up-scheme://foo,bar\">made-up-scheme://foo,bar</a></p>\n"
	);
}

#[test]
fn example_600() {
	let markdown = "<https://../>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"https://../\">https://../</a></p>\n");
}

#[test]
fn example_601() {
	let markdown = "<localhost:5001/foo>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"localhost:5001/foo\">localhost:5001/foo</a></p>\n"
	);
}

#[test]
fn example_602() {
	let markdown = "<https://foo.bar/baz bim>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;https://foo.bar/baz bim&gt;</p>\n");
}

#[test]
fn example_603() {
	let markdown = "<https://example.com/\\[\\>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"https://example.com/%5C%5B%5C\">https://example.com/\\[\\</a></p>\n"
	);
}

#[test]
fn example_604() {
	let markdown = "<foo@bar.example.com>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"mailto:foo@bar.example.com\">foo@bar.example.com</a></p>\n"
	);
}

#[test]
fn example_605() {
	let markdown = "<foo+special@Bar.baz-bar0.com>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"mailto:foo+special@Bar.baz-bar0.com\">foo+special@Bar.baz-bar0.com</a></p>\n"
	);
}

#[test]
fn example_606() {
	let markdown = "<foo\\+@bar.example.com>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;foo+@bar.example.com&gt;</p>\n");
}

#[test]
fn example_607() {
	let markdown = "<>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;&gt;</p>\n");
}

#[test]
fn example_608() {
	let markdown = "< https://foo.bar >\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt; https://foo.bar &gt;</p>\n");
}

#[test]
fn example_609() {
	let markdown = "<m:abc>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;m:abc&gt;</p>\n");
}

#[test]
fn example_610() {
	let markdown = "<foo.bar.baz>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;foo.bar.baz&gt;</p>\n");
}

#[test]
fn example_611() {
	let markdown = "https://example.com\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>https://example.com</p>\n");
}

#[test]
fn example_612() {
	let markdown = "foo@bar.example.com\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo@bar.example.com</p>\n");
}
