use ash::vk;
use std::{
	ops,
	sync::Arc
};
use super::{
	RenderPass,
	Attachments,
	attachment
};

macro_rules! pipeline_stages {
	($($elem:ident => $val:expr,)+) => (
		#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
		#[allow(missing_docs)]
		pub struct PipelineStages {
			$(
				pub $elem: bool,
			)+
		}

		impl PipelineStages {
			/// Builds an `PipelineStages` struct with none of the stages set.
			pub fn none() -> PipelineStages {
				PipelineStages {
					$(
						$elem: false,
					)+
				}
			}

			#[inline]
			pub(crate) fn into_vulkan(self) -> vk::PipelineStageFlags {
				let mut result = vk::PipelineStageFlags::empty();
				$(
					if self.$elem { result |= $val }
				)+
				result
			}
		}

		impl ops::BitOr for PipelineStages {
			type Output = PipelineStages;

			#[inline]
			fn bitor(self, rhs: PipelineStages) -> PipelineStages {
				PipelineStages {
					$(
						$elem: self.$elem || rhs.$elem,
					)+
				}
			}
		}

		impl ops::BitOrAssign for PipelineStages {
			#[inline]
			fn bitor_assign(&mut self, rhs: PipelineStages) {
				$(
					self.$elem = self.$elem || rhs.$elem;
				)+
			}
		}
	);
}

pipeline_stages! {
	top_of_pipe => vk::PipelineStageFlags::TOP_OF_PIPE,
	draw_indirect => vk::PipelineStageFlags::DRAW_INDIRECT,
	vertex_input => vk::PipelineStageFlags::VERTEX_INPUT,
	vertex_shader => vk::PipelineStageFlags::VERTEX_SHADER,
	tessellation_control_shader => vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER,
	tessellation_evaluation_shader => vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER,
	geometry_shader => vk::PipelineStageFlags::GEOMETRY_SHADER,
	fragment_shader => vk::PipelineStageFlags::FRAGMENT_SHADER,
	early_fragment_tests => vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
	late_fragment_tests => vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
	color_attachment_output => vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
	compute_shader => vk::PipelineStageFlags::COMPUTE_SHADER,
	transfer => vk::PipelineStageFlags::TRANSFER,
	bottom_of_pipe => vk::PipelineStageFlags::BOTTOM_OF_PIPE,
	host => vk::PipelineStageFlags::HOST,
	all_graphics => vk::PipelineStageFlags::ALL_GRAPHICS,
	all_commands => vk::PipelineStageFlags::ALL_COMMANDS,
}

macro_rules! access_flags {
	($($elem:ident => $val:expr,)+) => (
		#[derive(Debug, Copy, Clone, Default)]
		#[allow(missing_docs)]
		pub struct AccessFlags {
			$(
				pub $elem: bool,
			)+
		}

		impl AccessFlags {
			/// Builds an `AccessFlags` struct with all bits set.
			pub fn all() -> AccessFlags {
				AccessFlags {
					$(
						$elem: true,
					)+
				}
			}

			/// Builds an `AccessFlags` struct with none of the bits set.
			pub fn none() -> AccessFlags {
				AccessFlags {
					$(
						$elem: false,
					)+
				}
			}

			#[inline]
			pub(crate) fn into_vulkan(self) -> vk::AccessFlags {
				let mut result = vk::AccessFlags::empty();
				$(
					if self.$elem { result |= $val }
				)+
				result
			}
		}

		impl ops::BitOr for AccessFlags {
			type Output = AccessFlags;

			#[inline]
			fn bitor(self, rhs: AccessFlags) -> AccessFlags {
				AccessFlags {
					$(
						$elem: self.$elem || rhs.$elem,
					)+
				}
			}
		}

		impl ops::BitOrAssign for AccessFlags {
			#[inline]
			fn bitor_assign(&mut self, rhs: AccessFlags) {
				$(
					self.$elem = self.$elem || rhs.$elem;
				)+
			}
		}
	);
}

access_flags! {
	indirect_command_read => vk::AccessFlags::INDIRECT_COMMAND_READ,
	index_read => vk::AccessFlags::INDEX_READ,
	vertex_attribute_read => vk::AccessFlags::VERTEX_ATTRIBUTE_READ,
	uniform_read => vk::AccessFlags::UNIFORM_READ,
	input_attachment_read => vk::AccessFlags::INPUT_ATTACHMENT_READ,
	shader_read => vk::AccessFlags::SHADER_READ,
	shader_write => vk::AccessFlags::SHADER_WRITE,
	color_attachment_read => vk::AccessFlags::COLOR_ATTACHMENT_READ,
	color_attachment_write => vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
	depth_stencil_attachment_read => vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ,
	depth_stencil_attachment_write => vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
	transfer_read => vk::AccessFlags::TRANSFER_READ,
	transfer_write => vk::AccessFlags::TRANSFER_WRITE,
	host_read => vk::AccessFlags::HOST_READ,
	host_write => vk::AccessFlags::HOST_WRITE,
	memory_read => vk::AccessFlags::MEMORY_READ,
	memory_write => vk::AccessFlags::MEMORY_WRITE,
}

