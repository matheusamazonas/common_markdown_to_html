use crate::conversion::convert;

#[test]
fn example_25() {
	let markdown = "&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>  &amp; © Æ Ď\n¾ ℋ ⅆ\n∲ ≧̸</p>\n");
}

#[test]
fn example_26() {
	let markdown = "&#35; &#1234; &#992; &#0;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p># Ӓ Ϡ �</p>\n");
}

#[test]
fn example_27() {
	let markdown = "&#X22; &#XD06; &#xcab;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&quot; ആ ಫ</p>\n");
}

#[test]
fn example_28() {
	let markdown =
		"&nbsp &x; &#; &#x;\n&#87654321;\n&#abcdef0;\n&ThisIsNotDefined; &hi?;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>&amp;nbsp &amp;x; &amp;#; &amp;#x;\n&amp;#87654321;\n&amp;#abcdef0;\n&amp;ThisIsNotDefined; &amp;hi?;</p>\n"
	);
}

#[test]
fn example_29() {
	let markdown = "&copy\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&amp;copy</p>\n");
}

#[test]
fn example_30() {
	let markdown = "&MadeUpEntity;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&amp;MadeUpEntity;</p>\n");
}

#[test]
fn example_31() {
	let markdown = "<a href=\"&ouml;&ouml;.html\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<a href=\"&ouml;&ouml;.html\">\n");
}

#[test]
fn example_32() {
	let markdown = "[foo](/f&ouml;&ouml; \"f&ouml;&ouml;\")\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n"
	);
}

#[test]
fn example_33() {
	let markdown = "[foo]\n\n[foo]: /f&ouml;&ouml; \"f&ouml;&ouml;\"\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n"
	);
}

#[test]
fn example_34() {
	let markdown = "``` f&ouml;&ouml;\nfoo\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code class=\"language-föö\">foo\n</code></pre>\n"
	);
}

#[test]
fn example_35() {
	let markdown = "`f&ouml;&ouml;`\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>f&amp;ouml;&amp;ouml;</code></p>\n");
}

#[test]
fn example_36() {
	let markdown = "    f&ouml;f&ouml;\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>f&amp;ouml;f&amp;ouml;\n</code></pre>\n");
}

#[test]
fn example_37() {
	let markdown = "&#42;foo&#42;\n*foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*foo*\n<em>foo</em></p>\n");
}

#[test]
fn example_38() {
	let markdown = "&#42; foo\n\n* foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>* foo</p>\n<ul>\n<li>foo</li>\n</ul>\n");
}

#[test]
fn example_39() {
	let markdown = "foo&#10;&#10;bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\n\nbar</p>\n");
}

#[test]
fn example_40() {
	let markdown = "&#9;foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>\tfoo</p>\n");
}

#[test]
fn example_41() {
	let markdown = "[a](url &quot;tit&quot;)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>[a](url &quot;tit&quot;)</p>\n");
}
