use ash::vk;
use crate::pipeline;
use super::{
	future,
	Semaphore,
	semaphore,
	Fence,
	fence
};

pub unsafe trait Task: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error;

	type Past;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Past), Self::Error>;

	/// Execute the task without signaling any semaphore or fence upon completion.
	fn in_parallel(self) -> Result<(Self::Output, Self::Past), Self::Error> {
		self.execute(None, None)
	}

	fn then_signal_semaphore<S: Semaphore>(self, semaphore: S) -> Result<(Self::Output, semaphore::Future<Self::Past, S>), Self::Error> where Self: SignalSemaphore {
		semaphore.signal(self)
	}

	fn then_signal_fence<F: Fence>(self, fence: F) -> Result<(Self::Output, fence::Future<Self::Past, F>), Self::Error> where Self: SignalFence {
		fence.signal(self)
	}

	fn then_signal_semaphore_and_fence<S: Semaphore, F: Fence>(self, semaphore: S, fence: F) -> Result<(Self::Output, fence::FutureWithSemaphore<Self::Past, F, S>), Self::Error> where Self: SignalSemaphore + SignalFence {
		fence.signal_with_semaphore(semaphore, self)
	}
}

/// Task that *can* wait on a semaphore.
pub unsafe trait Wait: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<Self::Output, Self::Error>;
}

unsafe impl<T: Wait> Task for T {
	type Output = T::Output;
	type Error = T::Error;
	type Past = ();

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, ()), Self::Error> {
		Ok((Wait::execute(self, None, signal_semaphore, signal_fence)?, ()))
	}
}

/// Task that *can* wait on a semaphore.
pub unsafe trait WaitPipelineStages: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		wait_pipeline_stage_mask: Option<&[pipeline::stage::Flags]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<Self::Output, Self::Error>;
}

unsafe impl<T: WaitPipelineStages> Wait for T {
	type Output = T::Output;
	type Error = T::Error;

	#[inline]
	fn execute(
		self,
		wait_semaphore: Option<&[vk::Semaphore]>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<Self::Output, Self::Error> {
		WaitPipelineStages::execute(self, wait_semaphore, None, signal_semaphore, signal_fence)
	}
}

/// A task that *can* signal a semaphore upon completion.
pub trait SignalSemaphore: Task {}

/// A task that *can* signal a fence upon completion.
pub trait SignalFence: Task {}

pub struct Delayed<P, T> {
	past: P,
	task: T,
}

impl<P: future::SignalSemaphore, T: Wait> Delayed<P, T> {
	pub fn new(past: P, task: T) -> Self {
		Delayed {
			past, task
		}
	}
}

unsafe impl<P: future::SignalSemaphore, T: Wait> Task for Delayed<P, T> {
	type Output = T::Output;
	type Error = T::Error;
	type Past = P;

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, P), Self::Error> {
		let output = self.task.execute(Some(std::slice::from_ref(self.past.semaphore())), signal_semaphore, signal_fence)?;
		Ok((output, self.past))
	}
}

impl<P: future::SignalSemaphore, T: Wait + SignalSemaphore> SignalSemaphore for Delayed<P, T> {}
impl<P: future::SignalSemaphore, T: Wait + SignalFence> SignalFence for Delayed<P, T> {}

pub struct DelayedPipelineStages<P, T> {
	past: P,
	task: T,
	wait_pipeline_stage_mask: pipeline::stage::Flags
}

impl<P: future::SignalSemaphores, T: WaitPipelineStages> DelayedPipelineStages<P, T> {
	pub fn new(past: P, task: T, wait_pipeline_stage_mask: pipeline::stage::Flags) -> Self {
		DelayedPipelineStages {
			past, task, wait_pipeline_stage_mask
		}
	}
}

unsafe impl<P: future::SignalSemaphores, T: WaitPipelineStages> Task for DelayedPipelineStages<P, T> {
	type Output = T::Output;
	type Error = T::Error;
	type Past = P;

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, P), Self::Error> {
		let output = self.task.execute(Some(self.past.semaphores()), Some(std::slice::from_ref(&self.wait_pipeline_stage_mask)), signal_semaphore, signal_fence)?;
		Ok((output, self.past))
	}
}

impl<P: future::SignalSemaphores, T: WaitPipelineStages + SignalSemaphore> SignalSemaphore for DelayedPipelineStages<P, T> {}
impl<P: future::SignalSemaphores, T: WaitPipelineStages + SignalFence> SignalFence for DelayedPipelineStages<P, T> {}