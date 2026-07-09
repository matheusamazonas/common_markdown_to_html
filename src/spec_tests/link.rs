use crate::conversion::convert;

#[test]
fn example_482() {
	let markdown = "[link](/uri \"title\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\" title=\"title\">link</a></p>\n");
}

#[test]
fn example_483() {
	let markdown = "[link](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">link</a></p>\n");
}

#[test]
fn example_484() {
	let markdown = "[](./target.md)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"./target.md\"></a></p>\n");
}

#[test]
fn example_485() {
	let markdown = "[link]()\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"\">link</a></p>\n");
}

#[test]
fn example_486() {
	let markdown = "[link](<>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"\">link</a></p>\n");
}

#[test]
fn example_487() {
	let markdown = "[]()\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"\"></a></p>\n");
}

#[test]
fn example_488() {
	let markdown = "[link](/my uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link](/my uri)</p>\n");
}

#[test]
fn example_489() {
	let markdown = "[link](</my uri>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/my%20uri\">link</a></p>\n");
}

#[test]
fn example_490() {
	let markdown = "[link](foo\nbar)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link](foo\nbar)</p>\n");
}

#[test]
fn example_491() {
	let markdown = "[link](<foo\nbar>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link](<foo\nbar>)</p>\n");
}

#[test]
fn example_492() {
	let markdown = "[a](<b)c>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"b)c\">a</a></p>\n");
}

#[test]
fn example_493() {
	let markdown = "[link](<foo\\>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link](&lt;foo&gt;)</p>\n");
}

#[test]
fn example_494() {
	let markdown = "[a](<b)c\n[a](<b)c>\n[a](<b>c)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[a](&lt;b)c\n[a](&lt;b)c&gt;\n[a](<b>c)</p>\n");
}

#[test]
fn example_495() {
	let markdown = "[link](\\(foo\\))\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"(foo)\">link</a></p>\n");
}

#[test]
fn example_496() {
	let markdown = "[link](foo(and(bar)))\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo(and(bar))\">link</a></p>\n");
}

#[test]
fn example_497() {
	let markdown = "[link](foo(and(bar))\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link](foo(and(bar))</p>\n");
}

#[test]
fn example_498() {
	let markdown = "[link](foo\\(and\\(bar\\))\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo(and(bar)\">link</a></p>\n");
}

#[test]
fn example_499() {
	let markdown = "[link](<foo(and(bar)>)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo(and(bar)\">link</a></p>\n");
}

#[test]
fn example_500() {
	let markdown = "[link](foo\\)\\:)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo):\">link</a></p>\n");
}

#[test]
fn example_501() {
	let markdown = "[link](#fragment)\n\n[link](https://example.com#fragment)\n\n[link](https://example.com?foo=3#frag)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"#fragment\">link</a></p>\n<p><a href=\"https://example.com#fragment\">link</a></p>\n<p><a href=\"https://example.com?foo=3#frag\">link</a></p>\n"
	);
}

#[test]
fn example_502() {
	let markdown = "[link](foo\\bar)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo%5Cbar\">link</a></p>\n");
}

#[test]
fn example_503() {
	let markdown = "[link](foo%20b&auml;)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"foo%20b%C3%A4\">link</a></p>\n");
}

#[test]
fn example_504() {
	let markdown = "[link](\"title\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"%22title%22\">link</a></p>\n");
}

#[test]
fn example_505() {
	let markdown = "[link](/url \"title\")\n[link](/url 'title')\n[link](/url (title))\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a></p>\n"
	);
}

#[test]
fn example_506() {
	let markdown = "[link](/url \"title \\\"&quot;\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title &quot;&quot;\">link</a></p>\n"
	);
}

#[test]
fn example_507() {
	let markdown = "[link](/url \"title\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url%C2%A0%22title%22\">link</a></p>\n");
}

#[test]
fn example_508() {
	let markdown = "[link](/url \"title \"and\" title\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>\n"
	);
}

#[test]
fn example_509() {
	let markdown = "[link](/url 'title \"and\" title')\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title &quot;and&quot; title\">link</a></p>\n"
	);
}

#[test]
fn example_510() {
	let markdown = "[link](   /uri\n  \"title\"  )\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\" title=\"title\">link</a></p>\n");
}

#[test]
fn example_511() {
	let markdown = "[link] (/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link] (/uri)</p>\n");
}

