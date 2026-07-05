use crate::parsing::parse;
use crate::printing::print;
use yapcol::{Error, Input};

pub fn convert<I>(input: I, source_name: Option<String>) -> Result<String, Error>
where
	I: Iterator<Item = char>,
{
	let mut input = Input::new_from_chars(input, source_name);
	let blocks = parse(&mut input)?;
	Ok(print(blocks))
}
