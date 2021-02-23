use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	OomError,
	Format
};
use super::{
	Image
};

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Type {
	D1 = vk::ImageViewType::TYPE_1D.as_raw(),
	D2 = vk::ImageViewType::TYPE_2D.as_raw(),
	D = vk::ImageViewType::TYPE_3D.as_raw(),
	Cube = vk::ImageViewType::CUBE.as_raw(),
	D1Array = vk::ImageViewType::TYPE_1D_ARRAY.as_raw(),
	D2Array = vk::ImageViewType::TYPE_2D_ARRAY.as_raw(),
	CubeArray = vk::ImageViewType::CUBE_ARRAY.as_raw()
}

impl Type {
	pub(crate) fn into_vulkan(self) -> vk::ImageViewType {
		vk::ImageViewType::from_raw(self as i32)
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ComponentSwizzle {
	Identity = vk::ComponentSwizzle::IDENTITY.as_raw(),
	Zero = vk::ComponentSwizzle::ZERO.as_raw(),
	One = vk::ComponentSwizzle::ONE.as_raw(),
	Red = vk::ComponentSwizzle::R.as_raw(),
	Green = vk::ComponentSwizzle::G.as_raw(),
	Blue = vk::ComponentSwizzle::B.as_raw(),
	Alpha = vk::ComponentSwizzle::A.as_raw()
}

impl ComponentSwizzle {
	pub(crate) fn into_vulkan(self) -> vk::ComponentSwizzle {
		vk::ComponentSwizzle::from_raw(self as i32)
	}
}

impl Default for ComponentSwizzle {
	fn default() -> Self {
		Self::Identity
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentMapping {
	red: ComponentSwizzle,
	green: ComponentSwizzle,
	blue: ComponentSwizzle,
	alpha: ComponentSwizzle
}

impl ComponentMapping {
	pub fn new(
		red: ComponentSwizzle,
		green: ComponentSwizzle,
		blue: ComponentSwizzle,
		alpha: ComponentSwizzle
	) -> Self {
		Self {
			red, green, blue, alpha
		}
	}

	pub(crate) fn into_vulkan(self) -> vk::ComponentMapping {
		vk::ComponentMapping {
			r: self.red.into_vulkan(),
			g: self.green.into_vulkan(),
			b: self.blue.into_vulkan(),
			a: self.alpha.into_vulkan()
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Aspects {
	color: bool,
	depth: bool,
	stencil: bool,
	metadata: bool
}

impl Aspects {
	pub fn new(
		color: bool,
		depth: bool,
		stencil: bool,
		metadata: bool
	) -> Aspects {
		Self {
			color, depth, stencil, metadata
		}
	}

	pub fn color() -> Self {
		Self::new(true, false, false, false)
	}

	pub(crate) fn into_vulkan(self) -> vk::ImageAspectFlags {
		let mut flags = vk::ImageAspectFlags::empty();

		if self.color {
			flags |= vk::ImageAspectFlags::COLOR
		}

		if self.depth {
			flags |= vk::ImageAspectFlags::DEPTH
		}

		if self.stencil {
			flags |= vk::ImageAspectFlags::STENCIL
		}

		if self.metadata {
			flags |= vk::ImageAspectFlags::METADATA
		}

		flags
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SubresourceRange {
	pub aspects: Aspects,
	pub base_mip_level: u32,
	pub level_count: u32,
	pub base_array_layer: u32,
	pub layer_count: u32
}

impl SubresourceRange {
	pub(crate) fn into_vulkan(self) -> vk::ImageSubresourceRange {
		vk::ImageSubresourceRange {
			aspect_mask: self.aspects.into_vulkan(),
			base_mip_level: self.base_mip_level,
			level_count: self.level_count,
			base_array_layer: self.base_array_layer,
			layer_count: self.layer_count
		}
	}
}

pub struct View<I: Image> {
	image: I,
	handle: vk::ImageView
}

impl<I: Image> View<I> {
	pub fn new(
		image: I,
		ty: Type,
		format: Format,
		components: ComponentMapping,
		subresource_range: SubresourceRange
	) -> Result<View<I>, CreationError> {
		let infos = vk::ImageViewCreateInfo {
			image: image.handle(),
			view_type: ty.into_vulkan(),
			format: format.into_vulkan(),
			components: components.into_vulkan(),
			subresource_range: subresource_range.into_vulkan(),
			..Default::default()
		};

		let handle = unsafe {
			image.device().handle().create_image_view(&infos, None)?
		};

		Ok(View {
			image,
			handle
		})
	}

	pub(crate) fn handle(&self) -> vk::ImageView {
		self.handle
	}
}

impl<I: Image> Drop for View<I> {
	fn drop(&mut self) {
		unsafe {
			self.image.device().handle().destroy_image_view(self.handle, None)
		}
	}
}