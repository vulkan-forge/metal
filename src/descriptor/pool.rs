use std::{
	sync::Arc,
	rc::Rc
};
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	Device,
	DeviceOwned,
	OomError
};
use super::{
	set,
	Sets,
	Type,
};

/// Descriptor pool.
/// 
/// ## Safety
/// 
/// The `Reference` type must represent a sound reference to the original pool.
/// Such reference must not outlive the pool.
/// The `allocate` function must return valid descriptor sets
/// matching the layouts given as parameter and initialized with the given input values.
/// Each returned descriptor set must own a reference to the descriptor pool
/// it has been allocated from.
pub unsafe trait Pool: Sized {
	/// Pool reference.
	type Reference<'a>: Reference;

	/// Get a reference to this descriptor pool.
	fn reference<'a>(&'a self) -> Self::Reference<'a>;

	/// Allocates new descriptor sets.
	fn allocate<'a, 's, V, S: Sets<'s, Pool=Self::Reference<'a>> + set::InitAllFrom<'s, V>>(&'a self, layouts: &Arc<S::Layouts>, values: V) -> Result<S, AllocationError>;
}

/// Descriptor pool reference.
pub trait Reference: Clone {
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

/// No pool reference.
/// 
/// This is usefull for the only sets that do not require allocation: the empty sets.
impl Reference for () {
	unsafe fn free(&self, _handle: set::RawHandle) {
		panic!("trying to free an empty descriptor set")
	}
}

#[derive(Debug)]
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

	/// ## Safety
	/// 
	/// `pool_reference` must be a reference to `self`.
	unsafe fn allocate_with<'a, 's, V, S: Sets<'s> + set::InitAllFrom<'s, V>>(&'a self, pool_reference: S::Pool, layouts: &Arc<S::Layouts>, values: V) -> Result<S, AllocationError> {
		use set::Layouts;
		let layout_handles = layouts.handles();
		let layout_handles = layout_handles.as_ref();
		
		let infos = vk::DescriptorSetAllocateInfo {
			descriptor_pool: self.handle,
			descriptor_set_count: layout_handles.len() as u32,
			p_set_layouts: layout_handles.as_ptr(),
			..Default::default()
		};

		let handles = self.device.handle.allocate_descriptor_sets(&infos)?;

		Ok(S::init_from(layouts, values, pool_reference, handles))
	}
}

impl DeviceOwned for Raw {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

unsafe impl Pool for Raw {
	type Reference<'a> = &'a Self;

	fn reference(&self) -> &Self {
		self
	}

	fn allocate<'a, 's, V, S: Sets<'s, Pool=Self::Reference<'a>> + set::InitAllFrom<'s, V>>(&'a self, layouts: &Arc<S::Layouts>, values: V) -> Result<S, AllocationError> {
		unsafe {
			self.allocate_with(self, layouts, values)
		}
	}
}

impl<'a> Reference for &'a Raw {
	unsafe fn free(&self, handle: set::RawHandle) {
		if self.free {
			self.device.handle.free_descriptor_sets(self.handle, &[handle])
		}
	}
}

unsafe impl Pool for Rc<Raw> {
	type Reference<'a> = Rc<Raw>;

	fn reference(&self) -> Rc<Raw> {
		self.clone()
	}

	fn allocate<'a, 's, V, S: Sets<'s, Pool=Self::Reference<'a>> + set::InitAllFrom<'s, V>>(&'a self, layouts: &Arc<S::Layouts>, values: V) -> Result<S, AllocationError> {
		unsafe {
			self.as_ref().allocate_with(self.clone(), layouts, values)
		}
	}
}

impl<'a> Reference for Arc<Raw> {
	unsafe fn free(&self, handle: set::RawHandle) {
		self.as_ref().free(handle)
	}
}

unsafe impl Pool for Arc<Raw> {
	type Reference<'a> = Arc<Raw>;

	fn reference(&self) -> Arc<Raw> {
		self.clone()
	}

	fn allocate<'a, 's, V, S: Sets<'s, Pool=Self::Reference<'a>> + set::InitAllFrom<'s, V>>(&'a self, layouts: &Arc<S::Layouts>, values: V) -> Result<S, AllocationError> {
		unsafe {
			self.as_ref().allocate_with(self.clone(), layouts, values)
		}
	}
}

impl<'a> Reference for Rc<Raw> {
	unsafe fn free(&self, handle: set::RawHandle) {
		self.as_ref().free(handle)
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.destroy_descriptor_pool(self.handle, None)
		}
	}
}