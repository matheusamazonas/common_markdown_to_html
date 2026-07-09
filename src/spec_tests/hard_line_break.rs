use crate::conversion::convert;

#[test]
fn example_633() {
	let markdown = "foo  \nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbaz</p>\n");
}

#[test]
fn example_634() {
	let markdown = "foo\\\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbaz</p>\n");
}

#[test]
fn example_635() {
	let markdown = "foo       \nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbaz</p>\n");
}

#[test]
fn example_636() {
	let markdown = "foo  \n     bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbar</p>\n");
}

#[test]
fn example_637() {
	let markdown = "foo\\\n     bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<br />\nbar</p>\n");
}

#[test]
fn example_638() {
	let markdown = "*foo  \nbar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo<br />\nbar</em></p>\n");
}

#[test]
fn example_639() {
	let markdown = "*foo\\\nbar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo<br />\nbar</em></p>\n");
}

#[test]
fn example_640() {
	let markdown = "`code  \nspan`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>code   span</code></p>\n");
}

#[test]
fn example_641() {
	let markdown = "`code\\\nspan`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>code\\ span</code></p>\n");
}

#[test]
fn example_642() {
	let markdown = "<a href=\"foo  \nbar\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo  \nbar\"></p>\n");
}

#[test]
fn example_643() {
	let markdown = "<a href=\"foo\\\nbar\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo\\\nbar\"></p>\n");
}

#[test]
fn example_644() {
	let markdown = "foo\\\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\\</p>\n");
}

#[test]
fn example_645() {
	let markdown = "foo  \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo</p>\n");
}

#[test]
fn example_646() {
	let markdown = "### foo\\\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo\\</h3>\n");
}

#[test]
fn example_647() {
	let markdown = "### foo  \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h3>foo</h3>\n");
}
