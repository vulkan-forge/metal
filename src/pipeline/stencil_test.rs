use ash::vk;
use crate::ops;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Operation {
	Keep = vk::StencilOp::KEEP.as_raw(),
	Zero = vk::StencilOp::ZERO.as_raw(),
	Replace = vk::StencilOp::REPLACE.as_raw(),
	IncrementAndClamp = vk::StencilOp::INCREMENT_AND_CLAMP.as_raw(),
	DecrementAndClamp = vk::StencilOp::DECREMENT_AND_CLAMP.as_raw(),
	Invert = vk::StencilOp::INVERT.as_raw(),
	IncrementAndWrap = vk::StencilOp::INCREMENT_AND_WRAP.as_raw(),
	DecrementAndWrap = vk::StencilOp::DECREMENT_AND_WRAP.as_raw()
}

impl Operation {
	pub(crate) fn into_vulkan(self) -> vk::StencilOp {
		vk::StencilOp::from_raw(self as i32)
	}
}

impl Default for Operation {
	#[inline]
	fn default() -> Self {
		unsafe {
			std::mem::transmute(vk::StencilOp::default().as_raw())
		}
	}
}

#[derive(Clone, Copy, Default, Debug)]
pub struct StencilTest {
	pub fail_operation: Operation,
	pub pass_operation: Operation,
	pub depth_fail_operation: Operation,
	pub compare_operation: ops::Compare,
	pub compare_mask: u32,
	pub write_mask: u32,
	pub reference: u32
}

impl StencilTest {
	pub fn new(
		fail_operation: Operation,
		pass_operation: Operation,
		depth_fail_operation: Operation,
		compare_operation: ops::Compare,
		compare_mask: u32,
		write_mask: u32,
		reference: u32
	) -> Self {
		StencilTest {
			fail_operation,
			pass_operation,
			depth_fail_operation,
			compare_operation,
			compare_mask,
			write_mask,
			reference
		}
	}

	pub(crate) fn set_vulkan(&self, infos: &mut vk::StencilOpState) {
		infos.fail_op = self.fail_operation.into_vulkan();
		infos.pass_op = self.pass_operation.into_vulkan();
		infos.depth_fail_op = self.depth_fail_operation.into_vulkan();
		infos.compare_op = self.compare_operation.into_vulkan();
		infos.compare_mask = self.compare_mask;
		infos.write_mask = self.write_mask;
		infos.reference = self.reference;
	}
}