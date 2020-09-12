use std::sync::Arc;
use once_cell::sync::OnceCell;
use ash::{
	vk,
	version::{
		EntryV1_0,
		InstanceV1_0
	}
};
use crate::{
	OomError,
	Entry,
	device
};

pub mod layer;
pub mod extension;
pub mod physical_device;
pub mod queue_family;

pub use layer::{
	ValidationLayer,
	InstanceValidationLayer
};
pub use extension::{
	Extension,
	Extensions
};
pub use physical_device::PhysicalDevice;
pub use queue_family::QueueFamily;

#[derive(Debug)]
pub enum CreationError {
	LoadError(Vec<&'static str>),
	OutOfMemory(OomError),
	InitializationFailed,
	MissingValidationLayer(ValidationLayer),
	MissingExtension(Extension),
	IncompatibleDriver
}

impl From<ash::InstanceError> for CreationError {
	fn from(e: ash::InstanceError) -> CreationError {
		match e {
			ash::InstanceError::LoadError(v) => CreationError::LoadError(v),
			ash::InstanceError::VkError(r) => r.into()
		}
	}
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INITIALIZATION_FAILED => CreationError::InitializationFailed,
			vk::Result::ERROR_LAYER_NOT_PRESENT => panic!("unchecked missing layer"),
			vk::Result::ERROR_EXTENSION_NOT_PRESENT => panic!("unchecked missing extension"),
			vk::Result::ERROR_INCOMPATIBLE_DRIVER => CreationError::IncompatibleDriver,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub struct MissingExtensionError(pub Extension);

pub struct Instance {
	entry: Arc<Entry>,
	pub(crate) handle: ash::Instance,
	loaded_extensions: Extensions,
	physical_devices_info: Vec<PhysicalDeviceInfo>,
	ext_khr_surface: OnceCell<ash::extensions::khr::Surface>,
	ext_khr_wayland_surface: OnceCell<ash::extensions::khr::WaylandSurface>
}

impl Instance {
	/// Create a new instance.
	pub fn new<E: IntoIterator<Item=Extension>>(entry: Arc<Entry>, required_extensions: E) -> Result<Instance, CreationError> {
		unsafe {
			let available_extensions = entry.extensions();

			let mut loaded_extensions = Extensions::none();
			let mut extension_names = Vec::new();
			for ext in required_extensions {
				if !available_extensions.contains(ext) {
					return Err(CreationError::MissingExtension(ext))
				}

				loaded_extensions.insert(ext);
				extension_names.push(ext.c_name().as_ptr())
			}

			// let validation_layers = [
			// 	#[cfg(debug_assertions)]
			// 	ValidationLayer::KhronosValidation
			// ];
			// let mut layer_names = Vec::new();
			// for ext in &validation_layers {
			// 	layer_names.push(ext.c_name().as_ptr())
			// }

			let app_info = vk::ApplicationInfo {
				api_version: vk::make_version(1, 0, 0),
				..Default::default()
			};

			let infos = vk::InstanceCreateInfo {
				p_application_info: &app_info,
				enabled_extension_count: extension_names.len() as u32,
				pp_enabled_extension_names: extension_names.as_ptr(),
				// enabled_layer_count: layer_names.len() as u32,
				// pp_enabled_layer_names: layer_names.as_ptr(),
				..Default::default()
			};

			let handle = entry.handle.create_instance(&infos, None)?;

			let physical_devices_info: Vec<_> = handle.enumerate_physical_devices().unwrap().into_iter().map(|pd| {
				let properties = handle.get_physical_device_properties(pd);
				let supported_features = handle.get_physical_device_features(pd).into();
				PhysicalDeviceInfo {
					handle: pd,
					properties,
					supported_features
				}
			}).collect();

			let instance = Instance {
				entry,
				handle,
				loaded_extensions,
				physical_devices_info,
				ext_khr_surface: OnceCell::new(),
				ext_khr_wayland_surface: OnceCell::new()
			};

			Ok(instance)
		}
	}

	#[inline]
	pub fn entry(&self) -> &Arc<Entry> {
		&self.entry
	}

	/// Get the list of physical devices.
	#[inline]
	pub fn physical_devices<'a>(self: &'a Arc<Self>) -> impl 'a + Iterator<Item=PhysicalDevice<'a>> {
		self.physical_devices_info.iter().map(move |info| {
			PhysicalDevice::new(self, info)
		})
	}

	#[inline]
	pub fn loaded_extensions(&self) -> &Extensions {
		&self.loaded_extensions
	}

	pub fn ext_khr_surface(&self) -> Result<&ash::extensions::khr::Surface, MissingExtensionError> {
		self.ext_khr_surface.get_or_try_init(|| {
			if self.loaded_extensions.khr_surface {
				Ok(ash::extensions::khr::Surface::new(&self.entry.handle, &self.handle))
			} else {
				Err(MissingExtensionError(Extension::KhrSurface))
			}
		})
	}

	pub fn ext_khr_wayland_surface(&self) -> Result<&ash::extensions::khr::WaylandSurface, MissingExtensionError> {
		self.ext_khr_wayland_surface.get_or_try_init(|| {
			if self.loaded_extensions.khr_wayland_surface {
				Ok(ash::extensions::khr::WaylandSurface::new(&self.entry.handle, &self.handle))
			} else {
				Err(MissingExtensionError(Extension::KhrWaylandSurface))
			}
		})
	}
}

impl Drop for Instance {
	fn drop(&mut self) {
		unsafe {
			self.handle.destroy_instance(None)
		}
	}
}

pub(crate) struct PhysicalDeviceInfo {
	handle: vk::PhysicalDevice,
	properties: vk::PhysicalDeviceProperties,
	supported_features: device::Features
}
