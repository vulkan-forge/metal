use std::fmt;
use ash::vk;

#[repr(transparent)]
pub struct Scissor(vk::Rect2D);

impl Scissor {
	pub fn new(
		x: i32,
		y: i32,
		width: u32,
		height: u32,
	) -> Scissor {
		Scissor(vk::Rect2D {
			offset: vk::Offset2D {
				x, y
			},
			extent: vk::Extent2D {
				width, height
			}
		})
	}

	#[inline]
	pub fn x(&self) -> i32 {
		self.0.offset.x
	}

	#[inline]
	pub fn y(&self) -> i32 {
		self.0.offset.y
	}
	
	#[inline]
	pub fn width(&self) -> u32 {
		self.0.extent.width
	}

	#[inline]
	pub fn height(&self) -> u32 {
		self.0.extent.height
	}
}

impl Default for Scissor {
	fn default() -> Self {
		Self::new(0, 0, 0, 0)
	}
}

impl fmt::Display for Scissor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(x={}, y={}, width={}, height={})", self.x(), self.y(), self.width(), self.height())
	}
}