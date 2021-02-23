use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	rc::Rc
};
use crate::{
	OomError,
	instance::physical_device,
	Device,
	DeviceOwned
};
use super::Buffer;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError)
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

#[derive(Debug)]
pub enum AllocError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for AllocError {
	fn from(r: vk::Result) -> AllocError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => AllocError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => AllocError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub struct Pool {
	device: Arc<Device>,
	handle: vk::CommandPool
}

impl Pool {
	pub fn new(device: &Arc<Device>, queue_family: physical_device::QueueFamily) -> Result<Pool, CreationError> {
		assert_eq!(device.physical_device().index(), queue_family.physical_device().index());
		
		let infos = vk::CommandPoolCreateInfo {
			queue_family_index: queue_family.index(),
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_command_pool(&infos, None)?
		};

		Ok(Pool {
			device: device.clone(),
			handle
		})
	}

	pub(crate) fn handle(&self) -> vk::CommandPool {
		self.handle
	}

	pub fn allocate<'a>(self: &Rc<Self>, count: u32) -> Result<Vec<Buffer<'a>>, AllocError> {
		let infos = vk::CommandBufferAllocateInfo {
			command_pool: self.handle,
			level: vk::CommandBufferLevel::PRIMARY,
			command_buffer_count: count,
			..Default::default()
		};

		let handles = unsafe {
			self.device.handle().allocate_command_buffers(&infos)?
		};

		Ok(handles.into_iter().map(|h| Buffer::new(self, h)).collect())
	}
}

impl DeviceOwned for Pool {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Drop for Pool {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_command_pool(self.handle, None)
		}
	}
}