use crate::blocks::leaf_block::LeafBlock;
use crate::parsing::thematic_break::parse_thematic_break;
use yapcol::{CharToken, Error, Input, Parser, StringParser};

pub(crate) trait LeafBlockParser: StringParser<LeafBlock> {}

impl<T> LeafBlockParser for T where T: StringParser<LeafBlock> {}

pub fn parse(input: &mut Input<CharToken>) -> Result<LeafBlock, Error> {
	let thematic_break = parse_thematic_break().exhaustive();
	thematic_break(input)
}
