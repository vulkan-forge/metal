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
	NotHostVisible,
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
	#[inline]
	pub fn map(self, offset: u64, size: Option<u64>) -> Result<MappedMemory, MapError> {
		if self.memory_type().is_host_visible() {
			let ptr = unsafe {
				self.device.handle.map_memory(
					self.handle,
					offset,
					size.unwrap_or(vk::WHOLE_SIZE),
					vk::MemoryMapFlags::empty()
				)?
			};

			Ok(MappedMemory {
				memory: self,
				ptr
			})
		} else {
			Err(MapError::NotHostVisible)
		}
	}
}

impl DeviceOwned for Memory {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Drop for Memory {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.free_memory(self.handle, None)
		}
	}
}

pub struct MappedMemory {
	memory: Memory,
	ptr: *mut c_void
}

impl MappedMemory {
	pub fn ptr(&self) -> *mut c_void {
		self.ptr
	}

	pub fn as_memory(&self) -> &Memory {
		&self.memory
	}

	fn into_raw_parts(&self) -> (Memory, *mut c_void) {
		let memory = unsafe { std::ptr::read(&self.memory) };
		let ptr = self.ptr;
		std::mem::forget(self);
		(memory, ptr)
	}

	/// Unmap the memory from host address space.
	pub fn unmap(self) -> Memory {
		let (memory, _) = self.into_raw_parts();
		unsafe { memory.device.handle.unmap_memory(memory.handle) };
		memory
	}
}

impl Drop for MappedMemory {
	fn drop(&mut self) {
		// unmap the memory.
		unsafe {
			self.memory.device.handle.unmap_memory(self.memory.handle)
		}
	}
}