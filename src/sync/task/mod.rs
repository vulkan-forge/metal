use ash::vk;
use crate::{
	pipeline,
	resource
};
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

pub unsafe trait Payload {
	// fn uses(&self, resource: &dyn resource::AbstractReference) -> bool;
}

unsafe impl Payload for () {
	// fn uses(&self, _resource: &dyn resource::AbstractReference) -> bool {
	// 	false
	// }
}

unsafe impl<A: future::Futures, B: Payload> Payload for (A, B) {
	// fn uses(&self, resource: &dyn resource::AbstractReference) -> bool {
	// 	self.0.uses(resource) || self.1.uses(resource)
	// }
}

unsafe impl<'a, T: Payload> Payload for &'a T {
	// fn uses(&self, resource: &dyn resource::AbstractReference) -> bool {
	// 	(*self).uses(resource)
	// }
}

// pub struct SinglePayload<T: resource::AbstractReference> {
// 	resource: T
// }

// unsafe impl<T: resource::AbstractReference> Payload for SinglePayload<T> {
// 	// fn uses(&self, other: &dyn resource::AbstractReference) -> bool {
// 	// 	resource::aliases(&self.resource, other)
// 	// }
// }

// impl<T: resource::AbstractReference> std::ops::Deref for SinglePayload<T> {
// 	type Target = T;

// 	fn deref(&self) -> &T {
// 		&self.resource
// 	}
// }

// impl<T: resource::AbstractReference> std::ops::DerefMut for SinglePayload<T> {
// 	fn deref_mut(&mut self) -> &mut T {
// 		&mut self.resource
// 	}
// }

pub unsafe trait Task: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error: std::error::Error;

	/// Task payload.
	/// 
	/// Stores all the data borrowed by the task that will be
	/// released upon task completion.
	type Payload: Payload;

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
	type Payload: Payload;

	/// Execute the task after the given future (if any) has signaled its semaphores.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute<P: future::SignalSemaphores>(
		self,
		past: Option<&P>,
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
		Wait::execute::<semaphore::Future<(), semaphore::Raw>>(self, None, signal_semaphore, signal_fence)
	}
}

/// Task that *can* wait on a semaphore.
pub unsafe trait WaitPipelineStages: Sized {
	/// The output value of the task, returned when the task starts.
	type Output;

	/// Error that may be raised by the task when it starts.
	type Error: std::error::Error;

	type Payload: Payload;

	/// Execute the task.
	/// 
	/// Note that `signal_semaphore` and `signal_fence` may not be used.
	fn execute<P: future::SignalSemaphores>(
		self,
		past: Option<&P>,
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
	fn execute<P: future::SignalSemaphores>(
		self,
		past: Option<&P>,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(Self::Output, Self::Payload), Self::Error> {
		WaitPipelineStages::execute(self, past, None, signal_semaphore, signal_fence)
	}
}

/// A task that *can* signal a semaphore upon completion.
pub trait SignalSemaphore: Task {}

/// A task that *can* signal a fence upon completion.
pub trait SignalFence: Task {}