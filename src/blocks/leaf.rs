#[derive(Debug, PartialEq)]
pub struct ParagraphLine {
	content: String,
	hard_break: bool,
}

#[derive(Debug, PartialEq)]
pub enum ParagraphEnd {
	EndOfInput,
	EmptyLine,
	ThematicBreak,
	ATXHeadingOpening(u8),
}

impl ParagraphLine {
	pub fn new(content: String, hard_break: bool) -> Self {
		ParagraphLine {
			content,
			hard_break,
		}
	}

	pub fn len(&self) -> usize {
		if self.hard_break {
			self.content.len() + 6
		} else {
			self.content.len()
		}
	}

	pub fn content(self) -> String {
		if self.hard_break {
			format!("{}<br />", self.content)
		} else {
			self.content
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Leaf {
	ThematicBreak,
	ATXHeading(u8, String),
	Paragraph(Vec<ParagraphLine>, ParagraphEnd),
}
