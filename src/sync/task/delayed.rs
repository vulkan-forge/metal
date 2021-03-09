use super::*;

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