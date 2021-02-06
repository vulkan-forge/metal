pub struct Capabilities {
	pub min_image_count: u32,
	pub max_image_count: Option<u32>,
	pub current_extent: Option<[u32; 2]>,
	pub min_image_extent: [u32; 2],
	pub max_image_extent: [u32; 2],
	pub max_image_array_layers: u32,
	pub supported_transforms: SupportedSurfaceTransforms,
	pub current_transform: SurfaceTransform,
	pub supported_composite_alpha: SupportedCompositeAlpha,
	pub supported_usage_flags: ImageUsage,
	pub supported_formats: Vec<(Format, ColorSpace)>,
	pub present_modes: SupportedPresentModes,
}