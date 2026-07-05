use crate::conversion::convert;

#[test]
fn example_613() {
	let markdown = "<a><bab><c2c>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a><bab><c2c></p>\n");
}

#[test]
fn example_614() {
	let markdown = "<a/><b2/>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a/><b2/></p>\n");
}

#[test]
fn example_615() {
	let markdown = "<a  /><b2\ndata=\"foo\" >\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><a  /><b2\ndata=\"foo\" ></p>\n");
}

#[test]
fn example_616() {
	let markdown = "<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 /></p>\n"
	);
}

#[test]
fn example_617() {
	let markdown = "Foo <responsive-image src=\"foo.jpg\" />\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo <responsive-image src=\"foo.jpg\" /></p>\n");
}

#[test]
fn example_618() {
	let markdown = "<33> <__>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;33&gt; &lt;__&gt;</p>\n");
}

#[test]
fn example_619() {
	let markdown = "<a h*#ref=\"hi\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>\n");
}

#[test]
fn example_620() {
	let markdown = "<a href=\"hi'> <a href=hi'>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>\n");
}

#[test]
fn example_621() {
	let markdown = "< a><\nfoo><bar/ >\n<foo bar=baz\nbim!bop />\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>&lt; a&gt;&lt;\nfoo&gt;&lt;bar/ &gt;\n&lt;foo bar=baz\nbim!bop /&gt;</p>\n"
	);
}

#[test]
fn example_622() {
	let markdown = "<a href='bar'title=title>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;a href='bar'title=title&gt;</p>\n");
}

#[test]
fn example_623() {
	let markdown = "</a></foo >\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p></a></foo ></p>\n");
}

#[test]
fn example_624() {
	let markdown = "</a href=\"foo\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;/a href=&quot;foo&quot;&gt;</p>\n");
}

#[test]
fn example_625() {
	let markdown = "foo <!-- this is a --\ncomment - with hyphens -->\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>foo <!-- this is a --\ncomment - with hyphens --></p>\n"
	);
}

#[test]
fn example_626() {
	let markdown = "foo <!--> foo -->\n\nfoo <!---> foo -->\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>foo <!--> foo --&gt;</p>\n<p>foo <!---> foo --&gt;</p>\n"
	);
}

#[test]
fn example_627() {
	let markdown = "foo <?php echo $a; ?>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <?php echo $a; ?></p>\n");
}

#[test]
fn example_628() {
	let markdown = "foo <!ELEMENT br EMPTY>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <!ELEMENT br EMPTY></p>\n");
}

#[test]
fn example_629() {
	let markdown = "foo <![CDATA[>&<]]>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <![CDATA[>&<]]></p>\n");
}

#[test]
fn example_630() {
	let markdown = "foo <a href=\"&ouml;\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <a href=\"&ouml;\"></p>\n");
}

#[test]
fn example_631() {
	let markdown = "foo <a href=\"\\*\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <a href=\"\\*\"></p>\n");
}

#[test]
fn example_632() {
	let markdown = "<a href=\"\\\"\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>&lt;a href=&quot;&quot;&quot;&gt;</p>\n");
}
