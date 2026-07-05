use crate::conversion::convert;

#[test]
fn example_648() {
	let markdown = "foo\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\nbaz</p>\n");
}

#[test]
fn example_649() {
	let markdown = "foo \n baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\nbaz</p>\n");
}
