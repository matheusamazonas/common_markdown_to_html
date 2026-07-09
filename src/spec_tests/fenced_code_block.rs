use crate::conversion::convert;

#[test]
fn example_119() {
	let markdown = "```\n<\n >\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>&lt;\n &gt;\n</code></pre>\n");
}

#[test]
fn example_120() {
	let markdown = "~~~\n<\n >\n~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>&lt;\n &gt;\n</code></pre>\n");
}

#[test]
fn example_121() {
	let markdown = "``\nfoo\n``\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>foo</code></p>\n");
}

#[test]
fn example_122() {
	let markdown = "```\naaa\n~~~\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n~~~\n</code></pre>\n");
}

#[test]
fn example_123() {
	let markdown = "~~~\naaa\n```\n~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n```\n</code></pre>\n");
}

#[test]
fn example_124() {
	let markdown = "````\naaa\n```\n``````\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n```\n</code></pre>\n");
}

#[test]
fn example_125() {
	let markdown = "~~~~\naaa\n~~~\n~~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n~~~\n</code></pre>\n");
}

#[test]
fn example_126() {
	let markdown = "```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code></code></pre>\n");
}

#[test]
fn example_127() {
	let markdown = "`````\n\n```\naaa\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>\n```\naaa\n</code></pre>\n");
}

#[test]
fn example_128() {
	let markdown = "> ```\n> aaa\n\nbbb\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<pre><code>aaa\n</code></pre>\n</blockquote>\n<p>bbb</p>\n"
	);
}

#[test]
fn example_129() {
	let markdown = "```\n\n  \n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>\n  \n</code></pre>\n");
}

#[test]
fn example_130() {
	let markdown = "```\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code></code></pre>\n");
}

#[test]
fn example_131() {
	let markdown = " ```\n aaa\naaa\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\naaa\n</code></pre>\n");
}

#[test]
fn example_132() {
	let markdown = "  ```\naaa\n  aaa\naaa\n  ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\naaa\naaa\n</code></pre>\n");
}

#[test]
fn example_133() {
	let markdown = "   ```\n   aaa\n    aaa\n  aaa\n   ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n aaa\naaa\n</code></pre>\n");
}

#[test]
fn example_134() {
	let markdown = "    ```\n    aaa\n    ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>```\naaa\n```\n</code></pre>\n");
}

#[test]
fn example_135() {
	let markdown = "```\naaa\n  ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n</code></pre>\n");
}

#[test]
fn example_136() {
	let markdown = "   ```\naaa\n  ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n</code></pre>\n");
}

#[test]
fn example_137() {
	let markdown = "```\naaa\n    ```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n    ```\n</code></pre>\n");
}

#[test]
fn example_138() {
	let markdown = "``` ```\naaa\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code> </code>\naaa</p>\n");
}

#[test]
fn example_139() {
	let markdown = "~~~~~~\naaa\n~~~ ~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>aaa\n~~~ ~~\n</code></pre>\n");
}

#[test]
fn example_140() {
	let markdown = "foo\n```\nbar\n```\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n"
	);
}

#[test]
fn example_141() {
	let markdown = "foo\n---\n~~~\nbar\n~~~\n# baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<h2>foo</h2>\n<pre><code>bar\n</code></pre>\n<h1>baz</h1>\n"
	);
}

#[test]
fn example_142() {
	let markdown = "```ruby\ndef foo(x)\n  return 3\nend\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n"
	);
}

#[test]
fn example_143() {
	let markdown = "~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n"
	);
}

#[test]
fn example_144() {
	let markdown = "````;\n````\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code class=\"language-;\"></code></pre>\n");
}

#[test]
fn example_145() {
	let markdown = "``` aa ```\nfoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><code>aa</code>\nfoo</p>\n");
}

#[test]
fn example_146() {
	let markdown = "~~~ aa ``` ~~~\nfoo\n~~~\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre><code class=\"language-aa\">foo\n</code></pre>\n"
	);
}

#[test]
fn example_147() {
	let markdown = "```\n``` aaa\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<pre><code>``` aaa\n</code></pre>\n");
}
