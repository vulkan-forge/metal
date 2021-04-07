use ash::vk;
use crate::pipeline::input_assembly::Topology;
use super::{
	Read,
	TypedRead,
	Write,
	super::Index
};

pub type IndexType = vk::IndexType;

pub unsafe trait IndexRead<T: Topology>: Read {
	fn index_type(&self) -> IndexType;

	fn index_per_item(&self) -> u32;
}

pub unsafe trait IndexWrite<T: Topology>: IndexRead<T> + Write {
	// ...
}

unsafe impl<T: Topology, B: TypedRead> IndexRead<T> for B where B::Item: Index<T> {
	fn index_type(&self) -> IndexType {
		B::Item::TYPE
	}

	fn index_per_item(&self) -> u32 {
		B::Item::COUNT
	}
}