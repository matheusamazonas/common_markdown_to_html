use super::parser::LeafBlockParser;
use super::utils::{parse_line_end, parse_spaces_or_tabs};
use crate::blocks::leaf::Leaf;
use yapcol::{Parser, StringParser, choice, is, maybe};

fn parse_thematic_break_char(char: char) -> impl StringParser<()> {
	move |input| {
		let spaces = parse_spaces_or_tabs();
		is(char).and(maybe(&spaces)).at_least(3).discard()(input)
	}
}

pub(crate) fn parse_thematic_break() -> impl LeafBlockParser {
	|input| {
		let optional_leading_spaces = is(' ').up_to(3).maybe();
		let dashes = parse_thematic_break_char('-');
		let asterisks = parse_thematic_break_char('*');
		let underscores = parse_thematic_break_char('_');
		let markers = vec![Box::new(dashes), Box::new(asterisks), Box::new(underscores)];
		let marker = choice(&markers);
		optional_leading_spaces
			.and(marker)
			.and(parse_spaces_or_tabs())
			.and(parse_line_end())
			.map(|_| Leaf::ThematicBreak)(input)
	}
}
