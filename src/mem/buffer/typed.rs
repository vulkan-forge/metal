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
	Device,
	DeviceOwned,
	mem::{
		Slot,
		Allocator,
		buffer,
		Buffer,
		TypedBuffer
	}
};

/// Typed buffer.
pub struct Typed<T> {
	inner: buffer::Unbound,
	slot: Box<dyn Slot>,
	t: PhantomData<T>,
	len: u64
}

impl<T> Typed<T> {
	pub(crate) fn from_raw_parts(inner: buffer::Unbound, slot: Box<dyn Slot>) -> Self {
		let len = inner.len() / std::mem::size_of::<T>() as u64;
		Self {
			inner,
			slot,
			t: PhantomData,
			len
		}
	}

	pub fn memory_slot(&self) -> &dyn Slot {
		self.slot.as_ref()
	}

	/// Returns the number of elements in the buffer.
	/// 
	/// This is different from the byte size of the buffer.
	pub fn len(&self) -> u64 {
		self.len
	}
}

unsafe impl<T> crate::Resource for Typed<T> {
	fn uid(&self) -> u64 {
		self.inner.handle().as_raw()
	}
}

unsafe impl<T> Buffer for Typed<T> {
	fn handle(&self) -> vk::Buffer {
		self.inner.handle()
	}
}

unsafe impl<T> TypedBuffer for Typed<T> {
	type Item = T;
}

impl<T> DeviceOwned for Typed<T> {
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
	slot: A::HostVisibleSlot
}

impl<A: Allocator> HostVisibleTyped<A> {
	pub(crate) fn new(inner: buffer::Unbound, slot: A::HostVisibleSlot) -> Self {
		HostVisibleTyped {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &A::HostVisibleSlot {
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