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
use super::buffer;

pub mod raw;
pub mod sync;

pub use raw::Raw;
pub use sync::SyncPool;

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

pub trait Pool: DeviceOwned {
	type Buffer<'a> where Self: 'a;

	fn allocate<'a>(&'a self, count: u32) -> Result<Vec<Self::Buffer<'a>>, AllocError>;
}

pub trait Handle: DeviceOwned {
	fn handle(&self) -> vk::CommandPool;

	unsafe fn free(&self, buffer_handles: &[vk::CommandBuffer]);
}