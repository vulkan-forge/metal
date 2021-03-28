use ash::vk;
use crate::Resource;
use super::super::{
	Pool,
	Descriptor
};

pub type RawHandle = vk::DescriptorSetLayout;

/// Descriptor set layout.
pub trait Layout: Resource<Handle=RawHandle> {
	// ...
}

/// Property for a given layout having defining the given descriptor.
pub unsafe trait HasDescriptor<D: Descriptor> {
	/// Descriptor binding.
	const BINDING: u32;
}

/// List of desctriptor set layouts.
/// 
/// ## Safety
/// 
/// The `Allocated` type must be an array/tuple of `DescriptorSet` types
/// whose layout type `L` parameter matches the associated layout.
pub unsafe trait Layouts {
	/// Layout handles.
	type Handles<'a>: AsRef<[RawHandle]>;

	/// List of associated descriptor set (with the correct layout type).
	type Allocated<'p, P: Pool>;

	fn handles<'a>(&'a self) -> Self::Handles<'a>;

	/// Converts an untyped list of descriptor sets into
	/// a typed array/tuple of descriptor sets.
	unsafe fn allocated_from_raw<'p, P: Pool>(pool: &'p P, handles: Vec<super::RawHandle>) -> Self::Allocated<'p, P>;
}

unsafe impl<L: Layout> Layouts for L {
	type Handles<'h> = [RawHandle; 1];

	type Allocated<'p, P: Pool> = P::DescriptorSet<'p, L>;

	fn handles<'h>(&'h self) -> [RawHandle; 1] {
		[self.handle()]
	}

	unsafe fn allocated_from_raw<'p, P: Pool>(pool: &'p P, handles: Vec<super::RawHandle>) -> Self::Allocated<'p, P> {
		handles.into_iter().map(|handle| pool.wrap(handle)).next().unwrap()
	}
}

unsafe impl<'a, L: Layout> Layouts for &'a [L] {
	type Handles<'h> = Vec<RawHandle>;

	type Allocated<'p, P: Pool> = Vec<P::DescriptorSet<'p, L>>;

	fn handles<'h>(&'h self) -> Vec<RawHandle> {
		self.iter().map(|layout| layout.handle()).collect()
	}

	unsafe fn allocated_from_raw<'p, P: Pool>(pool: &'p P, handles: Vec<super::RawHandle>) -> Self::Allocated<'p, P> {
		handles.into_iter().map(|handle| pool.wrap(handle)).collect()
	}
}