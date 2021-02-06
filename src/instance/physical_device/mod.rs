use std::{
	sync::Arc,
	ffi::CStr
};
use ash::{
	vk,
	version::InstanceV1_0
};

use crate::device;
use super::{
	Instance,
	PhysicalDeviceInfo
};

mod memory_type;
mod queue_family;

pub use memory_type::MemoryType;
pub use queue_family::QueueFamily;

#[derive(Clone, Copy)]
pub struct PhysicalDevice<'a> {
	instance: &'a Arc<Instance>,
	index: u32,
	p: &'a PhysicalDeviceInfo
}

impl<'a> PhysicalDevice<'a> {
	#[inline]
	pub(crate) fn new(instance: &'a Arc<Instance>, index: u32) -> PhysicalDevice<'a> {
		let info = &instance.physical_devices_info[index as usize];

		PhysicalDevice {
			instance,
			index,
			p: info
		}
	}

	#[inline]
	pub fn index(&self) -> u32 {
		self.index
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::PhysicalDevice {
		self.p.handle
	}

	#[inline]
	pub fn instance(&self) -> &'a Arc<Instance> {
		self.instance
	}

	#[inline]
	pub fn name(&self) -> &str {
		unsafe {
			let c_name = CStr::from_ptr(self.p.properties.device_name.as_ptr());
			c_name.to_str().expect("physical device name is not UTF-8 encoded")
		}
	}

	#[inline]
	pub fn queue_families(&self) -> impl 'a + Iterator<Item=QueueFamily<'a>> {
		unsafe {
			let this = *self;
			self.instance.handle.get_physical_device_queue_family_properties(self.p.handle).into_iter().enumerate().map(move |(i, qf)| {
				QueueFamily::new(this, i as u32, qf)
			})
		}
	}

	#[inline]
	pub fn supported_features(&self) -> &device::Features {
		&self.p.supported_features
	}

	#[inline]
	pub fn memory_types(&self) -> impl 'a + Iterator<Item=MemoryType<'a>> {
		let this = *self;
		let len = self.p.memory_properties.memory_type_count;
		(0u32..len).into_iter().map(move |i| MemoryType::new(this, i))
	}
}