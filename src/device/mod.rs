use once_cell::sync::OnceCell;
use ash::{
	vk,
	version::{
		InstanceV1_0,
		DeviceV1_0
	}
};
use std::{
	sync::Arc,
	fmt,
	hash::{
		Hash,
		Hasher
	}
};
use crate::{
	OomError,
	Instance,
	instance::{
		PhysicalDevice,
		physical_device::{
			QueueFamily,
			MemoryType
		}
	}
};

pub mod extension;
pub mod feature;
pub mod queue;
pub mod memory;

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
pub use memory::{
	Memory,
	MappedMemory
};

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

#[derive(Debug)]
pub enum AllocationError {
	OutOfMemory(OomError),
	InvalidExternalHandle,
	InvalidOpaqueCaptureAddress
}

impl From<vk::Result> for AllocationError {
	fn from(r: vk::Result) -> AllocationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => AllocationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => AllocationError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INVALID_EXTERNAL_HANDLE => AllocationError::InvalidExternalHandle,
			vk::Result::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR => AllocationError::InvalidOpaqueCaptureAddress,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub struct MissingExtensionError(pub Extension);

pub struct Device {
	pub(crate) handle: ash::Device,
	instance: Arc<Instance>,
	physical_device_index: u32,
	loaded_extensions: Extensions,
	ext_khr_swapchain: OnceCell<ash::extensions::khr::Swapchain>
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
			physical_device_index: physical_device.index(),
			loaded_extensions,
			ext_khr_swapchain: OnceCell::new()
		});

		let queues = Queues {
			device: device.clone(),
			index_iter: queues_index_iter.into_iter()
		};

		Ok((device, queues))
	}

	pub fn handle(&self) -> &ash::Device {
		&self.handle
	}

	#[inline]
	pub fn physical_device(&self) -> PhysicalDevice {
		PhysicalDevice::new(&self.instance, self.physical_device_index)
	}

	/// Allocate some device memory.
	pub fn allocate_memory(self: &Arc<Self>, memory_type: MemoryType, size: u64) -> Result<Memory, AllocationError> {
		let infos = vk::MemoryAllocateInfo {
			allocation_size: size,
			memory_type_index: memory_type.index(),
			..Default::default()
		};

		let handle = unsafe {
			self.handle.allocate_memory(&infos, None)?
		};

		Ok(Memory::new(self, memory_type, size, handle))
	}

	pub fn ext_khr_swapchain(&self) -> Result<&ash::extensions::khr::Swapchain, MissingExtensionError> {
		self.ext_khr_swapchain.get_or_try_init(|| {
			if self.loaded_extensions.khr_swapchain {
				Ok(ash::extensions::khr::Swapchain::new(&self.instance.handle, &self.handle))
			} else {
				Err(MissingExtensionError(Extension::KhrSwapchain))
			}
		})
	}
}

impl PartialEq for Device {
	fn eq(&self, other: &Device) -> bool {
		self as *const _ == other as *const _
	}
}

impl Eq for Device {}

impl Hash for Device {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self as *const Self).hash(h)
	}
}

impl fmt::Debug for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Device({:?})", self.handle.handle())
	}
}

pub trait DeviceOwned {
	fn device(&self) -> &Arc<Device>;
}

impl<'a, T: ?Sized + DeviceOwned> DeviceOwned for &'a T {
	fn device(&self) -> &Arc<Device> {
		(*self).device()
	}
}

pub struct Queues {
	device: Arc<Device>,
	index_iter: std::vec::IntoIter<(u32, u32)>
}

impl DeviceOwned for Queues {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

impl Iterator for Queues {
	type Item = Queue;

	fn next(&mut self) -> Option<Queue> {
		match self.index_iter.next() {
			Some((queue_family_index, queue_index)) => {
				let handle = unsafe {
					self.device.handle.get_device_queue(queue_family_index, queue_index)
				};

				let queue = Queue::new(&self.device, handle, queue_family_index, queue_index);
				Some(queue)
			},
			None => None
		}
	}
}
