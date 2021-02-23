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
	future,
	Semaphore
};

#[derive(Debug)]
pub enum WaitError {
	OutOfMemory(OomError),
	DeviceLost
}

impl From<vk::Result> for WaitError {
	fn from(r: vk::Result) -> WaitError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => WaitError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => WaitError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_DEVICE_LOST => WaitError::DeviceLost,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub struct DeviceLost;

impl From<vk::Result> for DeviceLost {
	fn from(r: vk::Result) -> DeviceLost {
		match r {
			vk::Result::ERROR_DEVICE_LOST => DeviceLost,
			_ => unreachable!()
		}
	}
}

pub trait Fence: DeviceOwned {
	fn handle(&self) -> &vk::Fence;

	/// Signal this fence after executing the given task.
	fn signal<T: task::SignalFence>(self, task: T) -> Result<(T::Output, Future<T::Past, Self>), T::Error> where Self: Sized {
		let (output, past) = task.execute(None, Some(*self.handle()))?;

		let future = Future {
			past,
			fence: self
		};

		Ok((output, future))
	}

	/// Signal this fence after executing the given task.
	fn signal_with_semaphore<S: Semaphore, T: task::SignalFence + task::SignalSemaphore>(self, semaphore: S, task: T) -> Result<(T::Output, FutureWithSemaphore<T::Past, Self, S>), T::Error> where Self: Sized {
		let (output, past) = task.execute(Some(&[*semaphore.handle()]), Some(*self.handle()))?;

		let future = FutureWithSemaphore {
			past,
			fence: self,
			semaphore
		};

		Ok((output, future))
	}

	/// Block until the fence is signaled.
	fn wait(&self, timeout: Option<u64>) -> Result<(), WaitError> {
		unsafe {
			self.device().handle().wait_for_fences(std::slice::from_ref(self.handle()), true, timeout.unwrap_or(u64::MAX))?
		}

		Ok(())
	}

	fn is_signaled(&self) -> Result<bool, DeviceLost> {
		unsafe {
			Ok(self.device().handle().get_fence_status(*self.handle())?)
		}
	}

	/// Reset the fence.
	fn reset(&self) -> Result<(), OomError> {
		unsafe {
			self.device().handle().reset_fences(std::slice::from_ref(self.handle()))?
		}

		Ok(())
	}
}

pub struct Future<P, F> {
	past: P,
	fence: F,
}

impl<P, F> Future<P, F> {
	pub fn past(&self) -> &P {
		&self.past
	}
}

unsafe impl<P, F: Fence> future::Future for Future<P, F> {
	fn signal_fence(&self) -> Option<&vk::Fence> {
		Some(self.fence.handle())
	}
}

impl<P, F: Fence> future::SignalFence for Future<P, F> {
	fn wait(self, timeout: Option<u64>) -> Result<(), WaitError> {
		self.fence.wait(timeout)
	}
}

pub struct FutureWithSemaphore<P, F, S> {
	past: P,
	fence: F,
	semaphore: S
}

impl<P, F, S> FutureWithSemaphore<P, F, S> {
	pub fn past(&self) -> &P {
		&self.past
	}
}

unsafe impl<P, F: Fence, S: Semaphore> future::Future for FutureWithSemaphore<P, F, S> {
	fn signal_semaphore(&self) -> Option<&vk::Semaphore> {
		Some(self.semaphore.handle())
	}

	fn signal_fence(&self) -> Option<&vk::Fence> {
		Some(self.fence.handle())
	}
}

impl<P, F: Fence, S: Semaphore> future::SignalSemaphore for FutureWithSemaphore<P, F, S> {}
impl<P, F: Fence, S: Semaphore> future::SignalFence for FutureWithSemaphore<P, F, S> {
	fn wait(self, timeout: Option<u64>) -> Result<(), WaitError> {
		self.fence.wait(timeout)
	}
}

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

#[derive(PartialEq, Eq, Hash)]
pub struct Raw {
	device: Arc<Device>,
	handle: vk::Fence
}

impl Raw {
	pub fn new(device: &Arc<Device>) -> Result<Raw, CreationError> {
		let infos = vk::FenceCreateInfo {
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_fence(&infos, None)?
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
			self.device.handle().destroy_fence(self.handle, None)
		}
	}
}

impl<T: Borrow<Raw> + DeviceOwned> Fence for T {
	fn handle(&self) -> &vk::Fence {
		&self.borrow().handle
	}
}

impl DeviceOwned for Raw {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}