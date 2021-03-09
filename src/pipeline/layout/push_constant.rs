use ash::vk;
use crate::pipeline::shader;

pub unsafe trait PushConstants {
	const RANGES: &'static [Range];
}

pub unsafe trait Setter<C: PushConstants> {
	fn ranges(&self) -> &[(Range, *const u8)];
}

unsafe impl PushConstants for () {
	const RANGES: &'static [Range] = &[];
}

unsafe impl Setter<()> for () {
	fn ranges(&self) -> &[(Range, *const u8)] {
		&[]
	}
}

#[repr(transparent)]
pub struct Range(pub(crate) vk::PushConstantRange); // This MUST be homomorphic to `vk::PushConstantRange`.

impl Range {
	pub const fn new(stages: shader::Stages, offset: u32, size: u32) -> Range {
		Range(vk::PushConstantRange {
			stage_flags: stages.into_vulkan(),
			offset,
			size
		})
	}

	pub fn offset(&self) -> u32 {
		self.0.offset
	}

	pub fn size(&self) -> u32 {
		self.0.size
	}
}