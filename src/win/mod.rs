use winit::{
	window::{
		Window,
		WindowBuilder
	},
	event_loop::EventLoopWindowTarget,
	error::OsError as WindowCreationError
};
use std::{
	sync::Arc,
	fmt,
	error::Error
};
use crate::{
	Entry,
	Instance,
	instance,
	swapchain::{
		Surface,
		surface::CreationError as SurfaceCreationError
	}
};

#[derive(Debug)]
pub enum CreationError {
	SurfaceCreationError(SurfaceCreationError),
	WindowCreationError(WindowCreationError)
}

impl Error for CreationError {
	#[inline]
	fn cause(&self) -> Option<&dyn Error> {
		match *self {
			CreationError::SurfaceCreationError(ref err) => Some(err),
			CreationError::WindowCreationError(ref err) => Some(err),
		}
	}
}

impl fmt::Display for CreationError {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(fmt, "{}", match *self {
			CreationError::SurfaceCreationError(_) => "error while creating the surface",
			CreationError::WindowCreationError(_) => "error while creating the window",
		})
	}
}

impl From<SurfaceCreationError> for CreationError {
	#[inline]
	fn from(err: SurfaceCreationError) -> CreationError {
		CreationError::SurfaceCreationError(err)
	}
}

impl From<WindowCreationError> for CreationError {
	#[inline]
	fn from(err: WindowCreationError) -> CreationError {
		CreationError::WindowCreationError(err)
	}
}

pub trait WindowBuilderExt {
	fn build_vk_surface<E>(self, event_loop: &EventLoopWindowTarget<E>, instance: &Arc<Instance>) -> Result<Surface<Window>, CreationError>;
}

impl WindowBuilderExt for WindowBuilder {
	fn build_vk_surface<E>(self, event_loop: &EventLoopWindowTarget<E>, instance: &Arc<Instance>) -> Result<Surface<Window>, CreationError> {
		let window = self.build(event_loop)?;
		Ok(create_winit_surface(instance, window)?)
	}
}

/// Get the required extensions to enable to create surfaces with `winit`.
pub fn required_extensions(entry: &Entry) -> instance::Extensions {
	let ideal = instance::Extensions {
		khr_surface: true,
		khr_xlib_surface: true,
		khr_xcb_surface: true,
		khr_wayland_surface: true,
		khr_android_surface: true,
		khr_win32_surface: true,
		mvk_ios_surface: true,
		mvk_macos_surface: true,
		khr_get_physical_device_properties2: true,
		khr_get_surface_capabilities2: true,
		..instance::Extensions::none()
	};

	entry.extensions().intersection(&ideal)
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn create_winit_surface(instance: &Arc<Instance>, window: Window) -> Result<Surface<Window>, SurfaceCreationError> {
	unsafe {
		use winit::platform::unix::WindowExtUnix;

		match (window.wayland_display(), window.wayland_surface()) {
			(Some(display), Some(surface)) => {
				Surface::from_wayland(instance, display, surface, window)
			},
			_ => {
				// no wayland display found.
				panic!("Only wayland is supported for now")
			}
		}
	}
}

#[cfg(target_os = "windows")]
pub fn create_winit_surface(instance: &Arc<Instance>, window: Window) -> Result<Surface<Window>, SurfaceCreationError> {
	panic!("Windows is not supported yet")
}

#[cfg(target_os = "macos")]
pub fn create_winit_surface(instance: &Arc<Instance>, window: Window) -> Result<Surface<Window>, SurfaceCreationError> {
	panic!("Macos is not supported yet")
}

#[cfg(target_os = "android")]
pub fn create_winit_surface(instance: &Arc<Instance>, window: Window) -> Result<Surface<Window>, SurfaceCreationError> {
	panic!("Android is not supported yet")
}
