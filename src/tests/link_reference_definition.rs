use crate::conversion::convert;

#[test]
fn example_192() {
	let markdown = "[foo]: /url \"title\"\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\" title=\"title\">foo</a></p>\n");
}

#[test]
fn example_193() {
	let markdown = "   [foo]: \n      /url  \n           'the title'  \n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"the title\">foo</a></p>\n"
	);
}

#[test]
fn example_194() {
	let markdown = "[Foo*bar\\]]:my_(url) 'title (with parens)'\n\n[Foo*bar\\]]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"my_(url)\" title=\"title (with parens)\">Foo*bar]</a></p>\n"
	);
}

#[test]
fn example_195() {
	let markdown = "[Foo bar]:\n<my url>\n'title'\n\n[Foo bar]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"my%20url\" title=\"title\">Foo bar</a></p>\n"
	);
}

#[test]
fn example_196() {
	let markdown = "[foo]: /url '\ntitle\nline1\nline2\n'\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\" title=\"\ntitle\nline1\nline2\n\">foo</a></p>\n"
	);
}

#[test]
fn example_197() {
	let markdown = "[foo]: /url 'title\n\nwith blank line'\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>[foo]: /url 'title</p>\n<p>with blank line'</p>\n<p>[foo]</p>\n"
	);
}

#[test]
fn example_198() {
	let markdown = "[foo]:\n/url\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\">foo</a></p>\n");
}

#[test]
fn example_199() {
	let markdown = "[foo]:\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]:</p>\n<p>[foo]</p>\n");
}

#[test]
fn example_200() {
	let markdown = "[foo]: <>\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"\">foo</a></p>\n");
}

#[test]
fn example_201() {
	let markdown = "[foo]: <bar>(baz)\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]: <bar>(baz)</p>\n<p>[foo]</p>\n");
}

#[test]
fn example_202() {
	let markdown = "[foo]: /url\\bar\\*baz \"foo\\\"bar\\baz\"\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url%5Cbar*baz\" title=\"foo&quot;bar\\baz\">foo</a></p>\n"
	);
}

#[test]
fn example_203() {
	let markdown = "[foo]\n\n[foo]: url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"url\">foo</a></p>\n");
}

#[test]
fn example_204() {
	let markdown = "[foo]\n\n[foo]: first\n[foo]: second\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"first\">foo</a></p>\n");
}

#[test]
fn example_205() {
	let markdown = "[FOO]: /url\n\n[Foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/url\">Foo</a></p>\n");
}

#[test]
fn example_206() {
	let markdown = "[ΑΓΩ]: /φου\n\n[αγω]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"/%CF%86%CE%BF%CF%85\">αγω</a></p>\n");
}

#[test]
fn example_207() {
	let markdown = "[foo]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "");
}

#[test]
fn example_208() {
	let markdown = "[\nfoo\n]: /url\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>bar</p>\n");
}

#[test]
fn example_209() {
	let markdown = "[foo]: /url \"title\" ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[foo]: /url &quot;title&quot; ok</p>\n");
}

#[test]
fn example_210() {
	let markdown = "[foo]: /url\n\"title\" ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&quot;title&quot; ok</p>\n");
}

#[test]
fn example_211() {
	let markdown = "    [foo]: /url \"title\"\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>[foo]: /url &quot;title&quot;\n</code></pre>\n<p>[foo]</p>\n"
	);
}

#[test]
fn example_212() {
	let markdown = "```\n[foo]: /url\n```\n\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>[foo]: /url\n</code></pre>\n<p>[foo]</p>\n"
	);
}

#[test]
fn example_213() {
	let markdown = "Foo\n[bar]: /baz\n\n[bar]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\n[bar]: /baz</p>\n<p>[bar]</p>\n");
}

#[test]
fn example_214() {
	let markdown = "# [Foo]\n[foo]: /url\n> bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h1><a href=\"/url\">Foo</a></h1>\n<blockquote>\n<p>bar</p>\n</blockquote>\n"
	);
}

#[test]
fn example_215() {
	let markdown = "[foo]: /url\nbar\n===\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<h1>bar</h1>\n<p><a href=\"/url\">foo</a></p>\n");
}

#[test]
fn example_216() {
	let markdown = "[foo]: /url\n===\n[foo]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>===\n<a href=\"/url\">foo</a></p>\n");
}

#[test]
fn example_217() {
	let markdown = "[foo]: /foo-url \"foo\"\n[bar]: /bar-url\n  \"bar\"\n[baz]: /baz-url\n\n[foo],\n[bar],\n[baz]\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/foo-url\" title=\"foo\">foo</a>,\n<a href=\"/bar-url\" title=\"bar\">bar</a>,\n<a href=\"/baz-url\">baz</a></p>\n"
	);
}

#[test]
fn example_218() {
	let markdown = "[foo]\n\n> [foo]: /url\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/url\">foo</a></p>\n<blockquote>\n</blockquote>\n"
	);
}
