use ash::vk;
use crate::ops;

#[derive(Clone, Copy, Default, Debug)]
pub struct DepthTest {
	pub write: bool,
	pub operation: ops::Compare,

	/// Minimum and maximum depth bounds used in the depth bounds test.
	/// 
	/// Depth bounds testing is disabled if `None`.
	pub bounds: Option<(f32, f32)>
}

impl DepthTest {
	pub fn new(
		write: bool,
		operation: ops::Compare,
		bounds: Option<(f32, f32)>
	) -> Self {
		DepthTest {
			write,
			operation,
			bounds
		}
	}

	pub fn set_vulkan(&self, infos: &mut vk::PipelineDepthStencilStateCreateInfo) {
		infos.depth_test_enable = vk::TRUE;
		infos.depth_write_enable = if self.write { vk::TRUE } else { vk::FALSE };
		infos.depth_compare_op = self.operation.into_vulkan();

		match self.bounds {
			Some((min, max)) => {
				infos.depth_bounds_test_enable = vk::TRUE;
				infos.min_depth_bounds = min;
				infos.max_depth_bounds = max;
			},
			None => {
				infos.depth_bounds_test_enable = vk::FALSE;
			}
		}
	}
}