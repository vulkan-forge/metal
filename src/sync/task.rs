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
}

/// Task that *can* wait on a semaphore.
pub unsafe trait Wait: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error;

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
	type Error;

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

impl<P: future::SignalSemaphore, T: Wait<Payload=()>> Delayed<P, T> {
	/// Execute the task without signaling any semaphore or fence upon completion.
	pub fn in_parallel(self) -> Result<(T::Output, P), T::Error> {
		let (output, (past, ())) = self.execute(None, None)?;
		Ok((output, past))
	}
}
 
unsafe impl<P: future::SignalSemaphore, T: Wait> Task for Delayed<P, T> {
	type Output = T::Output;
	type Error = T::Error;
	type Payload = (P, T::Payload);

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error> {
		let (output, payload) = self.task.execute(Some(std::slice::from_ref(self.past.semaphore())), signal_semaphore, signal_fence)?;
		Ok((output, (self.past, payload)))
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
	type Payload = (P, T::Payload);

	#[inline]
	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error> {
		let (output, payload) = self.task.execute(Some(self.past.semaphores()), Some(std::slice::from_ref(&self.wait_pipeline_stage_mask)), signal_semaphore, signal_fence)?;
		Ok((output, (self.past, payload)))
	}
}

impl<P: future::SignalSemaphores, T: WaitPipelineStages + SignalSemaphore> SignalSemaphore for DelayedPipelineStages<P, T> {}
impl<P: future::SignalSemaphores, T: WaitPipelineStages + SignalFence> SignalFence for DelayedPipelineStages<P, T> {}