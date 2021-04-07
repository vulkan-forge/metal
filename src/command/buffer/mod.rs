use ash::{
	vk,
	version::DeviceV1_0
};
use std::collections::HashSet;
use crate::{
	resource,
	OomError,
	DeviceOwned,
	sync::{
		future::Futures,
		task
	}
};

pub mod local_recorder;
mod recorder;

pub use local_recorder::LocalRecorder;
pub use recorder::Recorder;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub enum RecordError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for RecordError {
	fn from(r: vk::Result) -> RecordError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => RecordError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => RecordError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub type BufferCopy = vk::BufferCopy;

pub type VulkanBuffer = vk::CommandBuffer;

/// Command buffer trait.
pub trait Buffer: Sized + DeviceOwned {
	fn handle(&self) -> VulkanBuffer;

	fn record<'a, F>(self, f: F) -> Result<Recorded<'a, Self>, RecordError> where F: FnOnce(&mut Recorder<'a, Self>) -> (), Self: Send {
		let infos = vk::CommandBufferBeginInfo {
			flags: vk::CommandBufferUsageFlags::empty(), // TODO
			p_inheritance_info: std::ptr::null(), // no inheritance for primary buffers.
			..Default::default()
		};

		unsafe {
			self.device().handle().begin_command_buffer(self.handle(), &infos)?
		}

		let mut recorder = Recorder {
			buffer: self,
			resources: HashSet::new()
		};

		f(&mut recorder);

		unsafe {
			recorder.buffer.device().handle().end_command_buffer(recorder.buffer.handle())?
		}

		Ok(Recorded {
			buffer: recorder.buffer,
			resources: recorder.resources
		})
	}

	fn record_local<'a, F>(self, f: F) -> Result<LocallyRecorded<'a, Self>, RecordError> where F: FnOnce(&mut LocalRecorder<'a, Self>) -> () {
		let infos = vk::CommandBufferBeginInfo {
			flags: vk::CommandBufferUsageFlags::empty(), // TODO
			p_inheritance_info: std::ptr::null(), // no inheritance for primary buffers.
			..Default::default()
		};

		unsafe {
			self.device().handle().begin_command_buffer(self.handle(), &infos)?
		}

		let mut recorder = LocalRecorder {
			buffer: self,
			resources: HashSet::new()
		};

		f(&mut recorder);

		unsafe {
			recorder.buffer.device().handle().end_command_buffer(recorder.buffer.handle())?
		}

		Ok(LocallyRecorded {
			buffer: recorder.buffer,
			resources: recorder.resources
		})
	}
}

impl<'a, B: Buffer> Buffer for &'a B {
	#[inline]
	fn handle(&self) -> VulkanBuffer {
		(*self).handle()
	}
}

/// Recorded command buffer trait.
pub unsafe trait RecordedBuffer: task::Payload {
	fn handle(&self) -> vk::CommandBuffer;

	/// Validate the references owned by the command buffer against the given past.
	fn check_borrow_rules<P: Futures>(&self, past: &P);
}

unsafe impl<'a, B: RecordedBuffer> RecordedBuffer for &'a B {
	#[inline]
	fn handle(&self) -> VulkanBuffer {
		(*self).handle()
	}

	#[inline]
	fn check_borrow_rules<P: Futures>(&self, past: &P) {
		(*self).check_borrow_rules(past)
	}
}

pub struct Recorded<'a, B: Buffer> {
	buffer: B,
	resources: HashSet<resource::SendRef<'a>>
}

impl<'a, B: Buffer> Recorded<'a, B> {
	pub fn resources(&self) -> &HashSet<resource::SendRef<'a>> {
		&self.resources
	}
}

unsafe impl<'a, B: Buffer> RecordedBuffer for Recorded<'a, B> {
	#[inline]
	fn handle(&self) -> vk::CommandBuffer {
		self.buffer.handle()
	}

	#[inline]
	fn check_borrow_rules<P: Futures>(&self, past: &P) {
		for resource in &self.resources {
			resource.check_borrow_rules(past)
		}
	}
}

unsafe impl<'a, B: Buffer> task::Payload for Recorded<'a, B> {
	#[inline]
	fn uses(&self, resource: &dyn resource::AbstractReference) -> bool {
		for r in &self.resources {
			if r.aliases(resource) {
				return true
			}
		}

		false
	}
}

pub struct LocallyRecorded<'a, B: Buffer> {
	buffer: B,
	resources: HashSet<resource::Ref<'a>>
}

impl<'a, B: Buffer> LocallyRecorded<'a, B> {
	pub fn resources(&self) -> &HashSet<resource::Ref<'a>> {
		&self.resources
	}
}

unsafe impl<'a, B: Buffer> RecordedBuffer for LocallyRecorded<'a, B> {
	#[inline]
	fn handle(&self) -> vk::CommandBuffer {
		self.buffer.handle()
	}

	#[inline]
	fn check_borrow_rules<P: Futures>(&self, past: &P) {
		for resource in &self.resources {
			resource.check_borrow_rules(past)
		}
	}
}

unsafe impl<'a, B: Buffer> task::Payload for LocallyRecorded<'a, B> {
	#[inline]
	fn uses(&self, resource: &dyn resource::AbstractReference) -> bool {
		for r in &self.resources {
			if r.aliases(resource) {
				return true
			}
		}

		false
	}
}