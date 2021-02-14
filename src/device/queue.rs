use std::sync::Arc;
use ash::vk;
use crate::instance::physical_device::QueueFamily;
use super::Device;

pub struct Queue {
	device: Arc<Device>,
	pub(crate) handle: vk::Queue,
	queue_family_index: u32,
	queue_index: u32
}

impl Queue {
	pub(crate) fn new(device: &Arc<Device>, handle: vk::Queue, queue_family_index: u32, queue_index: u32) -> Queue {
		Queue {
			device: device.clone(),
			handle,
			queue_family_index,
			queue_index
		}
	}

	pub fn family_index(&self) -> u32 {
		self.queue_family_index
	}

	pub fn family(&self) -> QueueFamily {
		self.device.physical_device().queue_family(self.queue_family_index).unwrap()
	}
}
