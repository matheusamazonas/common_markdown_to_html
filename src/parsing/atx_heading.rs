use super::parser::LeafBlockParser;
use super::utils::{line_end, spaces_or_tabs, word_with_spaces};
use crate::blocks::leaf::Leaf;
use yapcol::{Parser, StringInput, is, look_ahead};

fn is_marker_word(word: &String) -> bool {
	word.len() > 0 && word.chars().all(|c| c == '#')
}

pub(crate) fn parse_atx_heading() -> impl LeafBlockParser {
	|input| {
		let parse_opening = |input: &mut StringInput| {
			let markers = is(' ').up_to(3).and(is('#').once_up_to(6))(input)?;
			// Corner case: only look for spaces if the next character is not a line end.
			if look_ahead(&line_end())(input).is_err() {
				spaces_or_tabs()(input)?;
			}
			Ok(markers)
		};
		let level = parse_opening(input)?;
		let mut words = word_with_spaces().many_collect()(input)?;
		if words.len() > 0 && is_marker_word(&words[words.len() - 1]) {
			words.pop();
		}
		line_end()(input)?;
		Ok(Leaf::ATXHeading(level as u8, words.join(" ")))
	}
}
