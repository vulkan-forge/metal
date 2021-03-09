use ash::vk;
use crate::pipeline;
use super::{
	future,
	Semaphore,
	semaphore,
	Fence,
	fence
};

mod delayed;
mod map;

pub use delayed::*;
pub use map::*;

pub unsafe trait Task: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error: std::error::Error;

	/// Task payload.
	/// 
	/// Stores all the data borroed by the task that will be
	/// released upon task completion.
	type Payload;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error>;

	fn then_signal_semaphore<S: Semaphore>(self, semaphore: S) -> Result<(Self::Output, semaphore::Future<Self::Payload, S>), Self::Error> where Self: SignalSemaphore {
		semaphore.signal(self)
	}

	fn then_signal_fence<F: Fence>(self, fence: F) -> Result<(Self::Output, fence::Future<Self::Payload, F>), Self::Error> where Self: SignalFence {
		fence.signal(self)
	}

	fn then_signal_semaphore_and_fence<S: Semaphore, F: Fence>(self, semaphore: S, fence: F) -> Result<(Self::Output, fence::FutureWithSemaphore<Self::Payload, F, S>), Self::Error> where Self: SignalSemaphore + SignalFence {
		fence.signal_with_semaphore(semaphore, self)
	}

	fn map<F, U>(self, f: F) -> Map<Self, F> where F: FnOnce(Self::Output) -> U {
		Map::new(self, f)
	}
}

/// Task that *can* wait on a semaphore.
pub unsafe trait Wait: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error: std::error::Error;

	/// Payload.
	type Payload;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error>;
}

unsafe impl<T: Wait> Task for T {
	type Output = T::Output;
	type Error = T::Error;
	type Payload = T::Payload;

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error> {
		Wait::execute(self, None, signal_semaphore, signal_fence)
	}
}

/// Task that *can* wait on a semaphore.
pub unsafe trait WaitPipelineStages: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error: std::error::Error;

	type Payload;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		wait_pipeline_stage_mask: Option<&[pipeline::stage::Flags]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error>;
}

unsafe impl<T: WaitPipelineStages> Wait for T {
	type Output = T::Output;
	type Error = T::Error;
	type Payload = T::Payload;

	#[inline]
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error> {
		WaitPipelineStages::execute(self, wait_semaphore, None, signal_semaphore, signal_fence)
	}
}

/// A task that *can* signal a semaphore upon completion.
pub trait SignalSemaphore: Task {}

/// A task that *can* signal a fence upon completion.
pub trait SignalFence: Task {}