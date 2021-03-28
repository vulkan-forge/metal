use std::sync::Arc;
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	Device,
	OomError
};
use super::{
	set,
	Set,
	Type
};

/// Descriptor pool.
pub trait Pool: Sized {
	type DescriptorSet<'a, L: set::Layout>: Set<Layout=L>;

	unsafe fn wrap<'a, L: set::Layout>(&'a self, handle: set::RawHandle) -> Self::DescriptorSet<'a, L>;

	fn allocate<'a, L: set::Layouts>(&'a self, layouts: L) -> Result<L::Allocated<'a, Self>, AllocationError>;
}

/// Descriptor set deallocator.
pub trait Deallocator {
	/// Deallocate the given descriptor set.
	/// 
	/// Note that the pool may choose not to actually free the descriptor set
	/// (for instance if it has not been created with the `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT` flag).
	/// 
	/// ## Safety
	/// 
	/// The descriptor set must not be owned or borrowed.
	unsafe fn free(&self, handle: set::RawHandle);
}

pub enum CreationError {
	OutOfMemory(OomError),
	Fragmentation
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> Self {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfMemory(OomError::Device),
			vk::Result::ERROR_FRAGMENTATION_EXT => Self::Fragmentation,
			_ => panic!("unknown error")
		}
	}
}

pub enum AllocationError {
	OutOfMemory(OomError),
	FragmentedPool,
	OutOfPoolMemory
}

impl From<vk::Result> for AllocationError {
	fn from(r: vk::Result) -> Self {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfMemory(OomError::Device),
			vk::Result::ERROR_FRAGMENTED_POOL => Self::FragmentedPool,
			vk::Result::ERROR_OUT_OF_POOL_MEMORY => Self::OutOfPoolMemory,
			_ => panic!("unknown error")
		}
	}
}

#[repr(transparent)]
pub struct Size(vk::DescriptorPoolSize);

impl Size {
	pub fn new(ty: Type, descriptor_count: u32) -> Self {
		Self(vk::DescriptorPoolSize {
			ty: ty.into_vulkan(),
			descriptor_count
		})
	}
}

/// Raw descriptor pool.
pub struct Raw {
	device: Arc<Device>,
	handle: vk::DescriptorPool,

	/// Should we free the descriptor sets when dropped? 
	free: bool
}

impl Raw {
	pub fn new(
		device: &Arc<Device>,
		sizes: &[Size],
		max_sets: u32,
		free: bool
	) -> Result<Self, CreationError> {
		let mut flags = vk::DescriptorPoolCreateFlags::empty();

		if free {
			flags |= vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET;
		}

		let infos = vk::DescriptorPoolCreateInfo {
			flags,
			pool_size_count: sizes.len() as u32,
			p_pool_sizes: sizes.as_ptr() as *const _,
			max_sets,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_descriptor_pool(&infos, None)?
		};

		Ok(Self {
			device: device.clone(),
			handle,
			free
		})
	}
}

impl Pool for Raw {
	type DescriptorSet<'a, L: set::Layout> = set::Raw<&'a Self, L>;

	unsafe fn wrap<'a, L: set::Layout>(&'a self, handle: set::RawHandle) -> Self::DescriptorSet<'a, L> {
		set::Raw::new(self, handle)
	}

	fn allocate<'a, L: set::Layouts>(&'a self, layouts: L) -> Result<L::Allocated<'a, Self>, AllocationError> {
		let layout_handles = layouts.handles();
		let layout_handles = layout_handles.as_ref();
		
		let infos = vk::DescriptorSetAllocateInfo {
			descriptor_pool: self.handle,
			descriptor_set_count: layout_handles.len() as u32,
			p_set_layouts: layout_handles.as_ptr(),
			..Default::default()
		};

		let handles = unsafe {
			self.device.handle.allocate_descriptor_sets(&infos)?
		};

		Ok(unsafe {
			L::allocated_from_raw(self, handles)
		})
	}
}

impl<'a> Deallocator for &'a Raw {
	unsafe fn free(&self, handle: set::RawHandle) {
		if self.free {
			self.device.handle.free_descriptor_sets(self.handle, &[handle])
		}
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.destroy_descriptor_pool(self.handle, None)
		}
	}
}