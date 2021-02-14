use ash::vk;
use std::sync::Arc;
use super::Swapchain;

pub struct Image<W> {
	/// Swapchain handle.
	swapchain: Arc<Swapchain<W>>,

	/// The image is automatically released with the swapchain.
	handle: vk::Image
}

impl<W> Image<W> {
	pub(crate) fn new(swapchain: Arc<Swapchain<W>>, handle: vk::Image) -> Self {
		Image {
			swapchain,
			handle
		}
	}
}