use ash::vk;

pub struct Tesselation(vk::PipelineTessellationStateCreateInfo);

impl Tesselation {
	pub fn new(patch_control_points: u32) -> Tesselation {
		Tesselation(vk::PipelineTessellationStateCreateInfo {
			patch_control_points,
			..Default::default()
		})
	}

	pub(crate) fn as_vulkan(&self) -> &vk::PipelineTessellationStateCreateInfo {
		&self.0
	}
}