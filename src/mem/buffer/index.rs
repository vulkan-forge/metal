use ash::vk;
use crate::pipeline::input_assembly::{
	topology,
	Topology
};
use super::{
	Buffer,
	TypedBuffer
};

pub type IndexType = vk::IndexType;

pub unsafe trait IndexBuffer<T: Topology>: Buffer {
	fn index_type(&self) -> IndexType;

	fn index_per_item(&self) -> u32;
}

/// Buffer item type that can be used as index in an index buffer.
pub unsafe trait Index<T: Topology> {
	/// Vulkan index type.
	const TYPE: IndexType;

	/// How many indexes are represented by this type.
	const COUNT: u32;
}

unsafe impl Index<topology::PointList> for u8 {
	const TYPE: IndexType = IndexType::UINT8_EXT;

	const COUNT: u32 = 1;
}

unsafe impl Index<topology::PointList> for u16 {
	const TYPE: IndexType = IndexType::UINT16;

	const COUNT: u32 = 1;
}

unsafe impl Index<topology::PointList> for u32 {
	const TYPE: IndexType = IndexType::UINT32;

	const COUNT: u32 = 1;
}

unsafe impl<T: Topology, B: TypedBuffer> IndexBuffer<T> for B where B::Item: Index<T> {
	fn index_type(&self) -> IndexType {
		B::Item::TYPE
	}

	fn index_per_item(&self) -> u32 {
		B::Item::COUNT
	}
}