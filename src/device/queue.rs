use std::sync::Arc;
use ash::vk;
use super::Device;

pub struct Queue {
	pub(crate) handle: vk::Queue,
	device: Arc<Device>
}

impl Queue {
	pub(crate) fn new(device: &Arc<Device>, handle: vk::Queue) -> Queue {
		Queue {
			handle,
			device: device.clone()
		}
	}
}
