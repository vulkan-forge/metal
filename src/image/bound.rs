use std::{
	sync::Arc
};
use ash::{
	vk,
};
use crate::{
	Device,
	DeviceOwned,
	mem::Slot,
	resource
};
use super::{
	Unbound,
	Image
};

pub struct Bound<S: Slot> {
	inner: Unbound,
	slot: S
}

impl<S: Slot> Bound<S> {
	pub(crate) fn new(inner: Unbound, slot: S) -> Self {
		Self {
			inner,
			slot
		}
	}

	pub fn memory_slot(&self) -> &S {
		&self.slot
	}

	/// Releases the image and returns its memory slot.
	pub fn unbind(self) -> S {
		self.slot
	}

	pub fn boxed(self) -> Bound<Box<dyn Send + Slot>> where S: Send {
		Bound {
			inner: self.inner,
			slot: Box::new(self.slot)
		}
	}
}

impl<S: Slot> DeviceOwned for Bound<S> {
	fn device(&self) -> &Arc<Device> {
		self.inner.device()
	}
}

// unsafe impl<S: Slot> resource::AbstractReference for Bound<S> {
// 	fn uid(&self) -> u64 {
// 		use ash::vk::Handle;
// 		self.inner.handle().as_raw()
// 	}
// }

unsafe impl<S: Slot> resource::Reference for Bound<S> {
	type Handle = vk::Image;

	fn handle(&self) -> vk::Image {
		self.inner.handle()
	}
}

unsafe impl<S: Slot> Image for Bound<S> {
	// ...
}