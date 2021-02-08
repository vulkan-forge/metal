use std::{
	sync::Arc,
	ffi::c_void
};
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	OomError,
	instance::physical_device::MemoryType,
	Device,
	DeviceOwned
};

#[derive(Debug)]
pub enum MapError {
	OutOfMemory(OomError),
	MemoryMapFailed
}

impl From<vk::Result> for MapError {
	fn from(r: vk::Result) -> MapError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => MapError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => MapError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_MEMORY_MAP_FAILED => MapError::MemoryMapFailed,
			_ => unreachable!()
		}
	}
}

/// A region of allocated device memory.
pub struct Memory {
	/// Underlying vulkan handle.
	handle: vk::DeviceMemory,

	/// Device owning the memory.
	device: Arc<Device>,
	
	/// Memory type.
	memory_type_index: u32,
	
	/// Size (in bytes) of the memory region.
	size: u64
}

impl Memory {
	#[inline]
	pub(crate) fn new(device: &Arc<Device>, memory_type: MemoryType, size: u64, handle: vk::DeviceMemory) -> Memory {
		Memory {
			handle,
			device: device.clone(),
			memory_type_index: memory_type.index(),
			size
		}
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::DeviceMemory {
		self.handle
	}

	#[inline]
	pub fn size(&self) -> u64 {
		self.size
	}

	#[inline]
	pub fn memory_type(&self) -> MemoryType {
		MemoryType::new(self.device.physical_device(), self.memory_type_index)
	}

	/// Map the memory to host address space.
	/// 
	/// # Safety
	/// 
	/// The memory must be host visible and not already mapped.
	#[inline]
	pub unsafe fn map(&self, offset: u64, size: u64) -> Result<*mut c_void, MapError> {
		let ptr = self.device.handle.map_memory(
			self.handle,
			offset,
			size,
			vk::MemoryMapFlags::empty()
		)?;
		Ok(ptr)
	}

	/// Unmap the memory from host address space.
	/// 
	/// # Safety
	/// 
	/// The memory must be mapped.
	pub unsafe fn unmap(&self) {
		self.device.handle.unmap_memory(self.handle)
	}
}

impl DeviceOwned for Memory {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}