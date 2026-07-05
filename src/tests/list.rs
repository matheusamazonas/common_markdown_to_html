use crate::conversion::convert;

#[test]
fn example_301() {
	let markdown = "- foo\n- bar\n+ baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<ul>\n<li>baz</li>\n</ul>\n"
	);
}

#[test]
fn example_302() {
	let markdown = "1. foo\n2. bar\n3) baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>foo</li>\n<li>bar</li>\n</ol>\n<ol start=\"3\">\n<li>baz</li>\n</ol>\n"
	);
}

#[test]
fn example_303() {
	let markdown = "Foo\n- bar\n- baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>Foo</p>\n<ul>\n<li>bar</li>\n<li>baz</li>\n</ul>\n"
	);
}

#[test]
fn example_304() {
	let markdown = "The number of windows in my house is\n14.  The number of doors is 6.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>The number of windows in my house is\n14.  The number of doors is 6.</p>\n"
	);
}

#[test]
fn example_305() {
	let markdown = "The number of windows in my house is\n1.  The number of doors is 6.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>The number of windows in my house is</p>\n<ol>\n<li>The number of doors is 6.</li>\n</ol>\n"
	);
}

#[test]
fn example_306() {
	let markdown = "- foo\n\n- bar\n\n\n- baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>foo</p>\n</li>\n<li>\n<p>bar</p>\n</li>\n<li>\n<p>baz</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_307() {
	let markdown = "- foo\n  - bar\n    - baz\n\n\n      bim\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>\n<p>baz</p>\n<p>bim</p>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_308() {
	let markdown = "- foo\n- bar\n\n<!-- -->\n\n- baz\n- bim\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<!-- -->\n<ul>\n<li>baz</li>\n<li>bim</li>\n</ul>\n"
	);
}

#[test]
fn example_309() {
	let markdown = "-   foo\n\n    notcode\n\n-   foo\n\n<!-- -->\n\n    code\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>foo</p>\n<p>notcode</p>\n</li>\n<li>\n<p>foo</p>\n</li>\n</ul>\n<!-- -->\n<pre><code>code\n</code></pre>\n"
	);
}

#[test]
fn example_310() {
	let markdown = "- a\n - b\n  - c\n   - d\n  - e\n - f\n- g\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d</li>\n<li>e</li>\n<li>f</li>\n<li>g</li>\n</ul>\n"
	);
}

#[test]
fn example_311() {
	let markdown = "1. a\n\n  2. b\n\n   3. c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_312() {
	let markdown = "- a\n - b\n  - c\n   - d\n    - e\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d\n- e</li>\n</ul>\n"
	);
}

#[test]
fn example_313() {
	let markdown = "1. a\n\n  2. b\n\n    3. c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n</ol>\n<pre><code>3. c\n</code></pre>\n"
	);
}

#[test]
fn example_314() {
	let markdown = "- a\n- b\n\n- c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_315() {
	let markdown = "* a\n*\n\n* c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>a</p>\n</li>\n<li></li>\n<li>\n<p>c</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_316() {
	let markdown = "- a\n- b\n\n  c\n- d\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_317() {
	let markdown = "- a\n- b\n\n  [ref]: /url\n- d\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_318() {
	let markdown = "- a\n- ```\n  b\n\n\n  ```\n- c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a</li>\n<li>\n<pre><code>b\n\n\n</code></pre>\n</li>\n<li>c</li>\n</ul>\n"
	);
}

#[test]
fn example_319() {
	let markdown = "- a\n  - b\n\n    c\n- d\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a\n<ul>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n</ul>\n</li>\n<li>d</li>\n</ul>\n"
	);
}

#[test]
fn example_320() {
	let markdown = "* a\n  > b\n  >\n* c\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n</li>\n<li>c</li>\n</ul>\n"
	);
}

#[test]
fn example_321() {
	let markdown = "- a\n  > b\n  ```\n  c\n  ```\n- d\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n<pre><code>c\n</code></pre>\n</li>\n<li>d</li>\n</ul>\n"
	);
}

#[test]
fn example_322() {
	let markdown = "- a\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>a</li>\n</ul>\n");
}

#[test]
fn example_323() {
	let markdown = "- a\n  - b\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>a\n<ul>\n<li>b</li>\n</ul>\n</li>\n</ul>\n");
}

#[test]
fn example_324() {
	let markdown = "1. ```\n   foo\n   ```\n\n   bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ol>\n<li>\n<pre><code>foo\n</code></pre>\n<p>bar</p>\n</li>\n</ol>\n"
	);
}

#[test]
fn example_325() {
	let markdown = "* foo\n  * bar\n\n  baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n<p>baz</p>\n</li>\n</ul>\n"
	);
}

#[test]
fn example_326() {
	let markdown = "- a\n  - b\n  - c\n\n- d\n  - e\n  - f\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<ul>\n<li>\n<p>a</p>\n<ul>\n<li>b</li>\n<li>c</li>\n</ul>\n</li>\n<li>\n<p>d</p>\n<ul>\n<li>e</li>\n<li>f</li>\n</ul>\n</li>\n</ul>\n"
	);
}
