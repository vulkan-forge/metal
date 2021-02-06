use std::cmp::{
	PartialOrd,
	Ord,
	Ordering
};
use crate::{
	Device,
	device
};

pub enum Node {
	/// A free leaf of the given size
	Leaf(u32),

	/// An allocated leaf of the given size
	Allocated(u32),

	/// A split node.
	Split {
		/// Offset of the split relative to parent node (absolute of no parent).
		offset: u32
	}
}

pub struct Tree {
	nodes: Vec<Node>
}

impl Tree {
	pub fn new(size: u32) -> Tree {
		Tree {
			nodes: vec![Node::Free(size)]
		}
	}

	pub fn allocate(&mut self, offset: u32, size: u32) {
		// ...
	}

	pub fn free(&mut self, offset: u32, size: u32) {
		// ...
	}
}
