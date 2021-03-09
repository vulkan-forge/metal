use ash::{
	vk,
	version::DeviceV1_0
};
use std::sync::Arc;
use crate::{
	Device,
	DeviceOwned,
	sync,
	OomError,
	mem::{
		MemoryRequirements,
		Slot
	}
};
use super::{
	Usages,
	Bound
};

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

pub struct Unbound {
	handle: vk::Buffer,
	device: Arc<Device>,
	size: u64,
	usage: Usages
}

impl Unbound {
	/// Create a raw, uninitialized buffer of the given size.
	pub fn new<'a, U: Into<Usages>, S: Into<sync::SharingQueues>>(device: &Arc<Device>, size: u64, usage: U, sharing_queues: S) -> Result<Self, CreationError> {
		let usage = usage.into();
		assert!(!usage.is_empty());

		let sharing_queues = sharing_queues.into();
		let (sh_mode, sh_count, sh_indices) = sharing_queues.as_vulkan();

		let infos = vk::BufferCreateInfo {
			size,
			usage: usage.into_vulkan(),
			sharing_mode: sh_mode,
			queue_family_index_count: sh_count,
			p_queue_family_indices: sh_indices,
			..Default::default()
		};

		let handle = unsafe {
			device.handle.create_buffer(&infos, None)?
		};

		Ok(Unbound {
			handle,
			device: device.clone(),
			size,
			usage
		})
	}

	pub fn handle(&self) -> vk::Buffer {
		self.handle
	}

	#[inline]
	pub fn len(&self) -> u64 {
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
	pub unsafe fn bind<S: Send + Slot>(self, slot: S) -> Result<Bound, (Self, BindError)> {
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

		Ok(Bound::new(self, slot))
	}
}

impl DeviceOwned for Unbound {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Drop for Unbound {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.destroy_buffer(self.handle, None);
		}
	}
}