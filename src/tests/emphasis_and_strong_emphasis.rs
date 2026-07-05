use crate::conversion::convert;

#[test]
fn example_350() {
	let markdown = "*foo bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo bar</em></p>\n");
}

#[test]
fn example_351() {
	let markdown = "a * foo bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>a * foo bar*</p>\n");
}

#[test]
fn example_352() {
	let markdown = "a*\"foo\"*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>a*&quot;foo&quot;*</p>\n");
}

#[test]
fn example_353() {
	let markdown = "* a *\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>* a *</p>\n");
}

#[test]
fn example_354() {
	let markdown = "*$*alpha.\n\n*£*bravo.\n\n*€*charlie.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>*$*alpha.</p>\n<p>*£*bravo.</p>\n<p>*€*charlie.</p>\n"
	);
}

#[test]
fn example_355() {
	let markdown = "foo*bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<em>bar</em></p>\n");
}

#[test]
fn example_356() {
	let markdown = "5*6*78\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>5<em>6</em>78</p>\n");
}

#[test]
fn example_357() {
	let markdown = "_foo bar_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo bar</em></p>\n");
}

#[test]
fn example_358() {
	let markdown = "_ foo bar_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_ foo bar_</p>\n");
}

#[test]
fn example_359() {
	let markdown = "a_\"foo\"_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>a_&quot;foo&quot;_</p>\n");
}

#[test]
fn example_360() {
	let markdown = "foo_bar_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo_bar_</p>\n");
}

#[test]
fn example_361() {
	let markdown = "5_6_78\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>5_6_78</p>\n");
}

#[test]
fn example_362() {
	let markdown = "пристаням_стремятся_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>пристаням_стремятся_</p>\n");
}

#[test]
fn example_363() {
	let markdown = "aa_\"bb\"_cc\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>aa_&quot;bb&quot;_cc</p>\n");
}

#[test]
fn example_364() {
	let markdown = "foo-_(bar)_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo-<em>(bar)</em></p>\n");
}

#[test]
fn example_365() {
	let markdown = "_foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_foo*</p>\n");
}

#[test]
fn example_366() {
	let markdown = "*foo bar *\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*foo bar *</p>\n");
}

#[test]
fn example_367() {
	let markdown = "*foo bar\n*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*foo bar\n*</p>\n");
}

#[test]
fn example_368() {
	let markdown = "*(*foo)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*(*foo)</p>\n");
}

#[test]
fn example_369() {
	let markdown = "*(*foo*)*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>(<em>foo</em>)</em></p>\n");
}

#[test]
fn example_370() {
	let markdown = "*foo*bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo</em>bar</p>\n");
}

#[test]
fn example_371() {
	let markdown = "_foo bar _\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_foo bar _</p>\n");
}

#[test]
fn example_372() {
	let markdown = "_(_foo)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_(_foo)</p>\n");
}

#[test]
fn example_373() {
	let markdown = "_(_foo_)_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>(<em>foo</em>)</em></p>\n");
}

#[test]
fn example_374() {
	let markdown = "_foo_bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_foo_bar</p>\n");
}

#[test]
fn example_375() {
	let markdown = "_пристаням_стремятся\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_пристаням_стремятся</p>\n");
}

#[test]
fn example_376() {
	let markdown = "_foo_bar_baz_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo_bar_baz</em></p>\n");
}

#[test]
fn example_377() {
	let markdown = "_(bar)_.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>(bar)</em>.</p>\n");
}

#[test]
fn example_378() {
	let markdown = "**foo bar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo bar</strong></p>\n");
}

#[test]
fn example_379() {
	let markdown = "** foo bar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>** foo bar**</p>\n");
}

#[test]
fn example_380() {
	let markdown = "a**\"foo\"**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>a**&quot;foo&quot;**</p>\n");
}

#[test]
fn example_381() {
	let markdown = "foo**bar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<strong>bar</strong></p>\n");
}

#[test]
fn example_382() {
	let markdown = "__foo bar__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo bar</strong></p>\n");
}

#[test]
fn example_383() {
	let markdown = "__ foo bar__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__ foo bar__</p>\n");
}

