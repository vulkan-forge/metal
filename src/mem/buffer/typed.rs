use std::{
	sync::Arc,
	ops::Deref,
	marker::PhantomData
};
use ash::{
	vk,
	vk::Handle
};
use crate::{
	resource,
	Device,
	DeviceOwned,
	mem::{
		Slot,
		HostVisible,
		Allocator,
		buffer
	}
};

/// Typed buffer.
pub struct Typed<T, S> {
	inner: buffer::Unbound,
	slot: S,
	t: PhantomData<T>,
	len: u64
}

impl<T, S> Typed<T, S> {
	pub(crate) unsafe fn from_raw_parts(inner: buffer::Unbound, slot: S) -> Self {
		let len = inner.len() / std::mem::size_of::<T>() as u64;
		Self {
			inner,
			slot,
			t: PhantomData,
			len
		}
	}

	pub fn memory_slot(&self) -> &S {
		&self.slot
	}

	/// Returns the number of elements in the buffer.
	/// 
	/// This is different from the byte size of the buffer.
	pub fn len(&self) -> u64 {
		self.len
	}
}

// unsafe impl<T, S> resource::AbstractReference for Typed<T, S> {
// 	fn uid(&self) -> u64 {
// 		self.inner.handle().as_raw()
// 	}
// }

unsafe impl<T, S> resource::Reference for Typed<T, S> {
	type Handle = vk::Buffer;

	fn handle(&self) -> vk::Buffer {
		self.inner.handle()
	}
}

unsafe impl<T, S> buffer::sub::Read for Typed<T, S> {
	fn byte_offset(&self) -> u64 {
		0
	}

	fn byte_len(&self) -> u64 {
		self.inner.len()
	}
}

unsafe impl<T, S> buffer::sub::TypedRead for Typed<T, S> {
	type Item = T;

	fn len(&self) -> u64 {
		self.len()
	}
}

impl<T, S> DeviceOwned for Typed<T, S> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

// impl<A: Allocator> Deref for Typed<A> {
// 	type Target = buffer::Unbound;

// 	fn deref(&self) -> &buffer::Unbound {
// 		&self.inner
// 	}
// }

/// Host visible buffer.
pub struct HostVisibleTyped<A: Allocator> {
	inner: buffer::Unbound,
	slot: HostVisible<A::Slot>
}

impl<A: Allocator> HostVisibleTyped<A> {
	pub(crate) fn new(inner: buffer::Unbound, slot: HostVisible<A::Slot>) -> Self {
		HostVisibleTyped {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &HostVisible<A::Slot> {
		&self.slot
	}
}

impl<A: Allocator> DeviceOwned for HostVisibleTyped<A> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

impl<A: Allocator> Deref for HostVisibleTyped<A> {
	type Target = buffer::Unbound;

	fn deref(&self) -> &buffer::Unbound {
		&self.inner
	}
}