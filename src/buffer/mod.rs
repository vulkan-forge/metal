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
	device::Queue,
	sync,
	OomError,
	alloc::{
		self,
		Allocator,
		Slot
	}
};

mod memory_requirements;

pub use memory_requirements::MemoryRequirements;

pub trait Buffer: crate::Resource {
	fn handle(&self) -> vk::Buffer;

	/// Checks if the buffer is ready to be used.
	/// 
	/// ## Safety
	/// 
	/// Once the function has returned `true`,
	/// it should return `true` forever.
	fn ready(&self) -> bool;
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Usage {
	TransferSource = vk::BufferUsageFlags::TRANSFER_SRC.as_raw(),
	TransferDestination = vk::BufferUsageFlags::TRANSFER_DST.as_raw(),
	UniformTexelBuffer = vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER.as_raw(),
	StorageTexelBuffer = vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER.as_raw(),
	UniformBuffer = vk::BufferUsageFlags::UNIFORM_BUFFER.as_raw(),
	StorageBuffer = vk::BufferUsageFlags::STORAGE_BUFFER.as_raw(),
	IndexBuffer = vk::BufferUsageFlags::INDEX_BUFFER.as_raw(),
	VertexBuffer = vk::BufferUsageFlags::VERTEX_BUFFER.as_raw(),
	IndirectBuffer = vk::BufferUsageFlags::INDIRECT_BUFFER.as_raw()
}

impl Usage {
	pub(crate) fn into_vulkan(self) -> vk::BufferUsageFlags {
		vk::BufferUsageFlags::from_raw(self as u32)
	}
}

impl std::ops::BitOr for Usage {
	type Output = Usages;

	fn bitor(self, rhs: Self) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOr<Usages> for Usage {
	type Output = Usages;

	fn bitor(self, rhs: Usages) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

#[derive(Clone, Copy)]
pub struct Usages(vk::BufferUsageFlags);

impl Usages {
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn into_vulkan(self) -> vk::BufferUsageFlags {
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

impl From<Usage> for Usages {
	fn from(u: Usage) -> Usages {
		Usages(u.into_vulkan())
	}
}

impl std::ops::BitOr for Usages {
	type Output = Usages;

	fn bitor(self, rhs: Self) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOr<Usage> for Usages {
	type Output = Usages;

	fn bitor(self, rhs: Usage) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOrAssign for Usages {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.into_vulkan()
	}
}

impl std::ops::BitOrAssign<Usage> for Usages {
	fn bitor_assign(&mut self, rhs: Usage) {
		self.0 |= rhs.into_vulkan()
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