#[test]
fn example_384() {
	let markdown = "__\nfoo bar__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__\nfoo bar__</p>\n");
}

#[test]
fn example_385() {
	let markdown = "a__\"foo\"__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>a__&quot;foo&quot;__</p>\n");
}

#[test]
fn example_386() {
	let markdown = "foo__bar__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo__bar__</p>\n");
}

#[test]
fn example_387() {
	let markdown = "5__6__78\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>5__6__78</p>\n");
}

#[test]
fn example_388() {
	let markdown = "пристаням__стремятся__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>пристаням__стремятся__</p>\n");
}

#[test]
fn example_389() {
	let markdown = "__foo, __bar__, baz__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo, <strong>bar</strong>, baz</strong></p>\n"
	);
}

#[test]
fn example_390() {
	let markdown = "foo-__(bar)__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo-<strong>(bar)</strong></p>\n");
}

#[test]
fn example_391() {
	let markdown = "**foo bar **\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>**foo bar **</p>\n");
}

#[test]
fn example_392() {
	let markdown = "**(**foo)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>**(**foo)</p>\n");
}

#[test]
fn example_393() {
	let markdown = "*(**foo**)*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>(<strong>foo</strong>)</em></p>\n");
}

#[test]
fn example_394() {
	let markdown =
		"**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.\n<em>Asclepias physocarpa</em>)</strong></p>\n"
	);
}

#[test]
fn example_395() {
	let markdown = "**foo \"*bar*\" foo**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>\n"
	);
}

#[test]
fn example_396() {
	let markdown = "**foo**bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo</strong>bar</p>\n");
}

#[test]
fn example_397() {
	let markdown = "__foo bar __\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__foo bar __</p>\n");
}

#[test]
fn example_398() {
	let markdown = "__(__foo)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__(__foo)</p>\n");
}

#[test]
fn example_399() {
	let markdown = "_(__foo__)_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>(<strong>foo</strong>)</em></p>\n");
}

#[test]
fn example_400() {
	let markdown = "__foo__bar\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__foo__bar</p>\n");
}

#[test]
fn example_401() {
	let markdown = "__пристаням__стремятся\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__пристаням__стремятся</p>\n");
}

#[test]
fn example_402() {
	let markdown = "__foo__bar__baz__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo__bar__baz</strong></p>\n");
}

#[test]
fn example_403() {
	let markdown = "__(bar)__.\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>(bar)</strong>.</p>\n");
}

#[test]
fn example_404() {
	let markdown = "*foo [bar](/url)*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <a href=\"/url\">bar</a></em></p>\n");
}

#[test]
fn example_405() {
	let markdown = "*foo\nbar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo\nbar</em></p>\n");
}

#[test]
fn example_406() {
	let markdown = "_foo __bar__ baz_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <strong>bar</strong> baz</em></p>\n");
}

#[test]
fn example_407() {
	let markdown = "_foo _bar_ baz_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <em>bar</em> baz</em></p>\n");
}

#[test]
fn example_408() {
	let markdown = "__foo_ bar_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em><em>foo</em> bar</em></p>\n");
}

#[test]
fn example_409() {
	let markdown = "*foo *bar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <em>bar</em></em></p>\n");
}

#[test]
fn example_410() {
	let markdown = "*foo **bar** baz*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <strong>bar</strong> baz</em></p>\n");
}

#[test]
fn example_411() {
	let markdown = "*foo**bar**baz*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo<strong>bar</strong>baz</em></p>\n");
}

#[test]
fn example_412() {
	let markdown = "*foo**bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo**bar</em></p>\n");
}

#[test]
fn example_413() {
	let markdown = "***foo** bar*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em><strong>foo</strong> bar</em></p>\n");
}

#[test]
fn example_414() {
	let markdown = "*foo **bar***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo <strong>bar</strong></em></p>\n");
}

#[test]
fn example_415() {
	let markdown = "*foo**bar***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo<strong>bar</strong></em></p>\n");
}

#[test]
fn example_416() {
	let markdown = "foo***bar***baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo<em><strong>bar</strong></em>baz</p>\n");
}

#[test]
fn example_417() {
	let markdown = "foo******bar*********baz\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>\n"
	);
}

