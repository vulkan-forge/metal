use ash::vk;

pub struct Viewport(vk::Viewport); // This MUST be homomorphic with `vk::Viewport`

impl Viewport {
	pub fn new(
		x: f32,
		y: f32,
		width: f32,
		height: f32,
		min_depth: f32,
		max_depth: f32
	) -> Viewport {
		Viewport(vk::Viewport {
			x, y, width, height, min_depth, max_depth
		})
	}

	#[inline]
	pub fn x(&self) -> f32 {
		self.0.x
	}

	#[inline]
	pub fn y(&self) -> f32 {
		self.0.y
	}
	
	#[inline]
	pub fn width(&self) -> f32 {
		self.0.width
	}

	#[inline]
	pub fn height(&self) -> f32 {
		self.0.height
	}

	#[inline]
	pub fn min_depth(&self) -> f32 {
		self.0.min_depth
	}

	#[inline]
	pub fn max_depth(&self) -> f32 {
		self.0.max_depth
	}
}

impl Default for Viewport {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
	}
}