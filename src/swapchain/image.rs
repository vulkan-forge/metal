use ash::vk;
use std::{
	sync::Arc,
	rc::Rc
};
use crate::{
	Device,
	DeviceOwned,
	Resource
};
use super::{
	Inner
};

pub struct Image<W> {
	inner: Arc<Inner<W>>,

	/// The image is automatically released with the swapchain.
	handle: vk::Image
}

impl<W> Image<W> {
	pub(crate) fn new(inner: &Arc<Inner<W>>, handle: vk::Image) -> Self {
		Image {
			inner: inner.clone(),
			handle
		}
	}
}

impl<W> DeviceOwned for Image<W> {
	fn device(&self) -> &Arc<Device> {
		&self.inner.device
	}
}

unsafe impl<W> Resource for Image<W> {
	type Handle = vk::Image;

	fn handle(&self) -> vk::Image {
		self.handle
	}
}

unsafe impl<W> crate::Image for Image<W> {
	// ...
}