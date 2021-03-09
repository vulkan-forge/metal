use super::*;

pub struct Map<T, F> {
	task: T,
	f: F
}

impl<T, F> Map<T, F> {
	pub(crate) fn new(task: T, f: F) -> Self {
		Self {
			task, f
		}
	}
}

unsafe impl<T: Task, U, F> Task for Map<T, F> where F: FnOnce(T::Output) -> U {
	type Output = U;
	type Error = T::Error;
	type Payload = T::Payload;

	fn execute(self, signal_semaphore: Option<&[vk::Semaphore]>, signal_fence: Option<vk::Fence>) -> Result<(U, Self::Payload), Self::Error> {
		let (output, payload) = self.task.execute(signal_semaphore, signal_fence)?;
		Ok(((self.f)(output), payload))
	}
}

impl<T: SignalSemaphore, U, F> SignalSemaphore for Map<T, F> where F: FnOnce(T::Output) -> U {} 
impl<T: SignalFence, U, F> SignalFence for Map<T, F> where F: FnOnce(T::Output) -> U {} 