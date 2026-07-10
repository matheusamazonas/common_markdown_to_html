use crate::blocks::Leaf;
use crate::blocks::leaf::ParagraphLine;
use crate::parsing::atx_heading::parse_atx_heading_opening;
use crate::parsing::parser::LeafBlockParser;
use crate::parsing::thematic_break::parse_thematic_break;
use crate::parsing::utils::{empty_line, line_end, not_line_end, spaces_or_tabs};
use yapcol::{Parser, StringParser, choice, end_of_input, many_until_collect};

fn parse_paragraph_line() -> impl StringParser<String> {
	|input| {
		spaces_or_tabs().maybe()(input)?;
		let parse_not_line_end = not_line_end();
		let parse_line_end = line_end();
		many_until_collect(&parse_not_line_end, &parse_line_end).map(String::from_iter)(input)
	}
}

fn parse_paragraph_end() -> impl StringParser<()> {
	|input| {
		let parse_atx_heading_opening = parse_atx_heading_opening().attempt().discard();
		let parse_thematic_break = parse_thematic_break().attempt().discard();
		let parse_empty_line = empty_line().once_or_more().attempt().discard();
		let parse_end_of_input = end_of_input();
		let parsers: Vec<Box<dyn StringParser<()>>> = vec![
			Box::new(parse_atx_heading_opening),
			Box::new(parse_thematic_break),
			Box::new(parse_empty_line),
			Box::new(parse_end_of_input),
		];
		choice(&parsers)(input)
	}
}
pub(crate) fn parse_paragraph() -> impl LeafBlockParser {
	|input| {
		let parse_line = parse_paragraph_line();
		let parse_end = parse_paragraph_end();
		let lines = parse_line.once_or_more_until_collect(&parse_end)(input)?;
		let mut output = Vec::new();
		for (ix, line) in lines.iter().enumerate() {
			let ends_with_hard_break = ix != lines.len() - 1 && line.ends_with("  ");
			output.push(ParagraphLine::new(
				line.trim_end().to_string(),
				ends_with_hard_break,
			));
		}
		Ok(Leaf::Paragraph(output))
	}
}
