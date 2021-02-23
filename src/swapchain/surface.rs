use std::{
	os::raw::c_ulong,
	sync::Arc,
	error::Error,
	fmt
};
use ash::{
	vk,
};
use crate::{
	OomError,
	Instance,
	instance::{
		Extension,
		MissingExtensionError,
		PhysicalDevice,
		physical_device::QueueFamily
	},
	swapchain::{
		capabilities,
		Capabilities
	},
	image,
	Format
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

pub struct Surface<W> {
	instance: Arc<Instance>,
	handle: vk::SurfaceKHR,
	backend: W
}

impl<W> Surface<W> {
	/// Creates a `Surface` from an XCB window.
	///
	/// The surface's min, max and current extent will always match the window's dimensions.
	///
	/// # Safety
	///
	/// The caller must ensure that the `connection` and the `window` are both correct and stay
	/// alive for the entire lifetime of the surface. The `win` parameter can be used to ensure this.
	pub unsafe fn from_xcb<C>(
		instance: &Arc<Instance>,
		connection: *const C,
		window: u32,
		backend: W,
	) -> Result<Surface<W>, CreationError> {
		let infos = vk::XcbSurfaceCreateInfoKHR {
			connection: connection as *mut _,
			window: window,
			..Default::default()
		};

		let handle = instance.ext_khr_xcb_surface()?.create_xcb_surface(&infos, None)?;

		Ok(Surface {
			instance: instance.clone(),
			handle,
			backend
		})
	}

	/// Retrieves the capabilities of a surface when used by a certain device.
	///
	/// # Panic
	///
	/// - Panics if the device and the surface don't belong to the same instance.
	///
	pub fn capabilities(&self, device: PhysicalDevice) -> Result<Capabilities, CapabilitiesError> {
		unsafe {
			assert_eq!(
				&*self.instance as *const _,
				&**device.instance() as *const _,
				"Instance mismatch in Surface::capabilities"
			);

			let ext_khr_surface = device.instance().ext_khr_surface()?;

			let caps = ext_khr_surface.get_physical_device_surface_capabilities(
				device.handle(),
				self.handle
			)?;

			let formats = ext_khr_surface.get_physical_device_surface_formats(
				device.handle(),
				self.handle
			)?;

			let modes = ext_khr_surface.get_physical_device_surface_present_modes(
				device.handle(),
				self.handle
			)?;

			// let modes = {
			// 	let mut num = 0;
			// 	check_errors(vk.GetPhysicalDeviceSurfacePresentModesKHR(
			// 		device.internal_object(),
			// 		self.surface,
			// 		&mut num,
			// 		ptr::null_mut(),
			// 	))?;

			// 	let mut modes = Vec::with_capacity(num as usize);
			// 	check_errors(vk.GetPhysicalDeviceSurfacePresentModesKHR(
			// 		device.internal_object(),
			// 		self.surface,
			// 		&mut num,
			// 		modes.as_mut_ptr(),
			// 	))?;
			// 	modes.set_len(num as usize);
			// 	debug_assert!(modes
			// 		.iter()
			// 		.find(|&&m| m == vk::PRESENT_MODE_FIFO_KHR)
			// 		.is_some());
			// 	debug_assert!(modes.iter().count() > 0);
			// 	capabilities::supported_present_modes_from_list(modes.into_iter())
			// };

			Ok(Capabilities {
				min_image_count: caps.min_image_count,
				max_image_count: if caps.max_image_count == 0 {
					None
				} else {
					Some(caps.max_image_count)
				},
				current_extent: {
					if caps.current_extent.width == 0xffffffff && caps.current_extent.height == 0xffffffff {
						None
					} else {
						Some([caps.current_extent.width, caps.current_extent.height])
					}
				},
				min_image_extent: [caps.min_image_extent.width, caps.min_image_extent.height],
				max_image_extent: [caps.max_image_extent.width, caps.max_image_extent.height],
				max_image_array_layers: caps.max_image_array_layers,
				supported_transforms: capabilities::SurfaceTransforms::from_vulkan(caps.supported_transforms),
				current_transform: capabilities::SurfaceTransforms::from_vulkan(caps.current_transform).iter().next().unwrap(),
				supported_composite_alpha: capabilities::CompositeAlphas::from_vulkan(caps.supported_composite_alpha),
				supported_usage_flags: {
					let usage = image::Usage::from_vulkan(caps.supported_usage_flags);
					debug_assert!(usage.color_attachment); // specs say that this must be true
					usage
				},
				supported_formats: formats.into_iter().map(|f| {
					(
						Format::from_vulkan(f.format).unwrap(),
						capabilities::ColorSpace::from_vulkan(f.color_space),
					)
				}).collect(),
				present_modes: capabilities::PresentModes::from_vulkan(modes),
			})
		}
	}
	
	/// Creates a `Surface` from an Xlib window.
	///
	/// The surface's min, max and current extent will always match the window's dimensions.
	///
	/// # Safety
	///
	/// The caller must ensure that the `display` and the `window` are both correct and stay
	/// alive for the entire lifetime of the surface. The `win` parameter can be used to ensure this.
	pub unsafe fn from_xlib<D>(
		instance: &Arc<Instance>,
		display: *const D,
		window: c_ulong,
		backend: W,
	) -> Result<Surface<W>, CreationError> {
		let infos = vk::XlibSurfaceCreateInfoKHR {
			dpy: display as *mut _,
			window: window,
			..Default::default()
		};
		
		let handle = instance.ext_khr_xlib_surface()?.create_xlib_surface(&infos, None)?;

		Ok(Surface {
			instance: instance.clone(),
			handle,
			backend
		})
	}
	
	/// Create a surface from a wayland surface.
	///
	/// ## Safety
	///
	/// `display` must be a valid wayland display handle,
	/// and `surface` must be a valid wayland surface handle for this display connection.
	/// Both `display` and `surface` must not be freed before `backend`.
	pub unsafe fn from_wayland(
		instance: &Arc<Instance>,
		display: *mut std::ffi::c_void,
		surface: *mut std::ffi::c_void,
		backend: W
	) -> Result<Surface<W>, CreationError> {
		let infos = vk::WaylandSurfaceCreateInfoKHR {
			display,
			surface,
			..Default::default()
		};

		let handle = instance.ext_khr_wayland_surface()?.create_wayland_surface(&infos, None)?;

		Ok(Surface {
			instance: instance.clone(),
			handle,
			backend
		})
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::SurfaceKHR {
		self.handle
	}

	#[inline]
	pub fn backend(&self) -> &W {
		&self.backend
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