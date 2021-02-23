use ash::vk;
use std::{
	sync::Arc,
	rc::Rc
};
use crate::{
	Device,
	DeviceOwned
};
use super::{
	Inner
};

pub struct Image<W> {
	inner: Rc<Inner<W>>,

	/// The image is automatically released with the swapchain.
	handle: vk::Image
}

impl<W> Image<W> {
	pub(crate) fn new(inner: &Rc<Inner<W>>, handle: vk::Image) -> Self {
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

unsafe impl<W> crate::Image for Image<W> {
	fn handle(&self) -> vk::Image {
		self.handle
	}
}