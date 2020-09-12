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
	QueueFamily,
	PhysicalDeviceInfo
};

#[derive(Clone, Copy)]
pub struct PhysicalDevice<'a> {
	instance: &'a Arc<Instance>,
	p: &'a PhysicalDeviceInfo
}

impl<'a> PhysicalDevice<'a> {
	#[inline]
	pub(crate) fn new(instance: &'a Arc<Instance>, info: &'a PhysicalDeviceInfo) -> PhysicalDevice<'a> {
		PhysicalDevice {
			instance,
			p: info
		}
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
}
