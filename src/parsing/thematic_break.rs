use super::parser::LeafBlockParser;
use super::utils::{line_end, spaces_or_tabs};
use crate::blocks::leaf::Leaf;
use yapcol::{Parser, StringParser, choice, is};

fn parse_thematic_break_char(char: char) -> impl StringParser<()> {
	move |input| {
		let optional_spaces = spaces_or_tabs().maybe();
		is(char).and(optional_spaces).at_least(3).discard()(input)
	}
}

pub(crate) fn parse_thematic_break() -> impl LeafBlockParser {
	|input| {
		let parse_optional_leading_spaces = is(' ').up_to(3);
		let parse_dashes = parse_thematic_break_char('-');
		let parse_asterisks = parse_thematic_break_char('*');
		let parse_underscores = parse_thematic_break_char('_');
		let markers = vec![
			Box::new(parse_dashes),
			Box::new(parse_asterisks),
			Box::new(parse_underscores),
		];
		let parse_marker = choice(&markers);
		parse_optional_leading_spaces
			.and(parse_marker)
			.and(spaces_or_tabs().maybe())
			.and(line_end())
			.map(|_| Leaf::ThematicBreak)(input)
	}
}
