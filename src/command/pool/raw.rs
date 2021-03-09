use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	rc::Rc
};
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
	Handle
};

pub type RcBuffer = Buffer<Rc<Raw>>;

pub struct Raw {
	device: Arc<Device>,
	handle: vk::CommandPool
}

impl Raw {
	pub fn new(device: &Arc<Device>, queue_family: physical_device::QueueFamily) -> Result<Self, CreationError> {
		assert_eq!(device.physical_device().index(), queue_family.physical_device().index());
		
		let infos = vk::CommandPoolCreateInfo {
			queue_family_index: queue_family.index(),
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_command_pool(&infos, None)?
		};

		Ok(Self {
			device: device.clone(),
			handle
		})
	}

	pub unsafe fn allocate_into<F, B>(&self, count: u32, f: F) -> Result<Vec<B>, AllocError> where F: Fn(vk::CommandBuffer) -> B {
		let infos = vk::CommandBufferAllocateInfo {
			command_pool: self.handle,
			level: vk::CommandBufferLevel::PRIMARY,
			command_buffer_count: count,
			..Default::default()
		};

		let handles = unsafe {
			self.device.handle().allocate_command_buffers(&infos)?
		};

		Ok(handles.into_iter().map(f).collect())
	}

	pub fn allocate_rc(self: &Rc<Self>, count: u32) -> Result<Vec<RcBuffer>, AllocError> {
		unsafe {
			self.allocate_into(count, |h| Buffer::new(self.clone(), h))
		}
	}
}

impl Pool for Raw {
	type Buffer<'a> = Buffer<&'a Self>;

	fn allocate(&self, count: u32) -> Result<Vec<Buffer<&Self>>, AllocError> {
		unsafe {
			self.allocate_into(count, |h| Buffer::new(self, h))
		}
	}
}

impl Handle for Raw {
	fn handle(&self) -> vk::CommandPool {
		self.handle
	}

	unsafe fn free(&self, buffer_handles: &[vk::CommandBuffer]) {
		self.device.handle().free_command_buffers(self.handle, buffer_handles)
	}
}

impl<P: DeviceOwned + std::ops::Deref<Target=Raw>> Handle for P {
	fn handle(&self) -> vk::CommandPool {
		self.deref().handle()
	}

	unsafe fn free(&self, buffer_handles: &[vk::CommandBuffer]) {
		self.deref().free(buffer_handles)
	}
}

impl<P: DeviceOwned + std::ops::Deref<Target=Raw>> Pool for P {
	type Buffer<'a> = Buffer<&'a Raw>;

	fn allocate(&self, count: u32) -> Result<Vec<Buffer<&Raw>>, AllocError> {
		self.deref().allocate(count)
	}
}

impl DeviceOwned for Raw {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_command_pool(self.handle, None)
		}
	}
}

pub struct Buffer<P: Handle> {
	pool: P,
	handle: vk::CommandBuffer
}

impl<P: Handle> Buffer<P> {
	pub(crate) fn new(pool: P, handle: vk::CommandBuffer) -> Self {
		Buffer {
			pool,
			handle
		}
	}

	fn into_raw_parts(self) -> (P, vk::CommandBuffer) {
		let pool = unsafe { std::ptr::read(&self.pool) };
		let handle = self.handle;
		std::mem::forget(self);
		(pool, handle)
	}

	pub unsafe fn map_pool<F, Q: Handle>(self, f: F) -> Buffer<Q> where F: FnOnce(P) -> Q {
		let (pool, handle) = self.into_raw_parts();

		Buffer {
			pool: f(pool),
			handle
		}
	}
}

impl<P: Handle> command::Buffer for Buffer<P> {
	fn handle(&self) -> vk::CommandBuffer {
		self.handle
	}
}

impl<P: Handle> DeviceOwned for Buffer<P> {
	fn device(&self) -> &Arc<Device> {
		self.pool.device()
	}
}

impl<P: Handle> Drop for Buffer<P> {
	fn drop(&mut self) {
		unsafe {
			self.pool.device().handle().free_command_buffers(self.pool.handle(), &[self.handle])
		}
	}
}