use std::{
	marker::PhantomData
};
use ash::vk;
use crate::Resource;
use super::{
	pool
};

pub mod layout;

pub use layout::{
	Layout,
	Layouts
};

/// Raw vulkan descriptor set layout handle.
pub type LayoutRawHandle = vk::DescriptorSetLayout;

/// Raw vulkan descriptor set handle.
pub type RawHandle = vk::DescriptorSet;

pub unsafe trait Set: Resource<Handle=RawHandle> {
	/// Descriptor set layout.
	type Layout: Layout;
}

/// Descriptor set of a given layout.
pub struct Raw<P: pool::Deallocator, L: Layout> {
	pool: P,
	handle: RawHandle,
	layout: PhantomData<L>
}

/// Raw descriptor layout.
impl<P: pool::Deallocator, L: Layout> Raw<P, L> {
	pub(crate) fn new(pool: P, handle: RawHandle) -> Self {
		Self {
			pool,
			handle,
			layout: PhantomData
		}
	}
}

unsafe impl<P: pool::Deallocator, L: Layout> Resource for Raw<P, L> {
	type Handle = RawHandle;

	fn handle(&self) -> Self::Handle {
		self.handle
	}
}

unsafe impl<P: pool::Deallocator, L: Layout> Set for Raw<P, L> {
	type Layout = L;
}

impl<P: pool::Deallocator, L: Layout> Drop for Raw<P, L> {
	fn drop(&mut self) {
		unsafe {
			self.pool.free(self.handle)
		}
	}
}