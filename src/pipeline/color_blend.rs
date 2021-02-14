use ash::vk;
use crate::ops;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Operation {
	Add = vk::BlendOp::ADD.as_raw(),
	Substract = vk::BlendOp::SUBTRACT.as_raw(),
	ReverseSubstract = vk::BlendOp::REVERSE_SUBTRACT.as_raw(),
	Min = vk::BlendOp::MIN.as_raw(),
	Max = vk::BlendOp::MAX.as_raw()
}

impl Operation {
	pub(crate) fn into_vulkan(self) -> vk::BlendOp {
		vk::BlendOp::from_raw(self as i32)
	}
}

impl Default for Operation {
	fn default() -> Operation {
		Operation::Add
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum BlendFactor {
	Zero = vk::BlendFactor::ZERO.as_raw(),
	One = vk::BlendFactor::ONE.as_raw(),
	SourceColor = vk::BlendFactor::SRC_COLOR.as_raw(),
	OneMinusSourceColor = vk::BlendFactor::ONE_MINUS_SRC_COLOR.as_raw(),
	TargetColor = vk::BlendFactor::DST_COLOR.as_raw(),
	OneMinusTargetColor = vk::BlendFactor::ONE_MINUS_DST_COLOR.as_raw(),
	SourceAlpha = vk::BlendFactor::SRC_ALPHA.as_raw(),
	OneMinusSourceAlpha = vk::BlendFactor::ONE_MINUS_SRC_ALPHA.as_raw(),
	TargetAlpha = vk::BlendFactor::DST_ALPHA.as_raw(),
	OneMinusTargetAlpha = vk::BlendFactor::ONE_MINUS_DST_ALPHA.as_raw(),
	ConstantColor = vk::BlendFactor::CONSTANT_COLOR.as_raw(),
	OneMinusConstantColor = vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR.as_raw(),
	ConstantAlpha = vk::BlendFactor::CONSTANT_ALPHA.as_raw(),
	OneMinusConstantAlpha = vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA.as_raw(),
	SourceAlphaSaturate = vk::BlendFactor::SRC_ALPHA_SATURATE.as_raw(),
	Source1Color = vk::BlendFactor::SRC1_COLOR.as_raw(),
	OneMinusSource1Color = vk::BlendFactor::ONE_MINUS_SRC1_COLOR.as_raw(),
	Source1Alpha = vk::BlendFactor::SRC1_ALPHA.as_raw(),
	OneMinusSource1Alpha = vk::BlendFactor::ONE_MINUS_SRC1_ALPHA.as_raw()
}

impl BlendFactor {
	pub(crate) fn into_vulkan(self) -> vk::BlendFactor {
		vk::BlendFactor::from_raw(self as i32)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct ColorComponents {
	red: bool,
	green: bool,
	blue: bool,
	alpha: bool
}

impl ColorComponents {
	pub fn rgba() -> Self {
		Self {
			red: true,
			green: true,
			blue: true,
			alpha: true
		}
	}

	pub(crate) fn into_vulkan(self) -> vk::ColorComponentFlags {
		let mut flags = vk::ColorComponentFlags::empty();

		if self.red {
			flags |= vk::ColorComponentFlags::R;
		}

		if self.green {
			flags |= vk::ColorComponentFlags::G;
		}

		if self.blue {
			flags |= vk::ColorComponentFlags::B;
		}

		if self.alpha {
			flags |= vk::ColorComponentFlags::A;
		}

		flags
	}
}

pub struct AttachmentBlend {
	pub source_color_factor: BlendFactor,
	pub target_color_factor: BlendFactor,
	pub color_operation: Operation,
	pub source_alpha_factor: BlendFactor,
	pub target_alpha_factor: BlendFactor,
	pub alpha_operation: Operation
}

impl AttachmentBlend {
	pub fn new(
		source_color_factor: BlendFactor,
		target_color_factor: BlendFactor,
		color_operation: Operation,
		source_alpha_factor: BlendFactor,
		target_alpha_factor: BlendFactor,
		alpha_operation: Operation
	) -> AttachmentBlend {
		AttachmentBlend {
			source_color_factor,
			target_color_factor,
			color_operation,
			source_alpha_factor,
			target_alpha_factor,
			alpha_operation
		}
	}

	pub fn set_vulkan(&self, infos: &mut vk::PipelineColorBlendAttachmentState) {
		infos.blend_enable = vk::TRUE;
		infos.src_color_blend_factor = self.source_color_factor.into_vulkan();
		infos.dst_color_blend_factor = self.target_color_factor.into_vulkan();
		infos.color_blend_op = self.color_operation.into_vulkan();
		infos.src_alpha_blend_factor = self.source_alpha_factor.into_vulkan();
		infos.dst_alpha_blend_factor = self.target_alpha_factor.into_vulkan();
		infos.alpha_blend_op = self.alpha_operation.into_vulkan();
	}
}

#[repr(transparent)]
pub struct Attachment(vk::PipelineColorBlendAttachmentState);

impl Attachment {
	pub fn new(
		blend: Option<AttachmentBlend>,
		color_write_components: ColorComponents
	) -> Attachment {
		let mut inner = vk::PipelineColorBlendAttachmentState {
			blend_enable: vk::FALSE,
			color_write_mask: color_write_components.into_vulkan(),
			..Default::default()
		};

		if let Some(blend) = blend {
			blend.set_vulkan(&mut inner);
		}

		Attachment(inner)
	}
}

pub struct ColorBlend {
	attachments: Vec<Attachment>,
	inner: vk::PipelineColorBlendStateCreateInfo
}

impl ColorBlend {
	pub fn new(
		logic_operation: Option<ops::Logic>,
		blend_constants: [f32; 4]
	) -> ColorBlend {
		ColorBlend {
			attachments: Vec::new(),
			inner: vk::PipelineColorBlendStateCreateInfo {
				logic_op_enable: if logic_operation.is_some() { vk::TRUE } else { vk::FALSE },
				logic_op: logic_operation.map(|o| o.into_vulkan()).unwrap_or_default(),
				blend_constants,
				..Default::default()
			}
		}
	}

	pub fn add_attachment(&mut self, a: Attachment) {
		self.attachments.push(a);
		self.inner.attachment_count = self.attachments.len() as u32;
		self.inner.p_attachments = self.attachments.as_ptr() as *const _;
	}

	pub fn with_attachment(mut self, a: Attachment) -> Self {
		self.add_attachment(a);
		self
	}

	pub(crate) fn as_vulkan(&self) -> &vk::PipelineColorBlendStateCreateInfo {
		&self.inner
	}
}