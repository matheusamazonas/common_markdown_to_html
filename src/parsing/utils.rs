use yapcol::{Parser, StringParser, choice, either, is, satisfy};

pub(crate) trait EmptyParser: StringParser<()> {}

impl<T> EmptyParser for T where T: StringParser<()> {}

pub(crate) fn parse_spaces_or_tabs() -> impl EmptyParser {
	|input| {
		let parse_space = is(' ');
		let parse_tab = is('\t');
		let parse_either = either(&parse_space, &parse_tab);
		parse_either.once_or_more().discard()(input)
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
	parse_spaces_or_tabs().maybe().and(parse_line_end())
}

pub(crate) fn parse_not_empty_space() -> impl StringParser<char> {
	satisfy(|&c| match c {
		' ' => None,
		'\t' => None,
		'\n' => None,
		c => Some(c),
	})
}

pub(crate) fn parse_word() -> impl StringParser<String> {
	parse_not_empty_space()
		.once_or_more_collect()
		.map(|chars| chars.into_iter().collect())
}

pub(crate) fn parse_word_with_spaces() -> impl StringParser<String> {
	|input| {
		let word = parse_word()(input)?;
		parse_spaces_or_tabs().maybe()(input)?;
		Ok(word)
	}
}

#[cfg(test)]
mod tests {

	#[cfg(test)]
	mod spaces_or_tabs {
		use crate::parsing::utils::parse_spaces_or_tabs;
		use yapcol::{Input, end_of_input};

		#[test]
		fn empty_fails() {
			let mut input = Input::new_from_chars("".chars(), None);
			let output = parse_spaces_or_tabs()(&mut input);
			assert!(output.is_err());
			end_of_input()(&mut input).unwrap();
		}

		#[test]
		fn only_spaces_succeeds() {
			let mut input = Input::new_from_chars("     ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
			end_of_input()(&mut input).unwrap();
		}

		#[test]
		fn only_tabs_succeeds() {
			let mut input = Input::new_from_chars("\t\t\t".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
			end_of_input()(&mut input).unwrap();
		}

		#[test]
		fn tab_after_space_succeeds() {
			let mut input = Input::new_from_chars(" \t".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
			end_of_input()(&mut input).unwrap();
		}

		#[test]
		fn space_after_tab_succeeds() {
			let mut input = Input::new_from_chars("\t ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
			end_of_input()(&mut input).unwrap();
		}

		#[test]
		fn mixed_succeeds() {
			let mut input = Input::new_from_chars(" \t\t  \t \t  ".chars(), None);
			parse_spaces_or_tabs()(&mut input).unwrap();
			end_of_input()(&mut input).unwrap();
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

	#[cfg(test)]
	mod not_empty_space {
		use crate::parsing::utils::parse_not_empty_space;
		use yapcol::Input;

		#[test]
		fn whitespace_fails() {
			let mut input = Input::new_from_chars(" ".chars(), None);
			assert!(parse_not_empty_space()(&mut input).is_err());
		}

		#[test]
		fn tab_fails() {
			let mut input = Input::new_from_chars("\t".chars(), None);
			assert!(parse_not_empty_space()(&mut input).is_err());
		}

		#[test]
		fn line_feed_fails() {
			let mut input = Input::new_from_chars("\n".chars(), None);
			assert!(parse_not_empty_space()(&mut input).is_err());
		}

		#[test]
		fn others_fail() {
			let mut input = Input::new_from_chars("a".chars(), None);
			parse_not_empty_space()(&mut input).unwrap();
			let mut input = Input::new_from_chars("1".chars(), None);
			parse_not_empty_space()(&mut input).unwrap();
			let mut input = Input::new_from_chars("@".chars(), None);
			parse_not_empty_space()(&mut input).unwrap();
		}
	}

	#[cfg(test)]
	mod word {
		use crate::parsing::utils::parse_word;
		use yapcol::Input;

		#[test]
		fn one_word_no_space_succeeds() {
			let mut input = Input::new_from_chars("test".chars(), None);
			let output = parse_word()(&mut input).unwrap();
			assert_eq!(output, "test".to_string());
		}

		#[test]
		fn whitespace_fails() {
			let mut input = Input::new_from_chars(" ".chars(), None);
			assert!(parse_word()(&mut input).is_err());
		}

		#[test]
		fn tab_fails() {
			let mut input = Input::new_from_chars("\t".chars(), None);
			assert!(parse_word()(&mut input).is_err());
		}

		#[test]
		fn one_word_with_space_fails() {
			let mut input = Input::new_from_chars(" test".chars(), None);
			assert!(parse_word()(&mut input).is_err());
		}

		#[test]
		fn one_word_with_tab_fails() {
			let mut input = Input::new_from_chars("\ttest".chars(), None);
			assert!(parse_word()(&mut input).is_err());
		}

		#[test]
		fn line_break_fails() {
			let mut input = Input::new_from_chars("\n".chars(), None);
			assert!(parse_word()(&mut input).is_err());
		}
	}

	#[cfg(test)]
	mod word_with_spaces {
		use crate::parsing::utils::parse_word_with_spaces;
		use yapcol::Input;

		#[test]
		fn no_spaces_succeeds() {
			let mut input = Input::new_from_chars("test".chars(), None);
			let output = parse_word_with_spaces()(&mut input).unwrap();
			assert_eq!(output, String::from("test"));
		}

		#[test]
		fn one_space_succeeds() {
			let mut input = Input::new_from_chars("test ".chars(), None);
			let output = parse_word_with_spaces()(&mut input).unwrap();
			assert_eq!(output, String::from("test"));
		}

		#[test]
		fn one_tab_succeeds() {
			let mut input = Input::new_from_chars("test\t".chars(), None);
			let output = parse_word_with_spaces()(&mut input).unwrap();
			assert_eq!(output, String::from("test"));
		}

		#[test]
		fn many_spaces_succeeds() {
			let mut input = Input::new_from_chars("test    ".chars(), None);
			let output = parse_word_with_spaces()(&mut input).unwrap();
			assert_eq!(output, String::from("test"));
		}

		#[test]
		fn many_tabs_succeeds() {
			let mut input = Input::new_from_chars("test\t\t\t\t\t".chars(), None);
			let output = parse_word_with_spaces()(&mut input).unwrap();
			assert_eq!(output, String::from("test"));
		}
	}
}
