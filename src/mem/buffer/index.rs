use ash::vk;
use super::{
	Buffer,
	TypedBuffer
};

pub unsafe trait IndexBuffer: Buffer {
	fn index_type(&self) -> vk::IndexType;
}

// unsafe impl<B: std::ops::Deref> IndexBuffer for B where B::Target: IndexBuffer {
// 	fn index_type(&self) -> vk::IndexType {
// 		self.deref().index_type()
// 	}
// }

/// Buffer item type that can be used as index in an index buffer.
pub unsafe trait Index {
	const VULKAN: vk::IndexType;
}

unsafe impl Index for u8 {
	const VULKAN: vk::IndexType = vk::IndexType::UINT8_EXT;
}

unsafe impl Index for u16 {
	const VULKAN: vk::IndexType = vk::IndexType::UINT16;
}

unsafe impl Index for u32 {
	const VULKAN: vk::IndexType = vk::IndexType::UINT32;
}

unsafe impl<B: TypedBuffer> IndexBuffer for B where B::Item: Index {
	fn index_type(&self) -> vk::IndexType {
		B::Item::VULKAN
	}
}