use ash::vk;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum PolygonMode {
	Fill = vk::PolygonMode::FILL.as_raw(),
	Line = vk::PolygonMode::LINE.as_raw(),
	Point = vk::PolygonMode::POINT.as_raw()
}

impl PolygonMode {
	pub(crate) fn into_vulkan(self) -> vk::PolygonMode {
		vk::PolygonMode::from_raw(self as i32)
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum CullMode {
	None = vk::CullModeFlags::NONE.as_raw(),
	Front = vk::CullModeFlags::FRONT.as_raw(),
	Back = vk::CullModeFlags::BACK.as_raw(),
	FrontAndBack = vk::CullModeFlags::FRONT_AND_BACK.as_raw()
}

impl CullMode {
	pub(crate) fn into_vulkan(self) -> vk::CullModeFlags {
		vk::CullModeFlags::from_raw(self as u32)
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum FrontFace {
	Clockwise = vk::FrontFace::CLOCKWISE.as_raw(),
	CounterClockwise = vk::FrontFace::COUNTER_CLOCKWISE.as_raw()
}

impl FrontFace {
	pub(crate) fn into_vulkan(self) -> vk::FrontFace {
		vk::FrontFace::from_raw(self as i32)
	}
}


#[derive(Clone, Copy, Debug)]
pub struct DepthBias {
	constant_factor: f32,
	clamp: f32,
	slope_factor: f32
}

pub struct Rasterization(vk::PipelineRasterizationStateCreateInfo);

impl Rasterization {
	pub fn new(
		depth_clamp: bool,
		raster_discard: bool,
		polygon_mode: PolygonMode,
		cull_mode: CullMode,
		front_face: FrontFace,
		depth_bias: Option<DepthBias>,
		line_width: f32
	) -> Rasterization {
		let (depth_bias_enable, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) = 
			depth_bias.map(|d| (vk::TRUE, d.constant_factor, d.clamp, d.slope_factor))
			.unwrap_or((vk::FALSE, 0.0f32, 0.0f32, 0.0f32));

		Rasterization(vk::PipelineRasterizationStateCreateInfo {
			depth_clamp_enable: if depth_clamp { vk::TRUE } else { vk::FALSE },
			rasterizer_discard_enable: if raster_discard { vk::TRUE } else { vk::FALSE },
			polygon_mode: polygon_mode.into_vulkan(),
			cull_mode: cull_mode.into_vulkan(),
			front_face: front_face.into_vulkan(),
			depth_bias_enable,
			depth_bias_constant_factor,
			depth_bias_clamp,
			depth_bias_slope_factor,
			line_width,
			..Default::default()
		})
	}

	pub(crate) fn as_vulkan(&self) -> &vk::PipelineRasterizationStateCreateInfo {
		&self.0
	}
}