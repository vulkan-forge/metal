use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	ops::Deref
};
use crate::{
	Device,
	DeviceOwned,
	OomError,
	sync::SharingMode,
	alloc::{
		self,
		Allocator,
		Slot
	}
};

mod memory_requirements;

pub use memory_requirements::MemoryRequirements;

pub trait Buffer {
	fn handle(&self) -> vk::Buffer;
}

#[derive(Clone, Copy)]
pub struct Usage(vk::BufferUsageFlags);

impl Usage {
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn into_vulkan_flags(self) -> vk::BufferUsageFlags {
		self.0
	}

	pub fn transfer_source(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::TRANSFER_SRC)
	}

	pub fn transfer_destination(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::TRANSFER_DST)
	}

	pub fn uniform_texel_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER)
	}

	pub fn storage_texel_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER)
	}

	pub fn uniform_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::UNIFORM_BUFFER)
	}

	pub fn storage_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::STORAGE_BUFFER)
	}

	pub fn index_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::INDEX_BUFFER)
	}

	pub fn vertex_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::VERTEX_BUFFER)
	}

	pub fn indirect_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::INDIRECT_BUFFER)
	}
}

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError),
	InvalidOpaqueCaptureAddress
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR => CreationError::InvalidOpaqueCaptureAddress,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub enum BindError {
	OutOfMemory(OomError),
	InvalidOpaqueCaptureAddress
}

impl From<vk::Result> for BindError {
	fn from(r: vk::Result) -> BindError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => BindError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => BindError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR => BindError::InvalidOpaqueCaptureAddress,
			_ => unreachable!()
		}
	}
}

pub struct UnboundBuffer {
	handle: vk::Buffer,
	device: Arc<Device>,
	size: u64,
	usage: Usage
}

impl UnboundBuffer {
	/// Create a raw, uninitialized buffer of the given size.
	pub fn new(device: &Arc<Device>, size: u64, usage: Usage, sharing_mode: SharingMode) -> Result<Self, CreationError> {
		assert!(!usage.is_empty());

		let infos = match sharing_mode {
			SharingMode::Exclusive => {
				vk::BufferCreateInfo {
					size,
					usage: usage.into_vulkan_flags(),
					sharing_mode: vk::SharingMode::EXCLUSIVE,
					..Default::default()
				}
			},
			SharingMode::Concurrent(queues) => {
				vk::BufferCreateInfo {
					size,
					usage: usage.into_vulkan_flags(),
					sharing_mode: vk::SharingMode::CONCURRENT,
					queue_family_index_count: queues.len() as u32,
					p_queue_family_indices: queues.as_ptr(),
					..Default::default()
				}
			}
		};

		let handle = unsafe {
			device.handle.create_buffer(&infos, None)?
		};

		Ok(UnboundBuffer {
			handle,
			device: device.clone(),
			size,
			usage
		})
	}

	#[inline]
	pub fn size(&self) -> u64 {
		self.size
	}

	#[inline]
	pub fn memory_requirements(&self) -> MemoryRequirements {
		unsafe {
			let mr = self.device.handle.get_buffer_memory_requirements(self.handle);
			MemoryRequirements(mr)
		}
	}

	#[inline]
	pub unsafe fn bind<A: Allocator>(self, slot: A::Slot) -> Result<alloc::Buffer<A>, (Self, BindError)> {
		let memory = slot.memory();
		
		// We check for correctness in debug mode.
		debug_assert!({
			let mem_reqs = self.memory_requirements();
			mem_reqs.size() <= (memory.size() - slot.offset()) as u64
				&& (slot.offset() as u64 % mem_reqs.alignment()) == 0
				&& mem_reqs.contains_memory_type_index(memory.memory_type().index())
		});
		
		// Check for alignment correctness.
		{
			let limits = self.device.physical_device().limits();
			if self.usage.uniform_texel_buffer() || self.usage.storage_texel_buffer() {
				debug_assert!(slot.offset() % limits.min_texel_buffer_offset_alignment() == 0);
			}

			if self.usage.storage_buffer() {
				debug_assert!(slot.offset() % limits.min_storage_buffer_offset_alignment() == 0);
			}

			if self.usage.uniform_buffer() {
				debug_assert!(slot.offset() % limits.min_uniform_buffer_offset_alignment() == 0);
			}
		}

		match self.device.handle.bind_buffer_memory(self.handle, memory.handle(), slot.offset()) {
			Ok(()) => (),
			Err(e) => return Err((self, e.into()))
		}

		Ok(alloc::Buffer::new(self, slot))
	}
}

impl DeviceOwned for UnboundBuffer {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Drop for UnboundBuffer {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.destroy_buffer(self.handle, None);
		}
	}
}