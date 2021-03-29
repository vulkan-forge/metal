//! Unsafe interface to descriptor sets.

use std::{
	marker::PhantomData
};
use ash::vk;
use crate::{
	Resource,
	resource
};
use super::{
	pool,
	Pool,
	Descriptor
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

/// Undefined descriptor set.
/// 
/// ## Safety
/// 
/// The `Layout` type must correspond to the type of the descriptor set's layout.
/// The `Pool` type must correspond to the true type referencing the pool which the
/// descriptor set has been allocated from.
/// Each descriptor set instance must hold a reference to the pool it has been allocated from
/// using this type. Such reference is provided by the `Pool::reference` function.
pub unsafe trait Set: Sized + Resource<Handle=RawHandle> {
	/// Descriptor set layout.
	type Layout: Layout;

	/// Pool.
	type Pool: pool::Reference;

	/// Write the given descriptor to this descriptor set, at the given offset.
	fn write<D: Descriptor, V>(&mut self, offset: u32, value: V) where V: super::Writer<Self, D>, Self::Layout: layout::HasDescriptor<D> {
		value.write_to(self, offset)
	}
}

/// Descriptor set setter.
/// 
/// Allows one do write an entire set at once to define it.
/// 
/// ## Safety
/// 
/// The resources attached to the descriptor set using
/// `into_descriptor_set` must outlive the returned descriptor set.
pub unsafe trait Setter<S: Set> {
	/// Converts an untyped descriptor set into
	/// a typed descriptor sets.
	/// 
	/// ## Safety
	/// 
	/// The resources attached to the descriptor set must
	/// outlive the descriptor set.
	unsafe fn into_descriptor_set<'p, P: Pool<Reference<'p> = S::Pool>>(
		self,
		pool: &'p P,
		handle: RawHandle
	) -> S;
}

/// Descriptor set writer.
pub unsafe trait Writer<S: Set> {
	/// Write this value to the descriptor set.
	/// 
	/// ## Safety
	/// 
	/// Any resource used by the descriptor set after this operation
	/// must outlive the descriptor set.
	fn write(self, set: &mut S);
}

/// Multiple descriptor sets.
/// 
/// ## Safety
/// 
/// The `Layout` type must be a correct representation of
/// the layout of each set in the correct order.
/// 
/// The `Pool` type must correspond to the type of pool reference
/// from which every set has allocated from.
pub unsafe trait Sets<'s> {
	/// Descriptor set layouts.
	type Layouts: Layouts;

	/// Pool.
	type Pool: pool::Reference;

	/// Resources iterator.
	type Resources<'a>: Iterator<Item=resource::Ref<'s>>;

	/// Returns an iterator over the resources captured by the descriptor sets.
	/// This includes the descriptor sets themselves.
	/// 
	/// These resources must not be released before the sets.
	fn resources<'a>(&'a self) -> Self::Resources<'a>;
}

/// Descriptor sets that are send.
pub unsafe trait SendSets<'s>: Sets<'s> + Send {
	/// Send resources iterator.
	type SendResources<'a>: Iterator<Item=resource::SendRef<'s>>;

	/// Returns an iterator over the resources captured by the descriptor sets.
	/// This includes the descriptor sets themselves.
	/// 
	/// These resources must not be released before the sets.
	fn resources<'a>(&'a self) -> Self::SendResources<'a>;
}

/// No layouts.
unsafe impl<'s> Sets<'s> for () {
	type Layouts = ();

	type Pool = ();

	type Resources<'a> = std::iter::Empty<resource::Ref<'s>>;

	fn resources<'a>(&'a self) -> Self::Resources<'a> {
		std::iter::empty()
	}
}

/// No layouts.
unsafe impl<'s> SendSets<'s> for () {
	type SendResources<'a> = std::iter::Empty<resource::SendRef<'s>>;

	fn resources<'a>(&'a self) -> Self::SendResources<'a> {
		std::iter::empty()
	}
}

/// Descriptor sets setters.
/// 
/// Allows one do write entire sets at once to define it.
/// 
/// ## Safety
/// 
/// The resources attached to the descriptor sets using
/// `into_descriptor_sets` must outlive the returned descriptor sets.
pub unsafe trait Setters<'s, S: Sets<'s>> {
	/// Converts an untyped list of descriptor sets into
	/// a typed array/tuple of descriptor sets.
	/// 
	/// ## Safety
	/// 
	/// The resources attached to the descriptor set must
	/// outlive the descriptor set.
	unsafe fn into_descriptor_sets<'p, P: Pool<Reference<'p> = S::Pool>>(
		self,
		pool: &'p P,
		handle: Vec<RawHandle>
	) -> S;
}

/// Describes the transition between two descriptor sets.
/// 
/// This trait is used to call [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html)
/// with the appropriate parameters.
pub unsafe trait Transition<A, B> {
	fn first_set(&self) -> u32;

	fn descriptor_sets(&self) -> &[RawHandle];

	fn dynamic_offsets(&self) -> &[u32];

	fn apply(self, sets: A) -> B;
}

/// Descriptor set of a given layout.
pub struct Raw<P: pool::Reference, L: Layout> {
	pool: P,
	handle: RawHandle,
	layout: PhantomData<L>
}

/// Raw descriptor set.
/// 
/// Note that `Raw` does not implement `Set` since it does not guarantee that
/// the resources bound to the descriptor set outlive the set.
impl<P: pool::Reference, L: Layout> Raw<P, L> {
	/// Creates a new raw descriptor set.
	/// 
	/// ## Safety
	/// 
	/// The given descriptor set `handle` must have been allocated
	/// from the descriptor pool referenced by the `pool` reference.
	pub unsafe fn new(pool: P, handle: RawHandle) -> Self {
		Self {
			pool,
			handle,
			layout: PhantomData
		}
	}
}

unsafe impl<P: pool::Reference, L: Layout> Resource for Raw<P, L> {
	type Handle = RawHandle;

	fn handle(&self) -> Self::Handle {
		self.handle
	}
}

impl<P: pool::Reference, L: Layout> Drop for Raw<P, L> {
	fn drop(&mut self) {
		unsafe {
			self.pool.free(self.handle)
		}
	}
}