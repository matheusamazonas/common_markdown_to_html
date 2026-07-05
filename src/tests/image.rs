use crate::conversion::convert;

#[test]
fn example_572() {
	let markdown = "![foo](/url \"title\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_573() {
	let markdown = "![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n"
	);
}

#[test]
fn example_574() {
	let markdown = "![foo ![bar](/url)](/url2)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n");
}

#[test]
fn example_575() {
	let markdown = "![foo [bar](/url)](/url2)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n");
}

#[test]
fn example_576() {
	let markdown = "![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n"
	);
}

#[test]
fn example_577() {
	let markdown = "![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n"
	);
}

#[test]
fn example_578() {
	let markdown = "![foo](train.jpg)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"train.jpg\" alt=\"foo\" /></p>\n");
}

#[test]
fn example_579() {
	let markdown = "My ![foo bar](/path/to/train.jpg  \"title\"   )\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>My <img src=\"/path/to/train.jpg\" alt=\"foo bar\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_580() {
	let markdown = "![foo](<url>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"url\" alt=\"foo\" /></p>\n");
}

#[test]
fn example_581() {
	let markdown = "![](/url)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"/url\" alt=\"\" /></p>\n");
}

#[test]
fn example_582() {
	let markdown = "![foo][bar]\n\n[bar]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"/url\" alt=\"foo\" /></p>\n");
}

#[test]
fn example_583() {
	let markdown = "![foo][bar]\n\n[BAR]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"/url\" alt=\"foo\" /></p>\n");
}

#[test]
fn example_584() {
	let markdown = "![foo][]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_585() {
	let markdown = "![*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_586() {
	let markdown = "![Foo][]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_587() {
	let markdown = "![foo] \n[]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo\" title=\"title\" />\n[]</p>\n"
	);
}

#[test]
fn example_588() {
	let markdown = "![foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_589() {
	let markdown = "![*foo* bar]\n\n[*foo* bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_590() {
	let markdown = "![[foo]]\n\n[[foo]]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>![[foo]]</p>\n<p>[[foo]]: /url &quot;title&quot;</p>\n"
	);
}

#[test]
fn example_591() {
	let markdown = "![Foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n"
	);
}

#[test]
fn example_592() {
	let markdown = "!\\[foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>![foo]</p>\n");
}

#[test]
fn example_593() {
	let markdown = "\\![foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>!<a href=\"/url\" title=\"title\">foo</a></p>\n");
}
