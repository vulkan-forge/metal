use ash::vk;
use std::{
	rc::Rc,
	sync::Arc,
	fmt
};
use crate::{
	OomError,
	Device,
	DeviceOwned,
	device,
	Format,
	image::Usage,
	sync::{
		self,
		task,
	}
};

pub mod surface;
pub mod capabilities;
mod image;

pub use surface::Surface;
pub use capabilities::Capabilities;
use capabilities::{
	ColorSpace,
	SurfaceTransform,
	CompositeAlpha,
	PresentMode
};
pub use image::Image;

#[derive(Debug)]
pub enum CreationError {
	OomError(OomError),
	DeviceLost,
	SurfaceLost,
	NativeWindownInUse,
	InitializationFailed,
	MissingDeviceExtension(device::MissingExtensionError),
	CapabilitiesError(surface::CapabilitiesError),
	UnsupportedDimensions((u32, u32))
}

impl From<device::MissingExtensionError> for CreationError {
	fn from(e: device::MissingExtensionError) -> Self {
		CreationError::MissingDeviceExtension(e)
	}
}

impl From<surface::CapabilitiesError> for CreationError {
	fn from(e: surface::CapabilitiesError) -> Self {
		CreationError::CapabilitiesError(e)
	}
}

impl From<vk::Result> for CreationError {
	fn from(e: vk::Result) -> Self {
		match e {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OomError(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OomError(OomError::Device),
			vk::Result::ERROR_DEVICE_LOST => CreationError::DeviceLost,
			vk::Result::ERROR_SURFACE_LOST_KHR => CreationError::SurfaceLost,
			vk::Result::ERROR_NATIVE_WINDOW_IN_USE_KHR => CreationError::NativeWindownInUse,
			vk::Result::ERROR_INITIALIZATION_FAILED => CreationError::InitializationFailed,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub enum AcquireError {
	Timeout,
	NotReady,
	OomError(OomError),
	DeviceLost,
	SurfaceLost,
	FullScreenExclusiveModeLost,
	MissingDeviceExtension(device::MissingExtensionError),
	OutOfDate
}

impl From<device::MissingExtensionError> for AcquireError {
	fn from(e: device::MissingExtensionError) -> Self {
		AcquireError::MissingDeviceExtension(e)
	}
}

impl From<vk::Result> for AcquireError {
	fn from(e: vk::Result) -> Self {
		match e {
			vk::Result::TIMEOUT => AcquireError::Timeout,
			vk::Result::NOT_READY => AcquireError::NotReady,
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => AcquireError::OomError(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => AcquireError::OomError(OomError::Device),
			vk::Result::ERROR_DEVICE_LOST => AcquireError::DeviceLost,
			vk::Result::ERROR_OUT_OF_DATE_KHR => AcquireError::OutOfDate,
			vk::Result::ERROR_SURFACE_LOST_KHR => AcquireError::SurfaceLost,
			vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => AcquireError::FullScreenExclusiveModeLost,
			_ => unreachable!()
		}
	}
}

impl std::error::Error for AcquireError {
	// ...
}

impl fmt::Display for AcquireError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Timeout => write!(f, "timeout"),
			Self::NotReady => write!(f, "not ready"),
			Self::OomError(e) => e.fmt(f),
			Self::DeviceLost => write!(f, "device lost"),
			Self::OutOfDate => write!(f, "swapchain out of date"),
			Self::SurfaceLost => write!(f, "surface lost"),
			Self::FullScreenExclusiveModeLost => write!(f, "full screen exclusive mode lost"),
			Self::MissingDeviceExtension(e) => e.fmt(f)
		}
	}
}

pub(crate) struct Inner<W> {
	device: Arc<Device>,
	surface: Arc<Surface<W>>,
	format: Format,
	color_space: ColorSpace,
	handle: vk::SwapchainKHR
}

pub struct Swapchain<W> {
	inner: Rc<Inner<W>>,
}

impl<W> Swapchain<W> {
	pub fn new<'a, S: IntoIterator<Item=&'a device::Queue>>(
		device: &Arc<Device>,
		surface: &Arc<Surface<W>>,
		num_images: u32,
		format: Format,
		color_space: ColorSpace,
		dimensions: Option<(u32, u32)>,
		layers: u32,
		usage: Usage,
		sharing_queues: S,
		transform: SurfaceTransform,
		alpha: CompositeAlpha,
		mode: PresentMode,
		clipped: bool,
		old_swapchain: Option<&Swapchain<W>>
	) -> Result<(Swapchain<W>, Vec<Image<W>>), CreationError> {
		let capabilities = surface.capabilities(device.physical_device())?;

		let dimensions = if let Some(dimensions) = dimensions {
			if dimensions.0 < capabilities.min_image_extent[0] {
				return Err(CreationError::UnsupportedDimensions(dimensions));
			}
			if dimensions.1 < capabilities.min_image_extent[1] {
				return Err(CreationError::UnsupportedDimensions(dimensions));
			}
			if dimensions.0 > capabilities.max_image_extent[0] {
				return Err(CreationError::UnsupportedDimensions(dimensions));
			}
			if dimensions.1 > capabilities.max_image_extent[1] {
				return Err(CreationError::UnsupportedDimensions(dimensions));
			}
			dimensions
		} else {
			let extent = capabilities.current_extent.unwrap();
			(extent[0], extent[1])
		};

		let mut ids: Vec<u32> = sharing_queues.into_iter().map(|q| q.family_index()).collect();
		ids.sort();
		ids.dedup();

		let (sh_mode, sh_count, sh_indices) = if ids.len() > 1 {
			(vk::SharingMode::EXCLUSIVE, 0, std::ptr::null())
		} else {
			(vk::SharingMode::CONCURRENT, ids.len() as u32, ids.as_ptr())
		};

		let infos = vk::SwapchainCreateInfoKHR {
			surface: surface.handle(),
			min_image_count: num_images,
			image_format: format.into_vulkan(),
			image_color_space: color_space.into_vulkan(),
			image_extent: vk::Extent2D {
				width: dimensions.0,
				height: dimensions.1,
			},
			image_array_layers: layers,
			image_usage: usage.to_vulkan(),
			image_sharing_mode: sh_mode,
			queue_family_index_count: sh_count,
			p_queue_family_indices: sh_indices,
			pre_transform: transform.into_vulkan(),
			composite_alpha: alpha.into_vulkan(),
			present_mode: mode.into_vulkan(),
			clipped: if clipped { vk::TRUE } else { vk::FALSE },
			old_swapchain: if let Some(ref old_swapchain) = old_swapchain {
				old_swapchain.handle()
			} else {
				vk::SwapchainKHR::null()
			},
			..Default::default()
		};

		let ext_khr_swapchain = device.ext_khr_swapchain()?;

		let handle = unsafe {
			ext_khr_swapchain.create_swapchain(&infos, None)?
		};

		let inner = Rc::new(Inner {
			device: device.clone(),
			surface: surface.clone(),
			handle,
			format,
			color_space
		});

		let images = unsafe {
			ext_khr_swapchain.get_swapchain_images(handle)?
		}.into_iter().map(|h| Image::new(&inner, h)).collect();

		let swapchain = Swapchain {
			inner
		};

		Ok((swapchain, images))
	}

	pub fn surface(&self) -> &Arc<Surface<W>> {
		&self.inner.surface
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::SwapchainKHR {
		self.inner.handle
	}

	#[inline]
	pub fn format(&self) -> Format {
		self.inner.format
	}

	#[inline]
	pub fn color_space(&self) -> ColorSpace {
		self.inner.color_space
	}

	pub fn acquire_next_image(&mut self, timeout: Option<u64>) -> Acquire<W> {
		Acquire {
			swapchain: self,
			timeout
		}
	}
}

impl<W> DeviceOwned for Swapchain<W> {
	fn device(&self) -> &Arc<Device> {
		&self.inner.device
	}
}

/// `Acquire` payload.
/// 
/// This type contains a reference to the swapchain
/// to ensure that it is not released while acquiring an image.
pub struct Acquiring<W>(Rc<Inner<W>>);

pub struct Acquire<'a, W> {
	swapchain: &'a mut Swapchain<W>,
	timeout: Option<u64>
}

unsafe impl<'a, W> task::Task for Acquire<'a, W> {
	type Output = (u32, bool);
	type Error = AcquireError;
	type Payload = Acquiring<W>;

	fn execute(
		self,
		signal_semaphore: Option<&[vk::Semaphore]>,
		signal_fence: Option<vk::Fence>,
	) -> Result<((u32, bool), Acquiring<W>), AcquireError> {
		let ext_khr_swapchain = self.swapchain.inner.device.ext_khr_swapchain()?;
		let output = unsafe {
				ext_khr_swapchain.acquire_next_image(
				self.swapchain.inner.handle,
				self.timeout.unwrap_or(u64::MAX),
				signal_semaphore.map(|s| *s.first().unwrap()).unwrap_or(vk::Semaphore::null()),
				signal_fence.unwrap_or(vk::Fence::null())
			)?
		};

		Ok((output, Acquiring(self.swapchain.inner.clone())))
	}
}

impl<'a, W> task::SignalSemaphore for Acquire<'a, W> {}
impl<'a, W> task::SignalFence for Acquire<'a, W> {}

// pub struct Images<W> {
// 	inner: Rc<Inner<W>>,
// 	images: &'a [vk::Image]
// }

// impl<'a, W> Images<'a, W> {
// 	pub fn get(&self, i: u32) -> Option<Image<'a, W>> {
// 		match self.images.get(i as usize) {
// 			Some(h) => Some(Image::new(self.swapchain, *h)),
// 			None => None
// 		}
// 	}

// 	pub fn iter(&self) -> ImagesIter<'a, W> {
// 		ImagesIter {
// 			swapchain: self.swapchain,
// 			inner: self.images.iter()
// 		}
// 	}
// }

// pub struct ImagesIter<'a, W> {
// 	swapchain: &'a Swapchain<W>,
// 	inner: std::slice::Iter<'a, vk::Image>
// }

// impl<'a, W> Iterator for ImagesIter<'a, W> {
// 	type Item = Image<'a, W>;

// 	fn next(&mut self) -> Option<Image<'a, W>> {
// 		self.inner.next().map(|h| Image::new(self.swapchain, *h))
// 	}
// }

// pub struct ImageStream<'a, W> {
// 	swapchain: &'a Swapchain<W>
// }

// impl ImageStream<'a, W> {
	
// }