use crate::blocks::Block;
use crate::blocks::leaf::Leaf;
use crate::parsing::thematic_break::parse_thematic_break;
use yapcol::{CharToken, Error, Input, Parser, StringParser};

pub(crate) trait LeafBlockParser: StringParser<Leaf> {}

impl<T> LeafBlockParser for T where T: StringParser<Leaf> {}

pub fn parse(input: &mut Input<CharToken>) -> Result<Vec<Block>, Error> {
	let mut output = Vec::new();
	let thematic_break = parse_thematic_break();
	while let Ok(block) = thematic_break(input) {
		output.push(Block::LeafBlock(block));
	}
	Ok(output)
}
