use ash::{
	vk,
	version::DeviceV1_0
};
use std::sync::Arc;
use crate::{
	OomError,
	Device
};

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

pub struct Set {
	device: Arc<Device>,
	handle: vk::DescriptorSetLayout
}

impl Set {
	pub fn new(device: &Arc<Device>, bindings: &[Binding]) -> Result<Set, CreationError> {
		let infos = vk::DescriptorSetLayoutCreateInfo {
			flags: vk::DescriptorSetLayoutCreateFlags::empty(), // TODO
			binding_count: bindings.len() as u32,
			p_bindings: bindings.as_ptr() as *const _,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_descriptor_set_layout(&infos, None)?
		};

		Ok(Set {
			device: device.clone(),
			handle
		})
	}

	pub(crate) fn handle(&self) -> vk::DescriptorSetLayout {
		self.handle
	}
}

impl Drop for Set {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_descriptor_set_layout(self.handle, None);
		}
	}
}

#[repr(transparent)]
pub struct Binding(vk::DescriptorSetLayoutBinding);