/// Describes one of the passes of a render pass.
///
/// # Restrictions
///
/// All these restrictions are checked when the `RenderPass` object is created.
/// TODO: that'r not the case ^
///
/// - The number of color attachments must be less than the limit of the physical device.
/// - All the attachments in `color_attachments` and `depth_stencil` must have the same
///   samples count.
/// - If any attachment is used as both an input attachment and a color or
///   depth/stencil attachment, then each use must use the same layout.
/// - Elements of `preserve_attachments` must not be used in any of the other members.
/// - If `resolve_attachments` is not empty, then all the resolve attachments must be attachments
///   with 1 sample and all the color attachments must have more than 1 sample.
/// - If `resolve_attachments` is not empty, all the resolve attachments must have the same format
///   as the color attachments.
/// - If the first use of an attachment in this renderpass is as an input attachment and the
///   attachment is not also used as a color or depth/stencil attachment in the same subpass,
///   then the loading operation must not be `Clear`.
///
// TODO: add tests for all these restrictions
// TODO: allow unused attachments (for example attachment 0 and 2 are used, 1 is unused)
#[derive(Debug, Clone)]
pub struct Subpass {
	/// Indices and layouts of attachments to use as color attachments.
	pub color_attachments: Vec<attachment::Reference>, // TODO: Vec is slow

	/// Index and layout of the attachment to use as depth-stencil attachment.
	pub depth_stencil: Option<attachment::Reference>,

	/// Indices and layouts of attachments to use as input attachments.
	pub input_attachments: Vec<attachment::Reference>, // TODO: Vec is slow

	/// If not empty, each color attachment will be resolved into each corresponding entry of
	/// this list.
	///
	/// If this value is not empty, it **must** be the same length as `color_attachments`.
	pub resolve_attachments: Vec<attachment::Reference>, // TODO: Vec is slow

	/// Indices of attachments that will be preserved during this pass.
	pub preserve_attachments: Vec<usize>, // TODO: Vec is slow
}

impl Subpass {
	pub fn as_ref(&self) -> SubpassRef {
		SubpassRef {
			color_attachments: self.color_attachments.as_ref(),
			depth_stencil: self.depth_stencil.as_ref(),
			input_attachments: self.input_attachments.as_ref(),
			resolve_attachments: self.resolve_attachments.as_ref(),
			preserve_attachments: self.preserve_attachments.as_ref()
		}
	}
}

/// Describes one of the passes of a render pass.
///
/// # Restrictions
///
/// All these restrictions are checked when the `RenderPass` object is created.
/// TODO: that'r not the case ^
///
/// - The number of color attachments must be less than the limit of the physical device.
/// - All the attachments in `color_attachments` and `depth_stencil` must have the same
///   samples count.
/// - If any attachment is used as both an input attachment and a color or
///   depth/stencil attachment, then each use must use the same layout.
/// - Elements of `preserve_attachments` must not be used in any of the other members.
/// - If `resolve_attachments` is not empty, then all the resolve attachments must be attachments
///   with 1 sample and all the color attachments must have more than 1 sample.
/// - If `resolve_attachments` is not empty, all the resolve attachments must have the same format
///   as the color attachments.
/// - If the first use of an attachment in this renderpass is as an input attachment and the
///   attachment is not also used as a color or depth/stencil attachment in the same subpass,
///   then the loading operation must not be `Clear`.
///
// TODO: add tests for all these restrictions
// TODO: allow unused attachments (for example attachment 0 and 2 are used, 1 is unused)
#[derive(Debug, Clone, Copy)]
pub struct SubpassRef<'r> {
	/// Indices and layouts of attachments to use as color attachments.
	pub color_attachments: &'r [attachment::Reference],

	/// Index and layout of the attachment to use as depth-stencil attachment.
	pub depth_stencil: Option<&'r attachment::Reference>,

	/// Indices and layouts of attachments to use as input attachments.
	pub input_attachments: &'r [attachment::Reference],

	/// If not empty, each color attachment will be resolved into each corresponding entry of
	/// this list.
	///
	/// If this value is not empty, it **must** be the same length as `color_attachments`.
	pub resolve_attachments: &'r [attachment::Reference],

	/// Indices of attachments that will be preserved during this pass.
	pub preserve_attachments: &'r [usize],
}

