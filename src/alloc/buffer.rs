use std::{
	sync::Arc,
	ops::Deref
};
use crate::{
	Device,
	DeviceOwned,
	buffer::UnboundBuffer
};
use super::Allocator;

/// Bound buffer.
pub struct Buffer<A: Allocator> {
	inner: UnboundBuffer,
	slot: A::Slot
}

impl<A: Allocator> Buffer<A> {
	pub(crate) fn new(inner: UnboundBuffer, slot: A::Slot) -> Self {
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
	type Target = UnboundBuffer;

	fn deref(&self) -> &UnboundBuffer {
		&self.inner
	}
}

/// Host visible buffer.
pub struct HostVisibleBuffer<A: Allocator> {
	inner: UnboundBuffer,
	slot: A::HostVisibleSlot
}

impl<A: Allocator> HostVisibleBuffer<A> {
	pub(crate) fn new(inner: UnboundBuffer, slot: A::HostVisibleSlot) -> Self {
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
	type Target = UnboundBuffer;

	fn deref(&self) -> &UnboundBuffer {
		&self.inner
	}
}