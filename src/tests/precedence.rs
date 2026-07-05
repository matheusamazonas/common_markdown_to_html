use crate::conversion::convert;

#[test]
fn example_42() {
	let markdown = "- `one\n- two`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>`one</li>\n<li>two`</li>\n</ul>\n");
}
