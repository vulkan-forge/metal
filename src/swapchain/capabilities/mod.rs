use crate::{
	Format,
	image
};

#[macro_use]
mod set;
mod surface_transform;
mod composite_alpha;
mod present_mode;
mod color_space;

pub use surface_transform::{
	SurfaceTransform,
	SurfaceTransforms
};
pub use composite_alpha::{
	CompositeAlpha,
	CompositeAlphas
};
pub use present_mode::{
	PresentMode,
	PresentModes
};
pub use color_space::ColorSpace;

pub struct Capabilities {
	pub min_image_count: u32,
	pub max_image_count: Option<u32>,
	pub current_extent: Option<[u32; 2]>,
	pub min_image_extent: [u32; 2],
	pub max_image_extent: [u32; 2],
	pub max_image_array_layers: u32,
	pub supported_transforms: SurfaceTransforms,
	pub current_transform: SurfaceTransform,
	pub supported_composite_alpha: CompositeAlphas,
	pub supported_usage_flags: image::Usage,
	pub supported_formats: Vec<(Format, ColorSpace)>,
	pub present_modes: PresentModes,
}