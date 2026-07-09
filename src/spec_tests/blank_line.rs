use crate::conversion::convert;

#[test]
fn example_227() {
	let markdown = "  \n\naaa\n  \n\n# aaa\n\n  \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa</p>\n<h1>aaa</h1>\n");
}
