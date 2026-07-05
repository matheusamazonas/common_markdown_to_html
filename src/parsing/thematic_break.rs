use super::parser::LeafBlockParser;
use super::utils::{parse_line_end, parse_spaces_or_tabs};
use crate::blocks::leaf_block::LeafBlock;
use yapcol::{Parser, StringParser, choice, is, maybe};

fn parse_thematic_break_char(char: char) -> impl StringParser<()> {
	move |input| {
		let spaces = parse_spaces_or_tabs();
		is(char).and(maybe(&spaces)).at_least(3).discard()(input)
	}
}

pub(crate) fn parse_thematic_break() -> impl LeafBlockParser {
	|input| {
		let optional_leading_spaces = is(' ').up_to(3).maybe();
		let dashes = parse_thematic_break_char('-');
		let asterisks = parse_thematic_break_char('*');
		let underscores = parse_thematic_break_char('_');
		let markers = vec![Box::new(dashes), Box::new(asterisks), Box::new(underscores)];
		let marker = choice(&markers);
		optional_leading_spaces
			.and(marker)
			.and(parse_spaces_or_tabs())
			.and(parse_line_end())
			.map(|_| LeafBlock::ThematicBreak)(input)
	}
}

#[cfg(test)]
mod parsing_tests {
	// TODO: implement the following examples from the specs:
	// - 49
	// - 57 — 61

	use super::*;
	use crate::blocks::leaf_block::LeafBlock;
	use yapcol::Input;

	#[test] // Spec example 43.1
	fn no_spaces_dashes_success() {
		let mut input = Input::new_from_chars("---\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 43.2
	fn no_spaces_asterisks_success() {
		let mut input = Input::new_from_chars("***\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 43.3
	fn no_spaces_underscores_success() {
		let mut input = Input::new_from_chars("___\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 44
	fn wrong_character_fails_plus() {
		let mut input = Input::new_from_chars("+++\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 45
	fn wrong_character_fails_equals() {
		let mut input = Input::new_from_chars("===\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 46.1
	fn not_enough_underscores_fails_2() {
		let mut input = Input::new_from_chars("__\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 46.2
	fn not_enough_dashes_fails_2() {
		let mut input = Input::new_from_chars("--\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 46.3
	fn not_enough_asterisks_fails_2() {
		let mut input = Input::new_from_chars("**\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Not in spec
	fn not_enough_underscores_fails_1() {
		let mut input = Input::new_from_chars("_\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Not in spec
	fn not_enough_dashes_fails_1() {
		let mut input = Input::new_from_chars("-\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Not in spec
	fn not_enough_asterisks_fails_1() {
		let mut input = Input::new_from_chars("*\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Not in spec
	fn one_space_dashes_success() {
		let mut input = Input::new_from_chars(" ---\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 47.1
	fn one_spaces_asterisks_success() {
		let mut input = Input::new_from_chars(" ***\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn one_spaces_underscores_success() {
		let mut input = Input::new_from_chars(" ___\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn two_spaces_dashes_success() {
		let mut input = Input::new_from_chars("  ---\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 47.2
	fn two_spaces_asterisks_success() {
		let mut input = Input::new_from_chars("  ***\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn two_spaces_underscores_success() {
		let mut input = Input::new_from_chars("  ___\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn three_spaces_dashes_success() {
		let mut input = Input::new_from_chars("   ---\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 47.3
	fn three_spaces_asterisks_success() {
		let mut input = Input::new_from_chars("   ***\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn three_spaces_underscores_success() {
		let mut input = Input::new_from_chars("   ___\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn too_many_spaces_dashes_fails() {
		let mut input = Input::new_from_chars("    ---\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 48
	fn too_many_spaces_asterisks_fails() {
		let mut input = Input::new_from_chars("    ***\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Not in spec
	fn too_many_spaces_underscores_fails() {
		let mut input = Input::new_from_chars("    ___\n".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 50
	fn many_dashes_success() {
		let mut input = Input::new_from_chars("----------\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn many_asterisks_success() {
		let mut input = Input::new_from_chars("**********\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Not in spec
	fn many_underscores_success() {
		let mut input = Input::new_from_chars("__________\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 51
	fn spaces_tabs_between_dashes_success() {
		let mut input = Input::new_from_chars(" - - -\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 52
	fn spaces_tabs_between_asterisks_success() {
		let mut input = Input::new_from_chars(" **  * ** * ** * **\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 53
	fn spaces_tabs_between_underscores_success() {
		let mut input = Input::new_from_chars("-     -      -      -\n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 54
	fn trailing_spaces_and_tabs_succeeds() {
		let mut input = Input::new_from_chars("- - - -    \n".chars(), None);
		let block = parse_thematic_break()(&mut input).unwrap();
		assert_eq!(block, LeafBlock::ThematicBreak);
	}

	#[test] // Spec example 55.1
	fn extra_characters_end_fails() {
		let mut input = Input::new_from_chars("_ _ _ _ a".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 55.2
	fn extra_characters_start_fails() {
		let mut input = Input::new_from_chars("a------".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 55.3
	fn extra_characters_middle_fails() {
		let mut input = Input::new_from_chars("---a---".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}

	#[test] // Spec example 56
	fn mixed_characters_fails() {
		let mut input = Input::new_from_chars(" *-*".chars(), None);
		let outcome = parse_thematic_break()(&mut input);
		assert!(outcome.is_err());
	}
}
