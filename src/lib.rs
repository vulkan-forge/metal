#[macro_use]
extern crate log;

use std::{
	error::Error,
	ffi::CStr,
	fmt
};
use once_cell::sync::OnceCell;
use ash::version::EntryV1_0;

#[macro_use]
mod set;
pub mod ops;
pub mod sync;
pub mod instance;
pub mod device;
pub mod buffer;
pub mod swapchain;
pub mod alloc;
pub mod format;
pub mod image;
pub mod pipeline;
pub mod framebuffer;

#[cfg(feature = "winit")]
pub mod win;

use instance::{
	layer::InstanceValidationLayer
};

pub use instance::Instance;
pub use device::{
	Device,
	DeviceOwned
};
pub use format::Format;
pub use swapchain::Swapchain;

pub struct Entry {
	handle: ash::Entry,
	extensions: OnceCell<instance::Extensions>
}

impl Entry {
	pub fn new() -> Result<Entry, ash::LoadingError> {
		Ok(Entry {
			handle: ash::Entry::new()?,
			extensions: OnceCell::new()
		})
	}

	pub fn validation_layers<'a>(&'a self) -> impl 'a + Iterator<Item=InstanceValidationLayer<'a>> {
		self.handle.enumerate_instance_layer_properties().unwrap().into_iter().map(move |props| {
			InstanceValidationLayer::new(self, props)
		})
	}

	pub fn extensions(&self) -> &instance::Extensions {
		self.extensions.get_or_init(|| unsafe {
			let mut extensions = instance::Extensions::none();
			for ext_prop in self.handle.enumerate_instance_extension_properties().unwrap() {
				let c_name = CStr::from_ptr(ext_prop.extension_name.as_ptr());
				match instance::Extension::from_c_name(c_name) {
					Some(ext) => extensions.insert(ext),
					None => {
						let name = c_name.to_str().expect("instance extension name is not UTF-8 encoded");
						warn!("unknown instance extension `{}`", name)
					}
				}
			}

			extensions
		})
	}
}

/// Out of memory error.
#[derive(Debug)]
pub enum OomError {
	/// Host is out of memory.
	Host,

	/// Device is out of memory.
	Device
}

impl Error for OomError { }

impl fmt::Display for OomError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			OomError::Host => write!(f, "host is out of memory"),
			OomError::Device => write!(f, "device is out of memory")
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Unbuildable(());
