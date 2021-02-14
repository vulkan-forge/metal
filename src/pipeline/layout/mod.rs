use ash::{
	vk,
	version::DeviceV1_0
};
use std::sync::Arc;
use crate::{
	OomError,
	Device
};

pub mod set;
pub mod push_constant;

pub use set::Set;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError),
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub struct Layout {
	device: Arc<Device>,
	handle: vk::PipelineLayout
}

impl Layout {
	pub fn new(device: &Arc<Device>, set_layouts: &[Set], push_constant_ranges: &[push_constant::Range]) -> Result<Layout, CreationError> {
		let vk_set_layouts: Vec<_> = set_layouts.iter().map(|l| l.handle()).collect();
		
		let infos = vk::PipelineLayoutCreateInfo {
			flags: vk::PipelineLayoutCreateFlags::empty(),
			set_layout_count: vk_set_layouts.len() as u32,
			p_set_layouts: vk_set_layouts.as_ptr(),
			push_constant_range_count: push_constant_ranges.len() as u32,
			p_push_constant_ranges: push_constant_ranges.as_ptr() as *const _,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_pipeline_layout(&infos, None)?
		};

		Ok(Layout {
			device: device.clone(),
			handle
		})
	}

	pub(crate) fn handle(&self) -> vk::PipelineLayout {
		self.handle
	}
}

impl Drop for Layout {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline_layout(self.handle, None)
		}
	}
}