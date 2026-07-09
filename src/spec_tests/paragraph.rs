use crate::conversion::convert;

#[test]
fn example_219() {
	let markdown = "aaa\n\nbbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa</p>\n<p>bbb</p>\n");
}

#[test]
fn example_220() {
	let markdown = "aaa\nbbb\n\nccc\nddd\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n");
}

#[test]
fn example_221() {
	let markdown = "aaa\n\n\nbbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa</p>\n<p>bbb</p>\n");
}

#[test]
fn example_222() {
	let markdown = "  aaa\n bbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa\nbbb</p>\n");
}

#[test]
fn example_223() {
	let markdown = "aaa\n             bbb\n                                       ccc\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa\nbbb\nccc</p>\n");
}

#[test]
fn example_224() {
	let markdown = "   aaa\nbbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa\nbbb</p>\n");
}

#[test]
fn example_225() {
	let markdown = "    aaa\nbbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n</code></pre>\n<p>bbb</p>\n");
}

#[test]
fn example_226() {
	let markdown = "aaa     \nbbb     \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aaa<br />\nbbb</p>\n");
}