#[test]
fn example_418() {
	let markdown = "*foo **bar *baz* bim** bop*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>\n"
	);
}

#[test]
fn example_419() {
	let markdown = "*foo [*bar*](/url)*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><em>foo <a href=\"/url\"><em>bar</em></a></em></p>\n"
	);
}

#[test]
fn example_420() {
	let markdown = "** is not an empty emphasis\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>** is not an empty emphasis</p>\n");
}

#[test]
fn example_421() {
	let markdown = "**** is not an empty strong emphasis\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>**** is not an empty strong emphasis</p>\n");
}

#[test]
fn example_422() {
	let markdown = "**foo [bar](/url)**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo <a href=\"/url\">bar</a></strong></p>\n"
	);
}

#[test]
fn example_423() {
	let markdown = "**foo\nbar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo\nbar</strong></p>\n");
}

#[test]
fn example_424() {
	let markdown = "__foo _bar_ baz__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo <em>bar</em> baz</strong></p>\n");
}

#[test]
fn example_425() {
	let markdown = "__foo __bar__ baz__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo <strong>bar</strong> baz</strong></p>\n"
	);
}

#[test]
fn example_426() {
	let markdown = "____foo__ bar__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong><strong>foo</strong> bar</strong></p>\n");
}

#[test]
fn example_427() {
	let markdown = "**foo **bar****\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo <strong>bar</strong></strong></p>\n");
}

#[test]
fn example_428() {
	let markdown = "**foo *bar* baz**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo <em>bar</em> baz</strong></p>\n");
}

#[test]
fn example_429() {
	let markdown = "**foo*bar*baz**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo<em>bar</em>baz</strong></p>\n");
}

#[test]
fn example_430() {
	let markdown = "***foo* bar**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong><em>foo</em> bar</strong></p>\n");
}

#[test]
fn example_431() {
	let markdown = "**foo *bar***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo <em>bar</em></strong></p>\n");
}

#[test]
fn example_432() {
	let markdown = "**foo *bar **baz**\nbim* bop**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo <em>bar <strong>baz</strong>\nbim</em> bop</strong></p>\n"
	);
}

#[test]
fn example_433() {
	let markdown = "**foo [*bar*](/url)**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong>foo <a href=\"/url\"><em>bar</em></a></strong></p>\n"
	);
}

#[test]
fn example_434() {
	let markdown = "__ is not an empty emphasis\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__ is not an empty emphasis</p>\n");
}

#[test]
fn example_435() {
	let markdown = "____ is not an empty strong emphasis\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>____ is not an empty strong emphasis</p>\n");
}

#[test]
fn example_436() {
	let markdown = "foo ***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo ***</p>\n");
}

#[test]
fn example_437() {
	let markdown = "foo *\\**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <em>*</em></p>\n");
}

#[test]
fn example_438() {
	let markdown = "foo *_*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <em>_</em></p>\n");
}

#[test]
fn example_439() {
	let markdown = "foo *****\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo *****</p>\n");
}

#[test]
fn example_440() {
	let markdown = "foo **\\***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <strong>*</strong></p>\n");
}

#[test]
fn example_441() {
	let markdown = "foo **_**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <strong>_</strong></p>\n");
}

#[test]
fn example_442() {
	let markdown = "**foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<em>foo</em></p>\n");
}

#[test]
fn example_443() {
	let markdown = "*foo**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo</em>*</p>\n");
}

#[test]
fn example_444() {
	let markdown = "***foo**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<strong>foo</strong></p>\n");
}

#[test]
fn example_445() {
	let markdown = "****foo*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>***<em>foo</em></p>\n");
}

#[test]
fn example_446() {
	let markdown = "**foo***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo</strong>*</p>\n");
}

#[test]
fn example_447() {
	let markdown = "*foo****\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo</em>***</p>\n");
}

#[test]
fn example_448() {
	let markdown = "foo ___\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo ___</p>\n");
}

#[test]
fn example_449() {
	let markdown = "foo _\\__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <em>_</em></p>\n");
}

#[test]
fn example_450() {
	let markdown = "foo _*_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <em>*</em></p>\n");
}

