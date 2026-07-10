use crate::blocks::Block;
use crate::blocks::leaf::Leaf;
use crate::parsing::atx_heading::parse_atx_heading;
use crate::parsing::paragraph::parse_paragraph;
use crate::parsing::thematic_break::parse_thematic_break;
use yapcol::{CharToken, Error, Input, StringParser, attempt, choice};

pub(crate) trait LeafBlockParser: StringParser<Leaf> {}

impl<T> LeafBlockParser for T where T: StringParser<Leaf> {}

fn parse_leaf() -> impl LeafBlockParser {
	|input| {
		let parse_thematic_break = parse_thematic_break();
		let parse_atx_heading = parse_atx_heading();
		let parsers: Vec<Box<dyn LeafBlockParser>> = vec![
			Box::new(attempt(&parse_thematic_break)),
			Box::new(attempt(&parse_atx_heading)),
			Box::new(parse_paragraph()),
		];
		choice(&parsers)(input)
	}
}

pub fn parse(input: &mut Input<CharToken>) -> Result<Vec<Block>, Error> {
	let mut output = Vec::new();
	let parse_block = parse_leaf();
	while let Ok(leaf) = parse_block(input) {
		output.push(Block::LeafBlock(leaf));
	}
	Ok(output)
}
