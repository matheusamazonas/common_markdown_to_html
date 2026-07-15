use crate::blocks::{Container, Leaf};

#[derive(Debug, PartialEq)]
pub enum Block {
	LeafBlock(Leaf),
	ContainerBlock(Container),
}
