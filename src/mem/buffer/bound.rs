use std::{
	sync::Arc,
	ops::Deref
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
		HostVisible,
		Allocator,
		buffer,
		Buffer
	}
};
use super::Typed;

/// Bound buffer.
pub struct Bound<S: Slot> {
	inner: buffer::Unbound,
	slot: S
}

impl<S: Slot> Bound<S> {
	pub(crate) fn new(inner: buffer::Unbound, slot: S) -> Self {
		Bound {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &S {
		&self.slot
	}

	/// Releases the buffer and returns its memory slot.
	pub fn unbind(self) -> S {
		self.slot
	}

	pub unsafe fn into_typed<T>(self) -> Typed<T> where S: Send {
		Typed::from_raw_parts(self.inner, Box::new(self.slot))
	}

	pub fn boxed(self) -> Bound<Box<dyn Send + Slot>> where S: Send {
		Bound {
			inner: self.inner,
			slot: Box::new(self.slot)
		}
	}
}

unsafe impl<S: Slot> crate::Resource for Bound<S> {
	type Handle = vk::Buffer;

	fn handle(&self) -> vk::Buffer {
		self.inner.handle()
	}
}

unsafe impl<S: Slot> Buffer for Bound<S> {
	// ...
}

impl<S: Slot> DeviceOwned for Bound<S> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

// impl<A: Allocator> Deref for Bound<A> {
// 	type Target = buffer::Unbound;

// 	fn deref(&self) -> &buffer::Unbound {
// 		&self.inner
// 	}
// }

/// Host visible buffer.
pub struct HostVisibleBound<A: Allocator> {
	inner: buffer::Unbound,
	slot: HostVisible<A::Slot>
}

impl<A: Allocator> HostVisibleBound<A> {
	pub(crate) fn new(inner: buffer::Unbound, slot: HostVisible<A::Slot>) -> Self {
		HostVisibleBound {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &HostVisible<A::Slot> {
		&self.slot
	}
}

impl<A: Allocator> DeviceOwned for HostVisibleBound<A> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

impl<A: Allocator> Deref for HostVisibleBound<A> {
	type Target = buffer::Unbound;

	fn deref(&self) -> &buffer::Unbound {
		&self.inner
	}
}