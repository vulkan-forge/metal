use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc
};
use crossbeam_queue::SegQueue;
use crate::{
	instance::physical_device,
	Device,
	DeviceOwned,
	command
};
use super::{
	CreationError,
	AllocError,
	Pool,
	Handle,
	Raw,
	raw
};

pub struct SendHandle {
	device: Arc<Device>,
	handle: vk::CommandPool,
	free_queue: Arc<SegQueue<vk::CommandBuffer>>
}

unsafe impl Send for SendHandle {}

/// A command pool with `Send` buffers.
pub struct SyncPool {
	raw: Raw,
	free_queue: Arc<SegQueue<vk::CommandBuffer>>
}

impl SyncPool {
	pub fn new(device: &Arc<Device>, queue_family: physical_device::QueueFamily) -> Result<Self, CreationError> {
		Ok(SyncPool {
			raw: Raw::new(device, queue_family)?,
			free_queue: Arc::new(SegQueue::new())
		})
	}

	fn send_handle(&self) -> SendHandle {
		SendHandle {
			device: self.raw.device().clone(),
			handle: self.raw.handle(),
			free_queue: self.free_queue.clone()
		}
	}

	fn process_deallocations(&self) {
		panic!("TODO")
	}
}

impl Pool for SyncPool {
	type Buffer<'a> = Buffer;

	fn allocate(&self, count: u32) -> Result<Vec<Buffer>, AllocError> {
		self.process_deallocations();
		unsafe {
			self.raw.allocate_into(count, |h| Buffer::new(self.send_handle(), h))
		}
	}
}

impl DeviceOwned for SyncPool {
	fn device(&self) -> &Arc<Device> {
		self.raw.device()
	}
}

impl Drop for SyncPool {
	fn drop(&mut self) {
		self.process_deallocations()
	}
}

impl DeviceOwned for SendHandle {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl super::Handle for SendHandle {
	fn handle(&self) -> vk::CommandPool {
		self.handle
	}

	unsafe fn free(&self, buffer_handles: &[vk::CommandBuffer]) {
		for handle in buffer_handles {
			self.free_queue.push(*handle)
		}
	}
}

pub type Buffer = raw::Buffer<SendHandle>;