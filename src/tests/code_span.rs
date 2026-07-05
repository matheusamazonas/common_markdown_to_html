use crate::conversion::convert;

#[test]
fn example_328() {
	let markdown = "`foo`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo</code></p>\n");
}

#[test]
fn example_329() {
	let markdown = "`` foo ` bar ``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo ` bar</code></p>\n");
}

#[test]
fn example_330() {
	let markdown = "` `` `\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>``</code></p>\n");
}

#[test]
fn example_331() {
	let markdown = "`  ``  `\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code> `` </code></p>\n");
}

#[test]
fn example_332() {
	let markdown = "` a`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code> a</code></p>\n");
}

#[test]
fn example_333() {
	let markdown = "` b `\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code> b </code></p>\n");
}

#[test]
fn example_334() {
	let markdown = "` `\n`  `\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code> </code>\n<code>  </code></p>\n");
}

#[test]
fn example_335() {
	let markdown = "``\nfoo\nbar  \nbaz\n``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo bar   baz</code></p>\n");
}

#[test]
fn example_336() {
	let markdown = "``\nfoo \n``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo </code></p>\n");
}

#[test]
fn example_337() {
	let markdown = "`foo   bar \nbaz`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo   bar  baz</code></p>\n");
}

#[test]
fn example_338() {
	let markdown = "`foo\\`bar`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo\\</code>bar`</p>\n");
}

#[test]
fn example_339() {
	let markdown = "``foo`bar``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo`bar</code></p>\n");
}

#[test]
fn example_340() {
	let markdown = "` foo `` bar `\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo `` bar</code></p>\n");
}

#[test]
fn example_341() {
	let markdown = "*foo`*`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*foo<code>*</code></p>\n");
}

#[test]
fn example_342() {
	let markdown = "[not a `link](/foo`)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[not a <code>link](/foo</code>)</p>\n");
}

#[test]
fn example_343() {
	let markdown = "`<a href=\"`\">`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>\n");
}

#[test]
fn example_344() {
	let markdown = "<a href=\"`\">`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a href=\"`\">`</p>\n");
}

#[test]
fn example_345() {
	let markdown = "`<https://foo.bar.`baz>`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>&lt;https://foo.bar.</code>baz&gt;`</p>\n");
}

#[test]
fn example_346() {
	let markdown = "<https://foo.bar.`baz>`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"https://foo.bar.%60baz\">https://foo.bar.`baz</a>`</p>\n"
	);
}

#[test]
fn example_347() {
	let markdown = "```foo``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>```foo``</p>\n");
}

#[test]
fn example_348() {
	let markdown = "`foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>`foo</p>\n");
}

#[test]
fn example_349() {
	let markdown = "`foo``bar``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>`foo<code>bar</code></p>\n");
}
