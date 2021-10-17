//! Magma is a strictly typed Rust interface for the vulkan API.
//! This means that whenever possible,
//! the well formedness and safety of each vulkan object instance
//! is statically checked using the powerful type system of Rust,
//! using traits to define and verify required properties.
//! However because of the nature of the vulkan API,
//! it is nearly impossible to avoid unsafe trait implementations using Magma alone.
//! It is highly recommended to use Magma in combination with Cast,
//! a pipeline modelization framework that can be used to automatically
//! (and safely) implement unsafe traits.
//! 
//! Magma is highly inspired by vulkano, but closer to the original vulkan API
//! and including more static checks.
//! 
//! ## Safety
//! 
//! Shader module signatures are unsafe.
//! Descriptor set transitions are unsafe.
#![feature(generic_associated_types)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]

#[doc(hidden)]
pub use ::core;

use std::{
	error::Error,
	ffi::CStr,
	fmt
};
use once_cell::sync::OnceCell;
use ash::{
	vk,
	version::EntryV1_0
};

#[macro_use]
mod set;
pub mod resource;
pub mod ops;
pub mod sync;
pub mod instance;
pub mod device;
pub mod swapchain;
pub mod mem;
pub mod format;
pub mod image;
pub mod pipeline;
pub mod framebuffer;
pub mod command;
pub mod descriptor;

#[cfg(feature = "winit")]
pub mod win;

pub use instance::Instance;
pub use device::{
	Device,
	DeviceOwned
};
pub use format::{
	Format,
	FormattedType
};
pub use swapchain::Swapchain;
pub use image::Image;
pub use framebuffer::Framebuffer;
// pub use descriptor::Descriptor;

pub struct Entry {
	handle: ash::Entry,
	extensions: OnceCell<instance::Extensions>,
	layers: OnceCell<instance::ValidationLayers>
}

impl Entry {
	pub fn new() -> Result<Entry, ash::LoadingError> {
		Ok(Entry {
			handle: ash::Entry::new()?,
			extensions: OnceCell::new(),
			layers: OnceCell::new()
		})
	}

	pub fn validation_layers<'a>(&'a self) -> &instance::ValidationLayers {
		self.layers.get_or_init(|| unsafe {
			let mut layers = instance::ValidationLayers::none();
			for layer_prop in self.handle.enumerate_instance_layer_properties().unwrap() {
				let c_name = CStr::from_ptr(layer_prop.layer_name.as_ptr());
				match instance::ValidationLayer::from_c_name(c_name) {
					Some(layer) => {
						log::info!("available validation layer `{}`", layer);
						layers.insert(layer)
					},
					None => {
						let name = c_name.to_str().expect("validation layer name is not UTF-8 encoded");
						log::warn!("unknown validation layer `{}`", name)
					}
				}
			}

			layers
		})
	}

	pub fn extensions(&self) -> &instance::Extensions {
		self.extensions.get_or_init(|| unsafe {
			let mut extensions = instance::Extensions::none();
			for ext_prop in self.handle.enumerate_instance_extension_properties().unwrap() {
				let c_name = CStr::from_ptr(ext_prop.extension_name.as_ptr());
				match instance::Extension::from_c_name(c_name) {
					Some(ext) => {
						log::info!("available instance extension `{}`", ext);
						extensions.insert(ext)
					},
					None => {
						let name = c_name.to_str().expect("instance extension name is not UTF-8 encoded");
						log::warn!("unknown instance extension `{}`", name)
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

impl From<vk::Result> for OomError {
	fn from(r: vk::Result) -> OomError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => OomError::Host,
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => OomError::Device,
			_ => unreachable!()
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Unbuildable(());
