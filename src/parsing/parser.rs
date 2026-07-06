use crate::blocks::Block;
use crate::blocks::leaf::Leaf;
use crate::parsing::atx_heading::parse_atx_heading;
use crate::parsing::thematic_break::parse_thematic_break;
use yapcol::{CharToken, Error, Input, StringParser, choice};

pub(crate) trait LeafBlockParser: StringParser<Leaf> {}

impl<T> LeafBlockParser for T where T: StringParser<Leaf> {}

pub fn parse(input: &mut Input<CharToken>) -> Result<Vec<Block>, Error> {
	let mut output = Vec::new();
	let parsers: Vec<Box<dyn LeafBlockParser>> = vec![
		Box::new(parse_thematic_break()),
		Box::new(parse_atx_heading()),
	];
	while let Ok(block) = choice(&parsers)(input) {
		output.push(Block::LeafBlock(block));
	}
	Ok(output)
}
