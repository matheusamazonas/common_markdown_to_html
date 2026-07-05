use crate::blocks::{Container, Leaf};

pub enum Block {
	LeafBlock(Leaf),
	ContainerBlock(Container),
}
