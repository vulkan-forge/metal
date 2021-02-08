use ash::vk;

pub struct Image {
	/// The image is automatically released with the swapchain.
	handle: vk::Image
}

impl Image {
	pub(crate) fn new(handle: vk::Image) -> Self {
		Image {
			handle
		}
	}
}