use crate::conversion::convert;

#[test]
fn example_228() {
	let markdown = "> # Foo\n> bar\n> baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n"
	);
}

#[test]
fn example_229() {
	let markdown = "># Foo\n>bar\n> baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n"
	);
}

#[test]
fn example_230() {
	let markdown = "   > # Foo\n   > bar\n > baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n"
	);
}

#[test]
fn example_231() {
	let markdown = "    > # Foo\n    > bar\n    > baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>&gt; # Foo\n&gt; bar\n&gt; baz\n</code></pre>\n"
	);
}

#[test]
fn example_232() {
	let markdown = "> # Foo\n> bar\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n"
	);
}

#[test]
fn example_233() {
	let markdown = "> bar\nbaz\n> foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>bar\nbaz\nfoo</p>\n</blockquote>\n");
}

#[test]
fn example_234() {
	let markdown = "> foo\n---\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n");
}

#[test]
fn example_235() {
	let markdown = "> - foo\n- bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<ul>\n<li>foo</li>\n</ul>\n</blockquote>\n<ul>\n<li>bar</li>\n</ul>\n"
	);
}

#[test]
fn example_236() {
	let markdown = ">     foo\n    bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<pre><code>foo\n</code></pre>\n</blockquote>\n<pre><code>bar\n</code></pre>\n"
	);
}

#[test]
fn example_237() {
	let markdown = "> ```\nfoo\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<pre><code></code></pre>\n</blockquote>\n<p>foo</p>\n<pre><code></code></pre>\n"
	);
}

#[test]
fn example_238() {
	let markdown = "> foo\n    - bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo\n- bar</p>\n</blockquote>\n");
}

#[test]
fn example_239() {
	let markdown = ">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n</blockquote>\n");
}

#[test]
fn example_240() {
	let markdown = ">\n>  \n> \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n</blockquote>\n");
}

#[test]
fn example_241() {
	let markdown = ">\n> foo\n>  \n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo</p>\n</blockquote>\n");
}

#[test]
fn example_242() {
	let markdown = "> foo\n\n> bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<p>foo</p>\n</blockquote>\n<blockquote>\n<p>bar</p>\n</blockquote>\n"
	);
}

#[test]
fn example_243() {
	let markdown = "> foo\n> bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n");
}

#[test]
fn example_244() {
	let markdown = "> foo\n>\n> bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<p>foo</p>\n<p>bar</p>\n</blockquote>\n"
	);
}

#[test]
fn example_245() {
	let markdown = "foo\n> bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>foo</p>\n<blockquote>\n<p>bar</p>\n</blockquote>\n"
	);
}

#[test]
fn example_246() {
	let markdown = "> aaa\n***\n> bbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<p>aaa</p>\n</blockquote>\n<hr />\n<blockquote>\n<p>bbb</p>\n</blockquote>\n"
	);
}

#[test]
fn example_247() {
	let markdown = "> bar\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<blockquote>\n<p>bar\nbaz</p>\n</blockquote>\n");
}

#[test]
fn example_248() {
	let markdown = "> bar\n\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n"
	);
}

#[test]
fn example_249() {
	let markdown = "> bar\n>\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n"
	);
}

#[test]
fn example_250() {
	let markdown = "> > > foo\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n</blockquote>\n</blockquote>\n"
	);
}

#[test]
fn example_251() {
	let markdown = ">>> foo\n> bar\n>>baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar\nbaz</p>\n</blockquote>\n</blockquote>\n</blockquote>\n"
	);
}

#[test]
fn example_252() {
	let markdown = ">     code\n\n>    not code\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<pre><code>code\n</code></pre>\n</blockquote>\n<blockquote>\n<p>not code</p>\n</blockquote>\n"
	);
}
