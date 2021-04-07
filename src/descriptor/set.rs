//! Unsafe interface to descriptor sets.

use std::{
	marker::PhantomData,
	sync::Arc
};
use ash::vk;
use crate::{
	resource
};
use super::{
	pool,
	Pool
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
pub unsafe trait Set: Sized {
	/// Descriptor set layout.
	type Layout: Layout;

	/// Pool.
	type Pool: pool::Reference;

	fn write(&self, update: &mut super::UpdateSet<Self>);
}

pub struct Instance<S: Set> {
	handle: RawHandle,
	data: Option<S>
}

impl<S: Set> Instance<S> {
	pub unsafe fn from_raw(handle: RawHandle) -> Self {
		Self {
			handle,
			data: None
		}
	}

	pub fn bound(&self) -> Bound<S> {
		Bound {
			inner: self
		}
	}

	pub fn bound_mut(&mut self) -> BoundMut<S> {
		BoundMut {
			inner: self
		}
	}

	pub(crate) fn set_data(&mut self, set: S) {
		self.data = Some(set)
	}
}

unsafe impl<S: Set> resource::AbstractReference for Instance<S> {
	fn uid(&self) -> u64 {
		use ash::vk::Handle;
		self.handle.as_raw()
	}
}

unsafe impl<S: Set> resource::Reference for Instance<S> {
	type Handle = RawHandle;

	fn handle(&self) -> RawHandle {
		self.handle
	}
}

pub struct Bound<'a, S: Set> {
	inner: &'a Instance<S>
}

impl<'a, S: Set> AsRef<S> for Bound<'a, S> {
	fn as_ref(&self) -> &S {
		self.inner.data.as_ref().unwrap()
	}
}

pub struct BoundMut<'a, S: Set> {
	inner: &'a mut Instance<S>
}

impl<'a, S: Set> BoundMut<'a, S> {
	/// Directly write the descriptor of the given set.
	pub fn write<D: super::Descriptor, T>(&mut self, offset: u32, value: T) where S: super::Write<D, T>, S::Layout: layout::HasDescriptor<D> {
		unsafe {
			let mut update = super::Update::new();
			let mut update_set = update.update_set(self.inner);
			update_set.write_descriptor(offset, &value); // this is safe because `update` is dropped just after this call.
			self.as_mut().set(value)
		}
	}
}

impl<'a, S: Set> AsRef<S> for BoundMut<'a, S> {
	fn as_ref(&self) -> &S {
		self.inner.data.as_ref().unwrap()
	}
}

impl<'a, S: Set> AsMut<S> for BoundMut<'a, S> {
	fn as_mut(&mut self) -> &mut S {
		self.inner.data.as_mut().unwrap()
	}
}

// pub unsafe trait InitFrom<T>: Set {
// 	/// Converts an untyped descriptor set into
// 	/// a typed descriptor sets.
// 	/// 
// 	/// ## Safety
// 	/// 
// 	/// The layout and the resources attached to the descriptor set must
// 	/// outlive the descriptor set.
// 	unsafe fn init_from(
// 		layout: &Arc<Self::Layout>,
// 		value: T,
// 		pool: Self::Pool,
// 		handle: RawHandle
// 	) -> Self;
// }

/// Descriptor set write.
pub unsafe trait Write<T>: Set {
	/// Write this value to the descriptor set.
	/// 
	/// ## Safety
	/// 
	/// Any resource used by the descriptor set after this operation
	/// must outlive the descriptor set.
	fn update(&self, update: &mut super::UpdateSet<Self>);
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
pub unsafe trait Sets<'r> {
	/// Descriptor set layouts.
	type Layouts: Layouts;

	/// Pool.
	type Pool: pool::Reference;

	fn into_descriptor_sets(self) -> Vec<resource::Ref<'r>>;
}

unsafe impl<'r, S: 'r + Set> Sets<'r> for Instance<S> {
	type Layouts = layout::Instance<S::Layout>;

	type Pool = S::Pool;

	fn into_descriptor_sets(self) -> Vec<resource::Ref<'r>> {
		vec!(resource::Ref::from(self))
	}
}

/// No layouts.
unsafe impl<'r> Sets<'r> for () {
	type Layouts = ();

	type Pool = ();

	fn into_descriptor_sets(self) -> Vec<resource::Ref<'r>> {
		vec![]
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
pub unsafe trait InitAllFrom<'r, T>: Sets<'r> {
	/// Converts an untyped list of descriptor sets into
	/// a typed array/tuple of descriptor sets.
	/// 
	/// ## Safety
	/// 
	/// The layouts and the resources attached to the descriptor set must
	/// outlive the descriptor set.
	unsafe fn init_from(
		layouts: &Arc<Self::Layouts>,
		values: T,
		pool: Self::Pool,
		handle: Vec<RawHandle>
	) -> Self;
}

/// Describes the transition between two descriptor sets.
/// 
/// This trait is used to call [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html)
/// with the appropriate parameters.
pub unsafe trait Transition<'r, A, B> {
	type Handles<'a>: AsRef<[RawHandle]>;
	type Offsets<'a>: AsRef<[u32]>;

	fn first_set(&self) -> u32;

	fn descriptor_sets<'a>(&'a self) -> Self::Handles<'a>;

	fn dynamic_offsets<'a>(&'a self) -> Self::Offsets<'a>;

	fn into_descriptor_sets(self) -> Vec<resource::Ref<'r>>;
}

pub unsafe trait SendTransition<'r, A, B>: Transition<'r, A, B> {
	fn into_send_descriptor_sets(self) -> Vec<resource::SendRef<'r>>;
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

unsafe impl<P: pool::Reference, L: Layout> resource::AbstractReference for Raw<P, L> {
	fn uid(&self) -> u64 {
		use ash::vk::Handle;
		self.handle.as_raw()
	}
}

unsafe impl<P: pool::Reference, L: Layout> resource::Reference for Raw<P, L> {
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