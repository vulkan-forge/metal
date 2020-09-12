use std::{
	sync::Arc,
	fmt
};
use ash::{
	vk,
	version::{
		InstanceV1_0,
		DeviceV1_0
	}
};
use crate::{
	OomError,
	Instance,
	instance::{
		PhysicalDevice,
		QueueFamily
	}
};

pub mod extension;
pub mod feature;
pub mod queue;

pub use extension::{
	Extension,
	Extensions
};
pub use feature::{
	Feature,
	Features
};
use feature::IntoFFiFeatures;
pub use queue::Queue;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError),
	InvalidQueuePriority(f32),
	InitializationFailed,
	MissingExtension(Extension),
	MissingFeature(Feature),
	TooManyObjets,
	DeviceLost
}

impl std::error::Error for CreationError { }

impl fmt::Display for CreationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use CreationError::*;

		match self {
			OutOfMemory(OomError::Host) => write!(f, "out of host memory"),
			OutOfMemory(OomError::Device) => write!(f, "out of device memory"),
			InvalidQueuePriority(p) => write!(f, "invalid queue priority `{}`", p),
			InitializationFailed => write!(f, "device initialization failed"),
			MissingExtension(e) => write!(f, "missing device extension `{}`", e),
			MissingFeature(t) => write!(f, "missing device feature `{}`", t),
			TooManyObjets => write!(f, "too many objets"),
			DeviceLost => write!(f, "device lost")
		}
	}
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INITIALIZATION_FAILED => CreationError::InitializationFailed,
			vk::Result::ERROR_EXTENSION_NOT_PRESENT => panic!("unchecked missing extension"),
			vk::Result::ERROR_FEATURE_NOT_PRESENT => panic!("unchecked missing feature"),
			vk::Result::ERROR_TOO_MANY_OBJECTS => CreationError::TooManyObjets,
			vk::Result::ERROR_DEVICE_LOST => CreationError::DeviceLost,
			_ => unreachable!()
		}
	}
}

pub struct Device {
	pub(crate) handle: ash::Device,
	instance: Arc<Instance>,
	loaded_extensions: Extensions
}

impl Device {
	pub fn new<'a, E: IntoIterator<Item=Extension>, Q: IntoIterator<Item=(QueueFamily<'a>, f32)>>(physical_device: PhysicalDevice, features: &Features, required_extensions: E, requested_queues: Q) -> Result<(Arc<Device>, Queues), CreationError> {
		let instance = physical_device.instance();

		let mut requested_queues_by_family = Vec::new();
		let mut queues_index_iter = Vec::new();

		for (queue_family, priority) in requested_queues {
			if requested_queues_by_family.len() <= queue_family.index() as usize {
				requested_queues_by_family.resize(queue_family.index() as usize + 1usize, Vec::new());
			}

			if priority < 0.0 || priority > 1.0 {
				return Err(CreationError::InvalidQueuePriority(priority))
			}

			let mut family_requested_queues = &mut requested_queues_by_family[queue_family.index() as usize];

			queues_index_iter.push((queue_family.index(), family_requested_queues.len() as u32));
			family_requested_queues.push(priority);
		}

		let mut queue_create_infos: Vec<_> = requested_queues_by_family.iter().enumerate().filter_map(|(queue_family_index, priorities)| {
			if priorities.is_empty() {
				None
			} else {
				Some(vk::DeviceQueueCreateInfo {
					queue_family_index: queue_family_index as u32,
					queue_count: priorities.len() as u32,
					p_queue_priorities: priorities.as_ptr(),
					..Default::default()
				})
			}
		}).collect();

		let mut loaded_extensions = Extensions::none();
		let mut extension_names = Vec::new();
		for ext in required_extensions {
			loaded_extensions.insert(ext);
			extension_names.push(ext.c_name().as_ptr())
		}

		let ffi_features = features.into_ffi();

		let infos = vk::DeviceCreateInfo {
			queue_create_info_count: queue_create_infos.len() as u32,
			p_queue_create_infos: queue_create_infos.as_ptr(),
			enabled_extension_count: extension_names.len() as u32,
			pp_enabled_extension_names: extension_names.as_ptr(),
			p_enabled_features: &ffi_features as *const vk::PhysicalDeviceFeatures,
			..Default::default()
		};

		let handle = unsafe {
			instance.handle.create_device(physical_device.handle(), &infos, None)?
		};

		let device = Arc::new(Device {
			handle,
			instance: instance.clone(),
			loaded_extensions
		});

		let queues = Queues {
			device: device.clone(),
			index_iter: queues_index_iter.into_iter()
		};

		Ok((device, queues))
	}
}

pub struct Queues {
	device: Arc<Device>,
	index_iter: std::vec::IntoIter<(u32, u32)>
}

impl Iterator for Queues {
	type Item = Arc<Queue>;

	fn next(&mut self) -> Option<Arc<Queue>> {
		match self.index_iter.next() {
			Some((queue_family_index, queue_index)) => {
				let handle = unsafe {
					self.device.handle.get_device_queue(queue_family_index, queue_index)
				};

				let queue = Queue::new(&self.device, handle);
				Some(Arc::new(queue))
			},
			None => None
		}
	}
}
