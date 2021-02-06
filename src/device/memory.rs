use std::sync::Arc;
use ash::vk;
use crate::{
	instance::physical_device::MemoryType,
	Device
};

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
}
