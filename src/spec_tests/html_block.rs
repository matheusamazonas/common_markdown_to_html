use crate::conversion::convert;

#[test]
fn example_148() {
	let markdown =
		"<table><tr><td>\n<pre>\n**Hello**,\n\n_world_.\n</pre>\n</td></tr></table>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<table><tr><td>\n<pre>\n**Hello**,\n<p><em>world</em>.\n</pre></p>\n</td></tr></table>\n"
	);
}

#[test]
fn example_149() {
	let markdown =
		"<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n\nokay.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n<p>okay.</p>\n"
	);
}

#[test]
fn example_150() {
	let markdown = " <div>\n  *hello*\n         <foo><a>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, " <div>\n  *hello*\n         <foo><a>\n");
}

#[test]
fn example_151() {
	let markdown = "</div>\n*foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "</div>\n*foo*\n");
}

#[test]
fn example_152() {
	let markdown = "<DIV CLASS=\"foo\">\n\n*Markdown*\n\n</DIV>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<DIV CLASS=\"foo\">\n<p><em>Markdown</em></p>\n</DIV>\n"
	);
}

#[test]
fn example_153() {
	let markdown = "<div id=\"foo\"\n  class=\"bar\">\n</div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div id=\"foo\"\n  class=\"bar\">\n</div>\n");
}

#[test]
fn example_154() {
	let markdown = "<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n");
}

#[test]
fn example_155() {
	let markdown = "<div>\n*foo*\n\n*bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div>\n*foo*\n<p><em>bar</em></p>\n");
}

#[test]
fn example_156() {
	let markdown = "<div id=\"foo\"\n*hi*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div id=\"foo\"\n*hi*\n");
}

#[test]
fn example_157() {
	let markdown = "<div class\nfoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div class\nfoo\n");
}

#[test]
fn example_158() {
	let markdown = "<div *???-&&&-<---\n*foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div *???-&&&-<---\n*foo*\n");
}

#[test]
fn example_159() {
	let markdown = "<div><a href=\"bar\">*foo*</a></div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div><a href=\"bar\">*foo*</a></div>\n");
}

#[test]
fn example_160() {
	let markdown = "<table><tr><td>\nfoo\n</td></tr></table>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<table><tr><td>\nfoo\n</td></tr></table>\n");
}

#[test]
fn example_161() {
	let markdown = "<div></div>\n``` c\nint x = 33;\n```\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div></div>\n``` c\nint x = 33;\n```\n");
}

#[test]
fn example_162() {
	let markdown = "<a href=\"foo\">\n*bar*\n</a>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<a href=\"foo\">\n*bar*\n</a>\n");
}

#[test]
fn example_163() {
	let markdown = "<Warning>\n*bar*\n</Warning>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<Warning>\n*bar*\n</Warning>\n");
}

#[test]
fn example_164() {
	let markdown = "<i class=\"foo\">\n*bar*\n</i>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<i class=\"foo\">\n*bar*\n</i>\n");
}

#[test]
fn example_165() {
	let markdown = "</ins>\n*bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "</ins>\n*bar*\n");
}

#[test]
fn example_166() {
	let markdown = "<del>\n*foo*\n</del>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<del>\n*foo*\n</del>\n");
}

#[test]
fn example_167() {
	let markdown = "<del>\n\n*foo*\n\n</del>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<del>\n<p><em>foo</em></p>\n</del>\n");
}

#[test]
fn example_168() {
	let markdown = "<del>*foo*</del>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><del><em>foo</em></del></p>\n");
}

#[test]
fn example_169() {
	let markdown = "<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\nokay\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\n<p>okay</p>\n"
	);
}

#[test]
fn example_170() {
	let markdown = "<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\nokay\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\n<p>okay</p>\n"
	);
}

#[test]
fn example_171() {
	let markdown = "<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n");
}

#[test]
fn example_172() {
	let markdown =
		"<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\nokay\n"
			.chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\n<p>okay</p>\n"
	);
}

#[test]
fn example_173() {
	let markdown = "<style\n  type=\"text/css\">\n\nfoo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<style\n  type=\"text/css\">\n\nfoo\n");
}

#[test]
fn example_174() {
	let markdown = "> <div>\n> foo\n\nbar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<blockquote>\n<div>\nfoo\n</blockquote>\n<p>bar</p>\n"
	);
}

#[test]
fn example_175() {
	let markdown = "- <div>\n- foo\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<ul>\n<li>\n<div>\n</li>\n<li>foo</li>\n</ul>\n");
}

#[test]
fn example_176() {
	let markdown = "<style>p{color:red;}</style>\n*foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<style>p{color:red;}</style>\n<p><em>foo</em></p>\n");
}

#[test]
fn example_177() {
	let markdown = "<!-- foo -->*bar*\n*baz*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<!-- foo -->*bar*\n<p><em>baz</em></p>\n");
}

#[test]
fn example_178() {
	let markdown = "<script>\nfoo\n</script>1. *bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<script>\nfoo\n</script>1. *bar*\n");
}

#[test]
fn example_179() {
	let markdown = "<!-- Foo\n\nbar\n   baz -->\nokay\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<!-- Foo\n\nbar\n   baz -->\n<p>okay</p>\n");
}

#[test]
fn example_180() {
	let markdown = "<?php\n\n  echo '>';\n\n?>\nokay\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<?php\n\n  echo '>';\n\n?>\n<p>okay</p>\n");
}

#[test]
fn example_181() {
	let markdown = "<!DOCTYPE html>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<!DOCTYPE html>\n");
}

#[test]
fn example_182() {
	let markdown = "<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\nokay\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\n<p>okay</p>\n"
	);
}

#[test]
fn example_183() {
	let markdown = "  <!-- foo -->\n\n    <!-- foo -->\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"  <!-- foo -->\n<pre><code>&lt;!-- foo --&gt;\n</code></pre>\n"
	);
}

#[test]
fn example_184() {
	let markdown = "  <div>\n\n    <div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "  <div>\n<pre><code>&lt;div&gt;\n</code></pre>\n");
}

#[test]
fn example_185() {
	let markdown = "Foo\n<div>\nbar\n</div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo</p>\n<div>\nbar\n</div>\n");
}

#[test]
fn example_186() {
	let markdown = "<div>\nbar\n</div>\n*foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div>\nbar\n</div>\n*foo*\n");
}

#[test]
fn example_187() {
	let markdown = "Foo\n<a href=\"bar\">\nbaz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>Foo\n<a href=\"bar\">\nbaz</p>\n");
}

#[test]
fn example_188() {
	let markdown = "<div>\n\n*Emphasized* text.\n\n</div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div>\n<p><em>Emphasized</em> text.</p>\n</div>\n");
}

#[test]
fn example_189() {
	let markdown = "<div>\n*Emphasized* text.\n</div>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<div>\n*Emphasized* text.\n</div>\n");
}

#[test]
fn example_190() {
	let markdown = "<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<table>\n<tr>\n<td>\nHi\n</td>\n</tr>\n</table>\n");
}

#[test]
fn example_191() {
	let markdown =
		"<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<table>\n  <tr>\n<pre><code>&lt;td&gt;\n  Hi\n&lt;/td&gt;\n</code></pre>\n  </tr>\n</table>\n"
	);
}
