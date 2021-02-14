use ash::{
	vk
};

use crate::{
	swapchain::{
		Surface,
		surface
	}
};
use super::PhysicalDevice;

#[derive(Clone, Copy)]
pub struct QueueFamily<'a> {
	physical_device: PhysicalDevice<'a>,
	index: u32,
	props: &'a vk::QueueFamilyProperties
}

impl<'a> QueueFamily<'a> {
	#[inline]
	pub(crate) fn new(physical_device: PhysicalDevice<'a>, index: u32, props: &'a vk::QueueFamilyProperties) -> QueueFamily<'a> {
		QueueFamily {
			physical_device,
			index,
			props
		}
	}

	/// Physical device this queue family is attached to.
	#[inline]
	pub fn physical_device(&self) -> PhysicalDevice<'a> {
		self.physical_device
	}

	/// Index of the queue family in the physical device.
	#[inline]
	pub fn index(&self) -> u32 {
		self.index
	}

	/// Queue family supports graphics operations.
	#[inline]
	pub fn supports_graphics(&self) -> bool {
		self.props.queue_flags.contains(vk::QueueFlags::GRAPHICS)
	}

	/// Queue family supports presentation on the given surface.
	///
	/// The `KHR_Surface` extension must be enabled or a missing extension error is returned.
	#[inline]
	pub fn supports_presentation<W>(&self, surface: &Surface<W>) -> Result<bool, surface::CapabilitiesError> {
		surface.is_supported(*self)
	}

	/// Queue family supports compute operations.
	#[inline]
	pub fn supports_compute(&self) -> bool {
		self.props.queue_flags.contains(vk::QueueFlags::COMPUTE)
	}

	/// Queue family supports transfer operations.
	#[inline]
	pub fn supports_transfer(&self) -> bool {
		self.props.queue_flags.contains(vk::QueueFlags::TRANSFER)
	}

	/// Queue family supports sparse resource memory management operations.
	#[inline]
	pub fn supports_sparse_binding(&self) -> bool {
		self.props.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING)
	}
}