#[test]
fn example_512() {
	let markdown = "[link [foo [bar]]](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n");
}

#[test]
fn example_513() {
	let markdown = "[link] bar](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link] bar](/uri)</p>\n");
}

#[test]
fn example_514() {
	let markdown = "[link [bar](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[link <a href=\"/uri\">bar</a></p>\n");
}

#[test]
fn example_515() {
	let markdown = "[link \\[bar](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">link [bar</a></p>\n");
}

#[test]
fn example_516() {
	let markdown = "[link *foo **bar** `#`*](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n"
	);
}

#[test]
fn example_517() {
	let markdown = "[![moon](moon.jpg)](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n"
	);
}

#[test]
fn example_518() {
	let markdown = "[foo [bar](/uri)](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo <a href=\"/uri\">bar</a>](/uri)</p>\n");
}

#[test]
fn example_519() {
	let markdown = "[foo *[bar [baz](/uri)](/uri)*](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo <em>[bar <a href=\"/uri\">baz</a>](/uri)</em>](/uri)</p>\n"
	);
}

#[test]
fn example_520() {
	let markdown = "![[[foo](uri1)](uri2)](uri3)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><img src=\"uri3\" alt=\"[foo](uri2)\" /></p>\n");
}

#[test]
fn example_521() {
	let markdown = "*[foo*](/uri)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<a href=\"/uri\">foo*</a></p>\n");
}

#[test]
fn example_522() {
	let markdown = "[foo *bar](baz*)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"baz*\">foo *bar</a></p>\n");
}

#[test]
fn example_523() {
	let markdown = "*foo [bar* baz]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo [bar</em> baz]</p>\n");
}

#[test]
fn example_524() {
	let markdown = "[foo <bar attr=\"](baz)\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo <bar attr=\"](baz)\"></p>\n");
}

#[test]
fn example_525() {
	let markdown = "[foo`](/uri)`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo<code>](/uri)</code></p>\n");
}

#[test]
fn example_526() {
	let markdown = "[foo<https://example.com/?search=](uri)>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo<a href=\"https://example.com/?search=%5D(uri)\">https://example.com/?search=](uri)</a></p>\n"
	);
}

#[test]
fn example_527() {
	let markdown = "[foo][bar]\n\n[bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">foo</a></p>\n");
}

#[test]
fn example_528() {
	let markdown = "[link [foo [bar]]][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n");
}

#[test]
fn example_529() {
	let markdown = "[link \\[bar][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">link [bar</a></p>\n");
}

#[test]
fn example_530() {
	let markdown = "[link *foo **bar** `#`*][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n"
	);
}

#[test]
fn example_531() {
	let markdown = "[![moon](moon.jpg)][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n"
	);
}

#[test]
fn example_532() {
	let markdown = "[foo [bar](/uri)][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo <a href=\"/uri\">bar</a>]<a href=\"/uri\">ref</a></p>\n"
	);
}

#[test]
fn example_533() {
	let markdown = "[foo *bar [baz][ref]*][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo <em>bar <a href=\"/uri\">baz</a></em>]<a href=\"/uri\">ref</a></p>\n"
	);
}

#[test]
fn example_534() {
	let markdown = "*[foo*][ref]\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<a href=\"/uri\">foo*</a></p>\n");
}

#[test]
fn example_535() {
	let markdown = "[foo *bar][ref]*\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">foo *bar</a>*</p>\n");
}

#[test]
fn example_536() {
	let markdown = "[foo <bar attr=\"][ref]\">\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo <bar attr=\"][ref]\"></p>\n");
}

#[test]
fn example_537() {
	let markdown = "[foo`][ref]`\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo<code>][ref]</code></p>\n");
}

#[test]
fn example_538() {
	let markdown = "[foo<https://example.com/?search=][ref]>\n\n[ref]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo<a href=\"https://example.com/?search=%5D%5Bref%5D\">https://example.com/?search=][ref]</a></p>\n"
	);
}

#[test]
fn example_539() {
	let markdown = "[foo][BaR]\n\n[bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">foo</a></p>\n");
}

#[test]
fn example_540() {
	let markdown = "[ẞ]\n\n[SS]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\">ẞ</a></p>\n");
}

#[test]
fn example_541() {
	let markdown = "[Foo\n  bar]: /url\n\n[Baz][Foo bar]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\">Baz</a></p>\n");
}

