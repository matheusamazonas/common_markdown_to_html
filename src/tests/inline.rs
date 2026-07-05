use crate::conversion::convert;

#[test]
fn example_327() {
	let markdown = "`hi`lo`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>hi</code>lo`</p>\n");
}
