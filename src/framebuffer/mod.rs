use ash::vk;
use std::convert::TryFrom;
pub mod render_pass;
pub use render_pass::{
	RenderPass,
	RenderPassBuilder
};

#[derive(Debug, Clone, Copy)]
pub struct SampleCount(vk::SampleCountFlags);

#[derive(Debug, Clone, Copy)]
pub struct InvalidSampleCount;

impl TryFrom<u8> for SampleCount {
	type Error = InvalidSampleCount;
	
	#[inline]
	fn try_from(c: u8) -> Result<SampleCount, InvalidSampleCount> {
		let f = match c {
			1 => vk::SampleCountFlags::TYPE_1,
			2 => vk::SampleCountFlags::TYPE_2,
			4 => vk::SampleCountFlags::TYPE_4,
			8 => vk::SampleCountFlags::TYPE_8,
			16 => vk::SampleCountFlags::TYPE_16,
			32 => vk::SampleCountFlags::TYPE_32,
			64 => vk::SampleCountFlags::TYPE_64,
			_ => return Err(InvalidSampleCount)
		};

		Ok(SampleCount(f))
	}
}

impl SampleCount {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::SampleCountFlags {
		self.0
	}
}