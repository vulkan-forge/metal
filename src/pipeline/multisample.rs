use ash::vk;
use std::convert::TryInto;
use crate::{
	framebuffer::SampleCount
};

pub struct Multisample(vk::PipelineMultisampleStateCreateInfo);

impl Multisample {
	pub fn new(
		rasterization_samples: SampleCount,
		min_sample_shading: Option<f32>,
		alpha_to_coverage: bool,
		alpha_to_one: bool
	) -> Multisample {
		Multisample(vk::PipelineMultisampleStateCreateInfo {
			rasterization_samples: rasterization_samples.into_vulkan(),
			sample_shading_enable: if min_sample_shading.is_some() { vk::TRUE } else { vk::FALSE },
			min_sample_shading: min_sample_shading.unwrap_or_default(),
			p_sample_mask: std::ptr::null(), // TODO
			alpha_to_coverage_enable: if alpha_to_coverage { vk::TRUE } else { vk::FALSE },
			alpha_to_one_enable: if alpha_to_one { vk::TRUE } else { vk::FALSE },
			..Default::default()
		})
	}

	pub(crate) fn as_vulkan(&self) -> &vk::PipelineMultisampleStateCreateInfo {
		&self.0
	}
}

impl Default for Multisample {
	fn default() -> Self {
		Self::new(1.try_into().unwrap(), None, false, false)
	}
}