use crate::blocks::leaf::Leaf;
use crate::blocks::{Block, Container};
fn print_block(block: Block) -> String {
	match block {
		Block::LeafBlock(leaf) => print_leaf(leaf),
		Block::ContainerBlock(container) => print_container(container),
	}
}

fn print_leaf(leaf: Leaf) -> String {
	match leaf {
		Leaf::ThematicBreak => String::from("<hr />"),
		Leaf::ATXHeading(level, text) => format!("<h{}>{}</h{}>", level, text, level),
		Leaf::Paragraph(lines) => {
			let lines: Vec<_> = lines.into_iter().map(|l| l.content()).collect();
			let content = lines.join("\n");
			format!("<p>{content}</p>")
		}
	}
}

fn print_container(_: Container) -> String {
	todo!()
}

pub fn print<B>(blocks: B) -> String
where
	B: IntoIterator<Item = Block>,
{
	let blocks_html: Vec<String> = blocks.into_iter().map(print_block).collect();
	let blocks_html_length: usize = blocks_html.iter().map(|h| h.len()).sum();
	let line_break_count = blocks_html.len();
	let mut output = String::with_capacity(blocks_html_length + line_break_count);
	for block_html in blocks_html {
		output.push_str(&block_html);
		output.push('\n');
	}
	output
}