#[test]
fn example_451() {
	let markdown = "foo _____\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo _____</p>\n");
}

#[test]
fn example_452() {
	let markdown = "foo __\\___\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <strong>_</strong></p>\n");
}

#[test]
fn example_453() {
	let markdown = "foo __*__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>foo <strong>*</strong></p>\n");
}

#[test]
fn example_454() {
	let markdown = "__foo_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_<em>foo</em></p>\n");
}

#[test]
fn example_455() {
	let markdown = "_foo__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo</em>_</p>\n");
}

#[test]
fn example_456() {
	let markdown = "___foo__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_<strong>foo</strong></p>\n");
}

#[test]
fn example_457() {
	let markdown = "____foo_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>___<em>foo</em></p>\n");
}

#[test]
fn example_458() {
	let markdown = "__foo___\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo</strong>_</p>\n");
}

#[test]
fn example_459() {
	let markdown = "_foo____\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo</em>___</p>\n");
}

#[test]
fn example_460() {
	let markdown = "**foo**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo</strong></p>\n");
}

#[test]
fn example_461() {
	let markdown = "*_foo_*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em><em>foo</em></em></p>\n");
}

#[test]
fn example_462() {
	let markdown = "__foo__\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong>foo</strong></p>\n");
}

#[test]
fn example_463() {
	let markdown = "_*foo*_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em><em>foo</em></em></p>\n");
}

#[test]
fn example_464() {
	let markdown = "****foo****\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong><strong>foo</strong></strong></p>\n");
}

#[test]
fn example_465() {
	let markdown = "____foo____\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><strong><strong>foo</strong></strong></p>\n");
}

#[test]
fn example_466() {
	let markdown = "******foo******\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><strong><strong><strong>foo</strong></strong></strong></p>\n"
	);
}

#[test]
fn example_467() {
	let markdown = "***foo***\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em><strong>foo</strong></em></p>\n");
}

#[test]
fn example_468() {
	let markdown = "_____foo_____\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><em><strong><strong>foo</strong></strong></em></p>\n"
	);
}

#[test]
fn example_469() {
	let markdown = "*foo _bar* baz_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>foo _bar</em> baz_</p>\n");
}

#[test]
fn example_470() {
	let markdown = "*foo __bar *baz bim__ bam*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p><em>foo <strong>bar *baz bim</strong> bam</em></p>\n"
	);
}

#[test]
fn example_471() {
	let markdown = "**foo **bar baz**\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>**foo <strong>bar baz</strong></p>\n");
}

#[test]
fn example_472() {
	let markdown = "*foo *bar baz*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*foo <em>bar baz</em></p>\n");
}

#[test]
fn example_473() {
	let markdown = "*[bar*](/url)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<a href=\"/url\">bar*</a></p>\n");
}

#[test]
fn example_474() {
	let markdown = "_foo [bar_](/url)\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>_foo <a href=\"/url\">bar_</a></p>\n");
}

#[test]
fn example_475() {
	let markdown = "*<img src=\"foo\" title=\"*\"/>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>*<img src=\"foo\" title=\"*\"/></p>\n");
}

#[test]
fn example_476() {
	let markdown = "**<a href=\"**\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>**<a href=\"**\"></p>\n");
}

#[test]
fn example_477() {
	let markdown = "__<a href=\"__\">\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p>__<a href=\"__\"></p>\n");
}

#[test]
fn example_478() {
	let markdown = "*a `*`*\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>a <code>*</code></em></p>\n");
}

#[test]
fn example_479() {
	let markdown = "_a `_`_\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(html, "<p><em>a <code>_</code></em></p>\n");
}

#[test]
fn example_480() {
	let markdown = "**a<https://foo.bar/?q=**>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>**a<a href=\"https://foo.bar/?q=**\">https://foo.bar/?q=**</a></p>\n"
	);
}

#[test]
fn example_481() {
	let markdown = "__a<https://foo.bar/?q=__>\n".chars();
	let html = convert(markdown, None).unwrap();
	assert_eq!(
		html,
		"<p>__a<a href=\"https://foo.bar/?q=__\">https://foo.bar/?q=__</a></p>\n"
	);
}
