use ash::{
	vk,
};
use crate::{
	Format,
	DeviceOwned,
	resource
};

mod usage;
mod layout;
mod unbound;
mod bound;
pub mod view;

pub use usage::Usage;
pub use layout::Layout;
pub use unbound::*;
pub use bound::*;
pub use view::{
	View,
	LocalViews,
	Views,
	SyncViews
};

pub use crate::framebuffer::SampleCount;

pub unsafe trait Image: resource::Reference<Handle=vk::Image> + DeviceOwned {
	fn into_view(
		self,
		ty: view::Type,
		format: Format,
		components: view::ComponentMapping,
		subresource_range: view::SubresourceRange
	) -> Result<view::Raw<Self>, view::CreationError> where Self: Sized {
		view::Raw::new(
			self,
			ty,
			format,
			components,
			subresource_range
		)
	}

	fn viewed(
		&self,
		ty: view::Type,
		format: Format,
		components: view::ComponentMapping,
		subresource_range: view::SubresourceRange
	) -> Result<view::Raw<&Self>, view::CreationError> {
		view::Raw::new(
			self,
			ty,
			format,
			components,
			subresource_range
		)
	}

	// requires #![feature(arbitrary_self_types)]
	// fn view<I>(
	// 	self: &I,
	// 	ty: view::Type,
	// 	format: Format,
	// 	components: view::ComponentMapping,
	// 	subresource_range: view::SubresourceRange
	// ) -> Result<View<I>, view::CreationError> where I: Deref<Target=Self> + Clone {
	// 	View::new(
	// 		self.clone(),
	// 		ty,
	// 		format,
	// 		components,
	// 		subresource_range
	// 	)
	// }
}

unsafe impl<'a, T: ?Sized + Image> Image for &'a T {
	// ...
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Type {
	D1 = vk::ImageType::TYPE_1D.as_raw(),
	D2 = vk::ImageType::TYPE_2D.as_raw(),
	D3 = vk::ImageType::TYPE_3D.as_raw(),
}

impl Type {
	pub(crate) fn into_vulkan(self) -> vk::ImageType {
		vk::ImageType::from_raw(self as i32)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Tiling {
	Linear = vk::ImageTiling::LINEAR.as_raw(),
	Optimal = vk::ImageTiling::OPTIMAL.as_raw()
}

impl Tiling {
	pub(crate) fn into_vulkan(self) -> vk::ImageTiling {
		vk::ImageTiling::from_raw(self as i32)
	}

	pub fn is_linear(&self) -> bool {
		match self {
			Self::Linear => true,
			_ => false
		}
	}
}
