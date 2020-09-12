use std::{
	sync::Arc,
	error::Error,
	fmt
};
use ash::{
	vk,
	extensions::khr
};
use crate::{
	OomError,
	Instance,
	instance::{
		Extension,
		MissingExtensionError,
		QueueFamily
	}
};

#[derive(Debug)]
pub enum CreationError {
	MissingExtension(Extension),
	OutOfMemory(OomError)
}

impl Error for CreationError {
	fn source(&self) -> Option<&(dyn 'static + Error)> {
		match self {
			CreationError::OutOfMemory(oom) => Some(oom),
			_ => None
		}
	}
}

impl fmt::Display for CreationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CreationError::MissingExtension(ext) =>  write!(f, "missing instance extension `{}`", ext),
			CreationError::OutOfMemory(oom) =>  oom.fmt(f)
		}
	}
}

impl From<MissingExtensionError> for CreationError {
	fn from(MissingExtensionError(ext): MissingExtensionError) -> CreationError {
		CreationError::MissingExtension(ext)
	}
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => panic!("invalid surface creation error")
		}
	}
}

#[derive(Debug)]
pub enum CapabilitiesError {
	/// Not enough memory.
	OutOfMemory(OomError),

	/// The surface has been lost and must be recreated.
	SurfaceLost,

	/// An extension is missing.
	MissingExtension(Extension),
}

impl Error for CapabilitiesError {
	fn source(&self) -> Option<&(dyn 'static + Error)> {
		match self {
			CapabilitiesError::OutOfMemory(oom) => Some(oom),
			_ => None
		}
	}
}

impl fmt::Display for CapabilitiesError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CapabilitiesError::OutOfMemory(oom) =>  oom.fmt(f),
			CapabilitiesError::SurfaceLost => write!(f, "surface is no longer accessible"),
			CapabilitiesError::MissingExtension(ext) =>  write!(f, "missing instance extension `{}`", ext)
		}
	}
}

impl From<vk::Result> for CapabilitiesError {
	fn from(r: vk::Result) -> CapabilitiesError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CapabilitiesError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CapabilitiesError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_SURFACE_LOST_KHR => CapabilitiesError::SurfaceLost,
			_ => panic!("invalid surface capabilities error")
		}
	}
}

impl From<MissingExtensionError> for CapabilitiesError {
	fn from(MissingExtensionError(ext): MissingExtensionError) -> CapabilitiesError {
		CapabilitiesError::MissingExtension(ext)
	}
}

// pub unsafe trait Surface {
// 	pub fn handle(&self) -> vk::SurfaceKHR;
// }

pub struct Surface<W> {
	instance: Arc<Instance>,
	handle: vk::SurfaceKHR,
	backend: W
}

impl<W> Surface<W> {
	/// Create a surface from a wayland surface.
	///
	/// ## Safety
	///
	/// `display` must be a valid wayland display handle,
	/// and `surface` must be a valid wayland surface handle for this display connection.
	/// Both `display` and `surface` must not be freed before `backend`.
	pub unsafe fn from_wayland<D, S>(instance: &Arc<Instance>, display: *const D, surface: *const S, backend: W) -> Result<Surface<W>, CreationError> {
		let infos = vk::WaylandSurfaceCreateInfoKHR {
			display: display as *mut _,
			surface: surface as *mut _,
			..Default::default()
		};

		let handle = instance.ext_khr_wayland_surface()?.create_wayland_surface(&infos, None)?;

		Ok(Surface {
			instance: instance.clone(),
			handle,
			backend
		})
	}

	/// Queue family supports presentation on the given surface.
	///
	/// The `KHR_Surface` extension must be enabled or a missing extension error is returned.
	#[inline]
	pub fn is_supported(&self, queue_family: QueueFamily) -> Result<bool, CapabilitiesError> {
		unsafe {
			Ok(queue_family.physical_device().instance().ext_khr_surface()?.get_physical_device_surface_support(
				queue_family.physical_device().handle(),
				queue_family.index(),
				self.handle
			)?)
		}
	}
}
