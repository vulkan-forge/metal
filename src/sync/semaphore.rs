use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	borrow::Borrow,
	sync::Arc
};
use crate::{
	OomError,
	Device,
	DeviceOwned
};
use super::{
	task,
	future
};

pub trait Semaphore {
	fn handle(&self) -> &vk::Semaphore;

	/// Signal this semaphore after executing the given task.
	fn signal<T: task::SignalSemaphore>(self, task: T) -> Result<(T::Output, Future<T::Past, Self>), T::Error> where Self: Sized {
		let (output, past) = task.execute(Some(&[*self.handle()]), None)?;

		let future = Future {
			past,
			semaphore: self
		};

		Ok((output, future))
	}
}

pub struct Future<P, S> {
	past: P,
	semaphore: S,
}

impl<P, S> Future<P, S> {
	pub fn past(&self) -> &P {
		&self.past
	}
}

unsafe impl<P, S: Semaphore> future::Future for Future<P, S> {
	fn signal_semaphore(&self) -> Option<&vk::Semaphore> {
		Some(self.semaphore.handle())
	}
}

impl<P, S: Semaphore> future::SignalSemaphore for Future<P, S> {}

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

pub struct Raw {
	device: Arc<Device>,
	handle: vk::Semaphore
}

impl Raw {
	pub fn new(device: &Arc<Device>) -> Result<Raw, CreationError> {
		let infos = vk::SemaphoreCreateInfo {
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_semaphore(&infos, None)?
		};

		Ok(Raw {
			device: device.clone(),
			handle
		})
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_semaphore(self.handle, None)
		}
	}
}

impl<T: Borrow<Raw>> Semaphore for T {
	fn handle(&self) -> &vk::Semaphore {
		&self.borrow().handle
	}
}

impl DeviceOwned for Raw {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}