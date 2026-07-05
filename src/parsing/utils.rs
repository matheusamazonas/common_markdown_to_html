use yapcol::{Parser, StringParser, choice, either, is};

pub(crate) trait EmptyParser: StringParser<()> {}

impl<T> EmptyParser for T where T: StringParser<()> {}

pub(crate) fn parse_spaces_or_tabs() -> impl EmptyParser {
	|input| {
		let space = is(' ');
		let tab = is('\t');
		let either = either(&space, &tab);
		either.many().discard()(input)
	}
}

pub(crate) fn parse_line_end() -> impl EmptyParser {
	|input| {
		let line_feed = is('\n').discard();
		let maybe_line_feed = is('\n').maybe();
		let carriage_return = is('\r').and(maybe_line_feed).discard();
		let parsers: Vec<Box<dyn EmptyParser>> =
			vec![Box::new(line_feed), Box::new(carriage_return)];
		choice(&parsers)(input)
	}
}

pub(crate) fn parse_empty_line() -> impl EmptyParser {
	parse_spaces_or_tabs().and(parse_line_end())
}

#[cfg(test)]
mod tests {

	#[cfg(test)]
	mod spaces_or_tabs {
		use crate::parsing::utils::parse_spaces_or_tabs;
		use yapcol::Input;

		#[test]
		fn empty_succeeds() {
			let mut input = Input::new_from_chars("".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn only_spaces_succeeds() {
			let mut input = Input::new_from_chars("     ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn only_tabs_succeeds() {
			let mut input = Input::new_from_chars("\t\t\t".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn tab_after_space_succeeds() {
			let mut input = Input::new_from_chars(" \t".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn space_after_tab_succeeds() {
			let mut input = Input::new_from_chars("\t ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn mixed_succeeds() {
			let mut input = Input::new_from_chars(" \t\t  \t \t  ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}

		#[test]
		fn intruder_stops_parsing_succeeds() {
			let mut input = Input::new_from_chars(" \t\t  \t0 \t  ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
		}
	}

	#[cfg(test)]
	mod line_end {
		use crate::parsing::utils::parse_line_end;
		use yapcol::Input;

		#[test]
		fn line_feed_succeeds() {
			let mut input = Input::new_from_chars("\n".chars(), None);
			parse_line_end()(&mut input).unwrap();
		}

		#[test]
		fn line_break_after_space_fails() {
			let mut input = Input::new_from_chars(" \n".chars(), None);
			assert!(parse_line_end()(&mut input).is_err());
		}

		#[test]
		fn carriage_return_succeeds() {
			let mut input = Input::new_from_chars("\r".chars(), None);
			parse_line_end()(&mut input).unwrap();
		}

		#[test]
		fn carriage_return_after_space_fails() {
			let mut input = Input::new_from_chars(" \r".chars(), None);
			assert!(parse_line_end()(&mut input).is_err());
		}

		#[test]
		fn carriage_return_and_line_feed_succeeds() {
			let mut input = Input::new_from_chars("\r\n".chars(), None);
			parse_line_end()(&mut input).unwrap();
		}

		#[test]
		fn carriage_return_and_line_feed_after_space_fails() {
			let mut input = Input::new_from_chars(" \r\n".chars(), None);
			assert!(parse_line_end()(&mut input).is_err());
		}
	}

	#[cfg(test)]
	mod empty_line {
		use crate::parsing::utils::parse_empty_line;
		use yapcol::Input;

		#[test]
		fn line_feed_succeeds() {
			let mut input = Input::new_from_chars("\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn spaces_and_line_feed_succeeds() {
			// 1 space
			let mut input = Input::new_from_chars(" \n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple spaces
			let mut input = Input::new_from_chars("   \n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn tabs_and_line_feed_succeeds() {
			// 1 tab
			let mut input = Input::new_from_chars("\t\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple tabs
			let mut input = Input::new_from_chars("\t\t\t\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn tabs_and_spaces_mix_and_line_feed_succeeds() {
			let mut input = Input::new_from_chars("\t \t\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("\t \t \n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("   \t\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn carriage_return_succeeds() {
			let mut input = Input::new_from_chars("\r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn spaces_and_carriage_return_succeeds() {
			// 1 space
			let mut input = Input::new_from_chars(" \r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple spaces
			let mut input = Input::new_from_chars("    \r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn tabs_and_carriage_return_succeeds() {
			// 1 tab
			let mut input = Input::new_from_chars("\t\r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple tabs
			let mut input = Input::new_from_chars("\t\t\t\r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn tabs_and_spaces_mix_and_carriage_return_succeeds() {
			let mut input = Input::new_from_chars("\t \t\r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("\t \t \r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("   \t\r".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn carriage_return_and_line_feed_succeeds() {
			let mut input = Input::new_from_chars("\r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn spaces_and_carriage_return_and_line_feed_succeeds() {
			// 1 space
			let mut input = Input::new_from_chars(" \r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple spaces
			let mut input = Input::new_from_chars("    \r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn tabs_and_carriage_return_and_line_feed_succeeds() {
			// 1 tab
			let mut input = Input::new_from_chars("\t\r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			// Multiple tabs
			let mut input = Input::new_from_chars("\t\t\t\r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}

		#[test]
		fn spaces_and_tabs_mix_and_carriage_return_and_line_feed_succeeds() {
			let mut input = Input::new_from_chars("\t \t\r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("\t \t \r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
			let mut input = Input::new_from_chars("   \t\r\n".chars(), None);
			parse_empty_line()(&mut input).unwrap();
		}
	}
}
