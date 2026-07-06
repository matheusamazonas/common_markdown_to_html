#[derive(Debug, PartialEq)]
pub enum Leaf {
	ThematicBreak,
	ATXHeading(u8, String),
}
