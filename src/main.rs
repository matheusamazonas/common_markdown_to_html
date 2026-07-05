use crate::conversion::convert;
use std::io;

mod blocks;
mod conversion;
mod parsing;
mod printing;

fn main() {
	let stdin = io::stdin();
	let input = &mut String::new();

	loop {
		println!("Enter markdown, or 'q' to quit:");
		input.clear();
		match stdin.read_line(input) {
			Ok(_) if input.len() == 2 && input.starts_with('q') => break,
			Ok(_) => match convert(input.chars(), Some("stdin".to_string())) {
				Ok(html) => println!("Success: {:?}", html),
				Err(e) => println!("Failed to parse markdown: {e}"),
			},
			Err(_) => println!("Failed to read input."),
		}
	}
}
