use super::parser::LeafBlockParser;
use super::utils::{parse_line_end, parse_spaces_or_tabs, parse_word_with_spaces};
use crate::blocks::leaf::Leaf;
use yapcol::{Parser, is, many_until_collect};

pub(crate) fn parse_atx_heading() -> impl LeafBlockParser {
	|input| {
		let level = is('#').once_up_to(6)(input)?;
		parse_spaces_or_tabs()(input)?;
		let parse_words = parse_word_with_spaces();
		let parse_closing = is('#')
			.many()
			.and(parse_spaces_or_tabs())
			.and(parse_line_end());
		let words = many_until_collect(&parse_words, &parse_closing)(input)?;
		Ok(Leaf::ATXHeading(level as u8, words.join(" ")))
	}
}
