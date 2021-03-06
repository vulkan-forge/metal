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
pub struct Bound {
	inner: buffer::Unbound,
	slot: Box<dyn Slot>
}

impl Bound {
	pub(crate) fn new<S: Slot>(inner: buffer::Unbound, slot: S) -> Self {
		Bound {
			inner,
			slot: Box::new(slot)
		}
	}

	pub fn memory_slot(&self) -> &dyn Slot {
		self.slot.as_ref()
	}

	pub unsafe fn into_typed<T>(self) -> Typed<T> {
		Typed::from_raw_parts(self.inner, self.slot)
	}
}

unsafe impl crate::Resource for Bound {
	fn uid(&self) -> u64 {
		self.inner.handle().as_raw()
	}
}

unsafe impl Buffer for Bound {
	fn handle(&self) -> vk::Buffer {
		self.inner.handle()
	}
}

impl DeviceOwned for Bound {
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