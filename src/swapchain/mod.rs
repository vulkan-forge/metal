use ash::vk;
use std::sync::Arc;
use crate::{
	OomError,
	Device,
	device,
	Format,
	image::Usage,
	sync::SharingMode
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

pub struct Swapchain<W> {
	device: Arc<Device>,
	surface: Arc<Surface<W>>,
	handle: vk::SwapchainKHR
}

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

impl<W> Swapchain<W> {
	pub fn new(
		device: Arc<Device>,
		surface: Arc<Surface<W>>,
		num_images: u32,
		format: Format,
		color_space: ColorSpace,
		dimensions: Option<(u32, u32)>,
		layers: u32,
		usage: Usage,
		sharing_mode: SharingMode,
		transform: SurfaceTransform,
		alpha: CompositeAlpha,
		mode: PresentMode,
		clipped: bool,
		old_swapchain: Option<&Swapchain<W>>
	) -> Result<(Swapchain<W>, Vec<Image>), CreationError> {
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

		let (sh_mode, sh_count, sh_indices) = match sharing_mode {
			SharingMode::Exclusive => (vk::SharingMode::EXCLUSIVE, 0, std::ptr::null()),
			SharingMode::Concurrent(ref ids) => {
				(vk::SharingMode::CONCURRENT, ids.len() as u32, ids.as_ptr())
			}
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

		let image_handles = unsafe {
			ext_khr_swapchain.get_swapchain_images(handle)?
		};

		let images: Vec<Image> = image_handles.into_iter().map(|handle| {
			Image::new(handle)
		}).collect();

		let swapchain = Swapchain {
			device: device.clone(),
			surface: surface.clone(),
			handle
		};

		Ok((swapchain, images))
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::SwapchainKHR {
		self.handle
	}
}