#[test]
fn example_542() {
	let markdown = "[foo] [bar]\n\n[bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo] <a href=\"/url\" title=\"title\">bar</a></p>\n"
	);
}

#[test]
fn example_543() {
	let markdown = "[foo]\n[bar]\n\n[bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo]\n<a href=\"/url\" title=\"title\">bar</a></p>\n"
	);
}

#[test]
fn example_544() {
	let markdown = "[foo]: /url1\n\n[foo]: /url2\n\n[bar][foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url1\">bar</a></p>\n");
}

#[test]
fn example_545() {
	let markdown = "[bar][foo\\!]\n\n[foo!]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[bar][foo!]</p>\n");
}

#[test]
fn example_546() {
	let markdown = "[foo][ref[]\n\n[ref[]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo][ref[]</p>\n<p>[ref[]: /uri</p>\n");
}

#[test]
fn example_547() {
	let markdown = "[foo][ref[bar]]\n\n[ref[bar]]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo][ref[bar]]</p>\n<p>[ref[bar]]: /uri</p>\n");
}

#[test]
fn example_548() {
	let markdown = "[[[foo]]]\n\n[[[foo]]]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[[[foo]]]</p>\n<p>[[[foo]]]: /url</p>\n");
}

#[test]
fn example_549() {
	let markdown = "[foo][ref\\[]\n\n[ref\\[]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">foo</a></p>\n");
}

#[test]
fn example_550() {
	let markdown = "[bar\\\\]: /uri\n\n[bar\\\\]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/uri\">bar\\</a></p>\n");
}

#[test]
fn example_551() {
	let markdown = "[]\n\n[]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[]</p>\n<p>[]: /uri</p>\n");
}

#[test]
fn example_552() {
	let markdown = "[\n ]\n\n[\n ]: /uri\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[\n]</p>\n<p>[\n]: /uri</p>\n");
}

#[test]
fn example_553() {
	let markdown = "[foo][]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">foo</a></p>\n");
}

#[test]
fn example_554() {
	let markdown = "[*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n"
	);
}

#[test]
fn example_555() {
	let markdown = "[Foo][]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n");
}

#[test]
fn example_556() {
	let markdown = "[foo] \n[]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title\">foo</a>\n[]</p>\n"
	);
}

#[test]
fn example_557() {
	let markdown = "[foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">foo</a></p>\n");
}

#[test]
fn example_558() {
	let markdown = "[*foo* bar]\n\n[*foo* bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n"
	);
}

#[test]
fn example_559() {
	let markdown = "[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[<a href=\"/url\" title=\"title\"><em>foo</em> bar</a>]</p>\n"
	);
}

#[test]
fn example_560() {
	let markdown = "[[bar [foo]\n\n[foo]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[[bar <a href=\"/url\">foo</a></p>\n");
}

#[test]
fn example_561() {
	let markdown = "[Foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n");
}

#[test]
fn example_562() {
	let markdown = "[foo] bar\n\n[foo]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\">foo</a> bar</p>\n");
}

#[test]
fn example_563() {
	let markdown = "\\[foo]\n\n[foo]: /url \"title\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]</p>\n");
}

#[test]
fn example_564() {
	let markdown = "[foo*]: /url\n\n*[foo*]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<a href=\"/url\">foo*</a></p>\n");
}

#[test]
fn example_565() {
	let markdown = "[foo][bar]\n\n[foo]: /url1\n[bar]: /url2\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url2\">foo</a></p>\n");
}

#[test]
fn example_566() {
	let markdown = "[foo][]\n\n[foo]: /url1\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url1\">foo</a></p>\n");
}

#[test]
fn example_567() {
	let markdown = "[foo]()\n\n[foo]: /url1\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"\">foo</a></p>\n");
}

#[test]
fn example_568() {
	let markdown = "[foo](not a link)\n\n[foo]: /url1\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url1\">foo</a>(not a link)</p>\n");
}

#[test]
fn example_569() {
	let markdown = "[foo][bar][baz]\n\n[baz]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]<a href=\"/url\">bar</a></p>\n");
}

#[test]
fn example_570() {
	let markdown = "[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url2\">foo</a><a href=\"/url1\">baz</a></p>\n"
	);
}

#[test]
fn example_571() {
	let markdown = "[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]<a href=\"/url1\">bar</a></p>\n");
}
