use super::parser::LeafBlockParser;
use super::utils::{line_end, spaces_or_tabs, word_with_spaces};
use crate::blocks::leaf::Leaf;
use yapcol::{Parser, StringInput, StringParser, is, look_ahead};

fn is_marker_word(word: &String) -> bool {
	word.len() > 0 && word.chars().all(|c| c == '#')
}

pub(crate) fn parse_atx_heading_opening() -> impl StringParser<u8> {
	|input: &mut StringInput| {
		let level = is(' ').up_to(3).and(is('#').once_up_to(6))(input)?;
		// Corner case: only look for spaces if the next character is not a line end.
		if look_ahead(&line_end())(input).is_err() {
			spaces_or_tabs()(input)?;
		}
		Ok(level as u8)
	}
}

pub(crate) fn parse_atx_heading() -> impl LeafBlockParser {
	|input| {
		let parse_opening = parse_atx_heading_opening();
		let level = parse_opening(input)?;
		let mut words = word_with_spaces().many_collect()(input)?;
		if words.len() > 0 && is_marker_word(&words[words.len() - 1]) {
			words.pop();
		}
		line_end()(input)?;
		Ok(Leaf::ATXHeading(level, words.join(" ")))
	}
}
