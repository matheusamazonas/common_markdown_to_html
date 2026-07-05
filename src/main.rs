use crate::parsing::parse;
use std::io;
use yapcol::Input;

mod blocks;
mod parsing;

fn main() {
	let stdin = io::stdin();
	let input = &mut String::new();

	loop {
		println!("Enter markdown, or 'q' to quit:");
		input.clear();
		match stdin.read_line(input) {
			Ok(_) if input.len() == 2 && input.starts_with('q') => break,
			Ok(_) => {
				let mut input = Input::new_from_chars(input.chars(), Some("stdin".to_string()));
				match parse(&mut input) {
					Ok(m) => println!("Success: {:?}", m),
					Err(e) => println!("Failed to parse markdown: {e}"),
				}
			}
			Err(_) => println!("Failed to read input."),
		}
	}
}