impl<'r> SubpassRef<'r> {
	/// Returns the vulkan subpass description.
	/// 
	/// # Safety
	/// 
	/// User must ensure that the returned object does not outlive the input lifetime.
	pub(crate) fn into_vulkan(self) -> vk::SubpassDescription {
		vk::SubpassDescription {
			flags: vk::SubpassDescriptionFlags::empty(), // TODO
			pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
			
			input_attachment_count: self.input_attachments.len() as u32,
			p_input_attachments: self.input_attachments.as_ptr() as *const _,
			
			color_attachment_count: self.color_attachments.len() as u32,
			p_color_attachments: self.color_attachments.as_ptr() as *const _,
			
			p_resolve_attachments: if self.resolve_attachments.is_empty() {
				std::ptr::null()
			} else {
				self.resolve_attachments.as_ptr() as *const _
			},

			p_depth_stencil_attachment: self.depth_stencil.as_ref().map(|r| r.as_vulkan()).unwrap_or(std::ptr::null()),

			preserve_attachment_count: self.preserve_attachments.len() as u32,
			p_preserve_attachments: self.preserve_attachments.as_ptr() as *const _
		}
	}
}

impl<'a> From<&'a Subpass> for SubpassRef<'a> {
	fn from(subpass: &'a Subpass) -> SubpassRef<'a> {
		subpass.as_ref()
	}
}

/// Describes a dependency between two subpasses of a render pass.
///
/// The implementation is allowed to change the order of the passes within a render pass, unless
/// you specify that there exists a dependency between two passes (ie. the result of one will be
/// used as the input of another one).
#[derive(Debug, Clone)]
pub struct Dependency {
	/// Index of the subpass that writes the data that `destination_subpass` is going to use.
	pub source_subpass: u32,

	/// Index of the subpass that reads the data that `source_subpass` wrote.
	pub destination_subpass: u32,

	/// The pipeline stages that must be finished on the previous subpass before the destination
	/// subpass can start.
	pub source_stages: PipelineStages,

	/// The pipeline stages of the destination subpass that must wait for the source to be finished.
	/// Stages that are earlier of the stages specified here can start before the source is
	/// finished.
	pub destination_stages: PipelineStages,

	/// The way the source subpass accesses the attachments on which we depend.
	pub source_access: AccessFlags,

	/// The way the destination subpass accesses the attachments on which we depend.
	pub destination_access: AccessFlags,

	/// If false, then the whole subpass must be finished for the next one to start. If true, then
	/// the implementation can start the new subpass for some given pixels as long as the previous
	/// subpass is finished for these given pixels.
	///
	/// In other words, if the previous subpass has some side effects on other parts of an
	/// attachment, then you should set it to false.
	///
	/// Passing `false` is always safer than passing `true`, but in practice you rarely need to
	/// pass `false`.
	pub by_region: bool,
}

impl Dependency {
	pub(crate) fn into_vulkan(self) -> vk::SubpassDependency {
		vk::SubpassDependency {
			src_subpass: self.source_subpass,
			dst_subpass: self.destination_subpass,
			src_stage_mask: self.source_stages.into_vulkan(),
			dst_stage_mask: self.destination_stages.into_vulkan(),
			src_access_mask: self.source_access.into_vulkan(),
			dst_access_mask: self.destination_access.into_vulkan(),
			dependency_flags: if self.by_region {
				vk::DependencyFlags::BY_REGION
			} else {
				vk::DependencyFlags::empty()
			}
		}
	}
}

pub struct Subpasses<'a> {
	attachments: &'a Attachments,
	subpasses: Vec<vk::SubpassDescription>
}

impl<'a> Subpasses<'a> {
	pub fn new(attachments: &'a Attachments) -> Subpasses {
		Subpasses {
			attachments,
			subpasses: Vec::new()
		}
	}

	pub fn attachments(&self) -> &'a Attachments {
		self.attachments
	}

	pub fn add(&mut self, subpass: SubpassRef<'a>) {
		self.subpasses.push(subpass.into_vulkan())
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Reference {
	render_pass: Arc<RenderPass>,
	index: u32
}

impl Reference {
	pub(crate) fn new(render_pass: &Arc<RenderPass>, index: u32) -> Self {
		Self {
			render_pass: render_pass.clone(),
			index
		}
	}

	pub fn render_pass(&self) -> &Arc<RenderPass> {
		&self.render_pass
	}

	pub fn index(&self) -> u32 {
		self.index
	}
}