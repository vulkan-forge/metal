use ash::vk;
use crate::{
	pipeline::input_assembly::{
		topology,
		Topology,
	}
};

pub type IndexType = vk::IndexType;

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