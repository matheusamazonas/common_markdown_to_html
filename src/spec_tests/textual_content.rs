use crate::conversion::convert;

#[test]
fn example_650() {
	let markdown = "hello $.;'there\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>hello $.;'there</p>\n");
}

#[test]
fn example_651() {
	let markdown = "Foo χρῆν\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo χρῆν</p>\n");
}

#[test]
fn example_652() {
	let markdown = "Multiple     spaces\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Multiple     spaces</p>\n");
}
