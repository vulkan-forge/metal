use std::sync::Arc;
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	OomError,
	instance::physical_device::QueueFamily,
	command,
	pipeline,
	device,
	DeviceOwned,
	sync::{
		self,
		task,
	}
};
use super::Device;

#[derive(Debug)]
pub enum SubmitError {
	OutOfMemory(OomError),
	DeviceLost
}

impl From<vk::Result> for SubmitError {
	fn from(r: vk::Result) -> SubmitError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => SubmitError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => SubmitError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_DEVICE_LOST => SubmitError::DeviceLost,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub enum PresentError {
	OutOfMemory(OomError),
	DeviceLost,
	MissingDeviceExtension(device::MissingExtensionError),
}

impl From<device::MissingExtensionError> for PresentError {
	fn from(e: device::MissingExtensionError) -> Self {
		PresentError::MissingDeviceExtension(e)
	}
}

impl From<vk::Result> for PresentError {
	fn from(r: vk::Result) -> PresentError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => PresentError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => PresentError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_DEVICE_LOST => PresentError::DeviceLost,
			_ => unreachable!()
		}
	}
}

/// Device queue.
pub struct Queue {
	device: Arc<Device>,
	handle: vk::Queue,
	queue_family_index: u32,
	queue_index: u32
}

impl Queue {
	pub(crate) fn new(device: &Arc<Device>, handle: vk::Queue, queue_family_index: u32, queue_index: u32) -> Queue {
		Queue {
			device: device.clone(),
			handle,
			queue_family_index,
			queue_index
		}
	}

	pub fn token(&self) -> Token {
		Token {
			device: self.device.clone(),
			handle: self.handle,
			queue_family_index: self.queue_family_index,
			queue_index: self.queue_index
		}
	}

	pub fn index(&self) -> u32 {
		self.queue_index
	}

	pub fn family_index(&self) -> u32 {
		self.queue_family_index
	}

	pub fn family(&self) -> QueueFamily {
		self.device.physical_device().queue_family(self.queue_family_index).unwrap()
	}

	pub fn submit<'a, 'r>(&'a mut self, buffer: &'a command::Buffer<'r>) -> Submit<'a, 'r> {
		// TODO check inner buffer queue access.

		Submit {
			queue: self,
			buffer
		}
	}

	pub fn present<'a, W>(&'a mut self, swapchain: &'a crate::Swapchain<W>, index: u32) -> Present<'a, W> {
		Present {
			queue: self,
			swapchain,
			index
		}
	}
}

impl DeviceOwned for Queue {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

pub struct Submit<'a, 'r> {
	queue: &'a mut Queue,
	buffer: &'a command::Buffer<'r>
}

unsafe impl<'a, 'r> task::WaitPipelineStages for Submit<'a, 'r> {
	type Output = ();
	type Error = SubmitError;

	fn execute(
		self,
		wait_semaphores: Option<&[vk::Semaphore]>,
		wait_pipeline_stage_mask: Option<&[pipeline::stage::Flags]>,
		signal_semaphores: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<(), SubmitError> {
		let infos = vk::SubmitInfo {
			wait_semaphore_count: wait_semaphores.map(|s| s.len() as u32).unwrap_or(0),
			p_wait_semaphores: wait_semaphores.map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
			p_wait_dst_stage_mask: wait_pipeline_stage_mask.map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
			
			command_buffer_count: 1,
			p_command_buffers: &self.buffer.handle(),

			signal_semaphore_count: signal_semaphores.map(|s| s.len() as u32).unwrap_or(0),
			p_signal_semaphores: signal_semaphores.map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
			..Default::default()
		};

		unsafe {
			self.queue.device.handle().queue_submit(self.queue.handle, &[infos], signal_fence.unwrap_or(vk::Fence::null()))?;
		}

		Ok(())
	}
}

impl<'a, 'r> task::SignalSemaphore for Submit<'a, 'r> {}
impl<'a, 'r> task::SignalFence for Submit<'a, 'r> {}

pub struct Present<'a, W> {
	queue: &'a mut Queue,
	swapchain: &'a crate::Swapchain<W>,
	index: u32
}

unsafe impl<'a, W> task::Wait for Present<'a, W> {
	type Output = bool;
	type Error = PresentError;

	fn execute(
		self,
		wait_semaphores: Option<&[vk::Semaphore]>,
		_signal_semaphores: Option<&[vk::Semaphore]>,
		_signal_fence: Option<vk::Fence>,
	) -> Result<bool, PresentError> {
		let ext_khr_swapchain = self.queue.device.ext_khr_swapchain()?;

		let mut result = vk::Result::SUCCESS;

		let infos = vk::PresentInfoKHR {
			wait_semaphore_count: wait_semaphores.map(|s| s.len() as u32).unwrap_or(0),
			p_wait_semaphores: wait_semaphores.map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
			swapchain_count: 1,
			p_swapchains: &self.swapchain.handle(),
			p_image_indices: &self.index,
			p_results: &mut result,
			..Default::default()
		};

		let suboptimal = unsafe {
			ext_khr_swapchain.queue_present(self.queue.handle, &infos)?
		};

		if result != vk::Result::SUCCESS {
			return Err(result.into())
		}

		Ok(suboptimal)
	}
}

pub struct Token {
	device: Arc<Device>,
	handle: vk::Queue,
	queue_family_index: u32,
	queue_index: u32
}