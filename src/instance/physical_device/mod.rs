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

mod limits;
mod memory_type;
mod queue_family;

pub use limits::Limits;
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
		let this = *self;
		self.p.queue_family_properties.iter().enumerate().map(move |(i, qf)| {
			QueueFamily::new(this, i as u32, qf)
		})
	}

	#[inline]
	pub fn queue_family(&self, id: u32) -> Option<QueueFamily<'a>> {
		let this = *self;
		self.p.queue_family_properties.get(id as usize).map(move |qf| QueueFamily::new(this, id, qf))
	}

	#[inline]
	pub fn supported_features(&self) -> &device::Features {
		&self.p.supported_features
	}

	#[inline]
	pub fn memory_type_count(&self) -> u32 {
		self.p.memory_properties.memory_type_count
	}

	#[inline]
	pub fn memory_type(&self, index: u32) -> Option<MemoryType<'a>> {
		if index < self.memory_type_count() {
			Some(MemoryType::new(*self, index))
		} else {
			None
		}
	}

	#[inline]
	pub fn memory_types(&self) -> impl 'a + Iterator<Item=MemoryType<'a>> {
		let this = *self;
		let len = self.memory_type_count();
		(0u32..len).into_iter().map(move |i| MemoryType::new(this, i))
	}

	pub fn limits(&self) -> Limits<'a> {
		Limits::from_vk_limits(&self.p.properties.limits)
	}
}
