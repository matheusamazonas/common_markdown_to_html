use crate::conversion::convert;

#[test]
fn example_253() {
	let markdown =
		"A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n"
	);
}

#[test]
fn example_254() {
	let markdown =
		"1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.\n"
			.chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_255() {
	let markdown = "- one\n\n two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n");
}

#[test]
fn example_256() {
	let markdown = "- one\n\n  two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n");
}

#[test]
fn example_257() {
	let markdown = " -    one\n\n     two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>one</li>\n</ul>\n<pre><code> two\n</code></pre>\n"
	);
}

#[test]
fn example_258() {
	let markdown = " -    one\n\n      two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n");
}

#[test]
fn example_259() {
	let markdown = "   > > 1.  one\n>>\n>>     two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>\n"
	);
}

#[test]
fn example_260() {
	let markdown = ">>- one\n>>\n  >  > two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<blockquote>\n<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n</blockquote>\n</blockquote>\n"
	);
}

#[test]
fn example_261() {
	let markdown = "-one\n\n2.two\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>-one</p>\n<p>2.two</p>\n");
}

#[test]
fn example_262() {
	let markdown = "- foo\n\n\n  bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n");
}

#[test]
fn example_263() {
	let markdown = "1.  foo\n\n    ```\n    bar\n    ```\n\n    baz\n\n    > bam\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n<blockquote>\n<p>bam</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_264() {
	let markdown = "- Foo\n\n      bar\n\n\n      baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>Foo</p>\n<pre><code>bar\n\n\nbaz\n</code></pre>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_265() {
	let markdown = "123456789. ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ol start=\"123456789\">\n<li>ok</li>\n</ol>\n");
}

#[test]
fn example_266() {
	let markdown = "1234567890. not ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>1234567890. not ok</p>\n");
}

#[test]
fn example_267() {
	let markdown = "0. ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ol start=\"0\">\n<li>ok</li>\n</ol>\n");
}

#[test]
fn example_268() {
	let markdown = "003. ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ol start=\"3\">\n<li>ok</li>\n</ol>\n");
}

#[test]
fn example_269() {
	let markdown = "-1. not ok\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>-1. not ok</p>\n");
}

#[test]
fn example_270() {
	let markdown = "- foo\n\n      bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_271() {
	let markdown = "  10.  foo\n\n           bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol start=\"10\">\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_272() {
	let markdown = "    indented code\n\nparagraph\n\n    more code\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n"
	);
}

#[test]
fn example_273() {
	let markdown = "1.     indented code\n\n   paragraph\n\n       more code\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_274() {
	let markdown = "1.      indented code\n\n   paragraph\n\n       more code\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<pre><code> indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_275() {
	let markdown = "   foo\n\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo</p>\n<p>bar</p>\n");
}

#[test]
fn example_276() {
	let markdown = "-    foo\n\n  bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>foo</li>\n</ul>\n<p>bar</p>\n");
}

#[test]
fn example_277() {
	let markdown = "-  foo\n\n   bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n");
}

#[test]
fn example_278() {
	let markdown = "-\n  foo\n-\n  ```\n  bar\n  ```\n-\n      baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo</li>\n<li>\n<pre><code>bar\n</code></pre>\n</li>\n<li>\n<pre><code>baz\n</code></pre>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_279() {
	let markdown = "-   \n  foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>foo</li>\n</ul>\n");
}

#[test]
fn example_280() {
	let markdown = "-\n\n  foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li></li>\n</ul>\n<p>foo</p>\n");
}

#[test]
fn example_281() {
	let markdown = "- foo\n-\n- bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n");
}

#[test]
fn example_282() {
	let markdown = "- foo\n-   \n- bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n");
}

#[test]
fn example_283() {
	let markdown = "1. foo\n2.\n3. bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ol>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ol>\n");
}

#[test]
fn example_284() {
	let markdown = "*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li></li>\n</ul>\n");
}

#[test]
fn example_285() {
	let markdown = "foo\n*\n\nfoo\n1.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo\n*</p>\n<p>foo\n1.</p>\n");
}

#[test]
fn example_286() {
	let markdown = " 1.  A paragraph\n     with two lines.\n\n         indented code\n\n     > A block quote.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_287() {
	let markdown = "  1.  A paragraph\n      with two lines.\n\n          indented code\n\n      > A block quote.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_288() {
	let markdown = "   1.  A paragraph\n       with two lines.\n\n           indented code\n\n       > A block quote.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_289() {
	let markdown = "    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code>1.  A paragraph\n    with two lines.\n\n        indented code\n\n    &gt; A block quote.\n</code></pre>\n"
	);
}

#[test]
fn example_290() {
	let markdown =
		"  1.  A paragraph\nwith two lines.\n\n          indented code\n\n      > A block quote.\n"
			.chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_291() {
	let markdown = "  1.  A paragraph\n    with two lines.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ol>\n<li>A paragraph\nwith two lines.</li>\n</ol>\n");
}

#[test]
fn example_292() {
	let markdown = "> 1. > Blockquote\ncontinued here.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n"
	);
}

#[test]
fn example_293() {
	let markdown = "> 1. > Blockquote\n> continued here.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n"
	);
}

#[test]
fn example_294() {
	let markdown = "- foo\n  - bar\n    - baz\n      - boo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz\n<ul>\n<li>boo</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_295() {
	let markdown = "- foo\n - bar\n  - baz\n   - boo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo</li>\n<li>bar</li>\n<li>baz</li>\n<li>boo</li>\n</ul>\n"
	);
}

#[test]
fn example_296() {
	let markdown = "10) foo\n    - bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol start=\"10\">\n<li>foo\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_297() {
	let markdown = "10) foo\n   - bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol start=\"10\">\n<li>foo</li>\n</ol>\n<ul>\n<li>bar</li>\n</ul>\n"
	);
}

#[test]
fn example_298() {
	let markdown = "- - foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<ul>\n<li>foo</li>\n</ul>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_299() {
	let markdown = "1. - 2. foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<ul>\n<li>\n<ol start=\"2\">\n<li>foo</li>\n</ol>\n</li>\n</ul>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_300() {
	let markdown = "- # Foo\n- Bar\n  ---\n  baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<h1>Foo</h1>\n</li>\n<li>\n<h2>Bar</h2>\nbaz</li>\n</ul>\n"
	);
}
