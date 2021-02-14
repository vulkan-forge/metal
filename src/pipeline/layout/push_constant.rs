use ash::vk;
use crate::pipeline::shader;

pub struct Range(vk::PushConstantRange); // This MUST be homomorphic to `vk::PushConstantRange`.

impl Range {
	pub fn new(stages: shader::Stages, offset: u32, size: u32) -> Range {
		Range(vk::PushConstantRange {
			stage_flags: stages.into_vulkan(),
			offset,
			size
		})
	}
}