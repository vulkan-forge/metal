use std::{
	sync::Arc,
	ops::Deref
};
use crate::{
	Device,
	DeviceOwned,
	buffer
};
use super::Allocator;

/// Bound buffer.
pub struct Buffer<A: Allocator> {
	inner: buffer::Unbound,
	slot: A::Slot
}

impl<A: Allocator> Buffer<A> {
	pub(crate) fn new(inner: buffer::Unbound, slot: A::Slot) -> Self {
		Buffer {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &A::Slot {
		&self.slot
	}
}

impl<A: Allocator> DeviceOwned for Buffer<A> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

impl<A: Allocator> Deref for Buffer<A> {
	type Target = buffer::Unbound;

	fn deref(&self) -> &buffer::Unbound {
		&self.inner
	}
}

/// Host visible buffer.
pub struct HostVisibleBuffer<A: Allocator> {
	inner: buffer::Unbound,
	slot: A::HostVisibleSlot
}

impl<A: Allocator> HostVisibleBuffer<A> {
	pub(crate) fn new(inner: buffer::Unbound, slot: A::HostVisibleSlot) -> Self {
		HostVisibleBuffer {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &A::HostVisibleSlot {
		&self.slot
	}
}

impl<A: Allocator> DeviceOwned for HostVisibleBuffer<A> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

impl<A: Allocator> Deref for HostVisibleBuffer<A> {
	type Target = buffer::Unbound;

	fn deref(&self) -> &buffer::Unbound {
		&self.inner
	}
}