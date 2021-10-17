use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	marker::PhantomData
};
use crate::{
	OomError,
	Device,
	framebuffer,
	resource::{
		self,
		Reference
	}
};
use super::{
	shader,
	Handle,
	Stages,
	Layout,
	vertex_input,
	VertexInput,
	InputAssembly,
	Tesselation,
	Rasterization,
	Multisample,
	DepthTest,
	StencilTest,
	ColorBlend,
	dynamic_state
};

#[derive(Debug)]
pub enum CreationError {
	OomError(OomError),
	InvalidShader,
	CompileRequired
}

impl From<vk::Result> for CreationError {
	fn from(e: vk::Result) -> Self {
		match e {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OomError(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OomError(OomError::Device),
			vk::Result::ERROR_INVALID_SHADER_NV => CreationError::InvalidShader,
			vk::Result::ERROR_PIPELINE_COMPILE_REQUIRED_EXT => CreationError::CompileRequired,
			_ => unreachable!()
		}
	}
}

pub trait Graphics: resource::Reference<Handle=Handle> {
	type Layout: Layout;
	type VertexInput: VertexInput;
	
	type ViewportsScissors: dynamic_state::ViewportsScissors;
	type Rasterization: dynamic_state::Rasterization;
	type BlendConstants: dynamic_state::BlendConstants;
	type DepthBounds: dynamic_state::DepthBounds;
	type StencilTest: dynamic_state::StencilTest;

	fn layout(&self) -> &Self::Layout;
}

/// Raw graphics pipeline.
/// 
/// A "raw" graphics pipeline cannot be directly used as is.
/// Its purpose is to be wrapped inside a newtype implementing the
/// `Graphics` trait using the [`graphics_pipeline!`] macro.
/// 
/// ## Example
/// 
/// ```
/// graphics_pipeline! {
/// 	pub struct MyPipeline<Layout, VertexInput, DynamicState>;
/// }
/// ```
pub struct Raw<
	L: Layout,
	I: VertexInput,
	V: dynamic_state::ViewportsScissors,
	R: dynamic_state::Rasterization,
	B: dynamic_state::BlendConstants,
	D: dynamic_state::DepthBounds,
	S: dynamic_state::StencilTest
> {
	device: Arc<Device>,
	render_subpass: framebuffer::render_pass::subpass::Reference,
	handle: vk::Pipeline,
	shader_modules: Vec<Arc<shader::Module>>,
	layout: L,
	vertex_input: PhantomData<I>,
	dynamic_states: PhantomData<(V, R, B, D, S)>
}

/// Creates a new pipeline type.
/// 
/// The created type will be a newtype wrapping a [`Raw`] graphics pipeline and
/// implementing the [`Graphics`] trait with the given
/// [`Layout`], [`VertexInput`] and dynamic states.
/// 
/// ## Example
/// 
/// ```
/// graphics_pipeline! {
/// 	/// My pipeline.
/// 	pub struct MyPipeline {
/// 		type Layout = MyLayout;
/// 		type VertexInput = MyVertexInput;
/// 		type ViewportsScissors = MyViewportsScissors;
/// 		type ColorBlend = MyColorBlend;
/// 		type Rasterization = MyRasterization;
/// 		type DepthBounds = MyDepthBounds;
/// 		type StencilTest = MyStencilTest;
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! graphics_pipeline {
	{
		$(#[$doc:meta])*
		$vis:vis struct $id:ident {
			type Layout = $layout:ty;
			type VertexInput = $vertex_input:ty;
			type ViewportsScissors = $viewports_scissors:ty;
			type BlendConstants = $blend_constants:ty;
			type Rasterization = $rasterization:ty;
			type DepthBounds = $depth_bounds:ty;
			type StencilTest = $stencil_test:ty;
			type RenderPass = $render_pass:ty;
		}
	} => {
		$(#[$doc])*
		$vis struct $id($crate::pipeline::graphics::Raw<
			$layout,
			$vertex_input,
			$viewports_scissors,
			$blend_constants,
			$rasterization,
			$depth_bounds,
			$stencil_test
		>);

		// unsafe impl $crate::resource::AbstractReference for $id {
		// 	fn uid(&self) -> u64 {
		// 		self.0.handle().as_raw()
		// 	}
		// }

		unsafe impl $crate::resource::Reference for $id {
			type Handle = $crate::pipeline::Handle;

			fn handle(&self) -> Self::Handle {
				self.0.handle()
			}
		}

		impl $crate::pipeline::Graphics for $id {
			type Layout = $layout;
			type VertexInput = $vertex_input;
			type ViewportsScissors = $viewports_scissors;
			type BlendConstants = $blend_constants;
			type Rasterization = $rasterization;
			type DepthBounds = $depth_bounds;
			type StencilTest = $stencil_test;

			fn layout(&self) -> &Self::Layout {
				self.0.layout()
			}
		}
	};
}

impl<
	L: Layout,
	I: VertexInput,
	V: dynamic_state::ViewportsScissors,
	R: dynamic_state::Rasterization,
	B: dynamic_state::BlendConstants,
	D: dynamic_state::DepthBounds,
	S: dynamic_state::StencilTest
> Raw<L, I, V, R, B, D, S> {
	/// Creates a new raw graphics pipeline.
	pub fn new<M: Stages>( // TODO statically ensure that the given stages match the layout & vertex input.
		device: &Arc<Device>,
		stages: &M,
		input_assembly: InputAssembly,
		tesselation: Option<Tesselation>,
		viewports: <V::Viewports as dynamic_state::Viewports>::InitialType,
		scissors: <V::Scissors as dynamic_state::Scissors>::InitialType,
		rasterization: Rasterization,
		multisample: Multisample,
		depth_test: Option<DepthTest>,
		stencil_tests: Option<(StencilTest, StencilTest)>,
		color_blend: ColorBlend,
		layout: L,
		render_subpass: framebuffer::render_pass::subpass::Reference
	) -> Result<Self, CreationError> {
		use dynamic_state::viewports::InitialType as InitialViewport;
		use dynamic_state::scissors::InitialType as InitialScissor;

		let mut shader_modules = Vec::new();
		let mut vk_stages = Vec::new();
		stages.for_each(|stage| {
			vk_stages.push(vk::PipelineShaderStageCreateInfo {
				stage: stage.ty.into_vulkan(),
				module: stage.entry_point.module().handle(),
				p_name: stage.entry_point.name().as_ptr(),
				p_specialization_info: std::ptr::null(),
				..Default::default()
			});

			shader_modules.push(stage.entry_point.module().clone())
		});

		let viewport_count = <V::Viewports as dynamic_state::Viewports>::COUNT as u32;
		let viewport_state = vk::PipelineViewportStateCreateInfo {
			viewport_count,
			p_viewports: viewports.ptr() as *const _,
			scissor_count: viewport_count,
			p_scissors: scissors.ptr() as *const _,
			..Default::default()
		};

		let depth_stencil_state = if depth_test.is_some() || stencil_tests.is_some() {
			let mut infos = vk::PipelineDepthStencilStateCreateInfo::default();

			if let Some(depth_test) = depth_test {
				depth_test.set_vulkan(&mut infos);
			}

			if let Some((front, back)) = stencil_tests {
				front.set_vulkan(&mut infos.front);
				back.set_vulkan(&mut infos.back);
			}

			Some(infos)
		} else {
			None
		};

		let mut vk_dynamic_states = Vec::new();
		V::build_vulkan_dynamic_states(&mut vk_dynamic_states);
		R::build_vulkan_dynamic_states(&mut vk_dynamic_states);
		D::build_vulkan_dynamic_states(&mut vk_dynamic_states);
		B::build_vulkan_dynamic_states(&mut vk_dynamic_states);
		S::build_vulkan_dynamic_states(&mut vk_dynamic_states);
		let dynamic_state = vk::PipelineDynamicStateCreateInfo {
			dynamic_state_count: vk_dynamic_states.len() as u32,
			p_dynamic_states: vk_dynamic_states.as_ptr() as *const _,
			..Default::default()
		};

		let mut vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default();
		vertex_input_state.vertex_binding_description_count = <I::Bindings as vertex_input::Bindings>::LIST.len() as u32;
		vertex_input_state.p_vertex_binding_descriptions = <I::Bindings  as vertex_input::Bindings>::LIST.as_ptr() as *const _;
		vertex_input_state.vertex_attribute_description_count = I::ATTRIBUTES.len() as u32;
		vertex_input_state.p_vertex_attribute_descriptions = I::ATTRIBUTES.as_ptr() as *const _;

		let infos = vk::GraphicsPipelineCreateInfo {
			// Shader stages
			stage_count: vk_stages.len() as u32,
			p_stages: vk_stages.as_ptr(),
			//
			p_vertex_input_state: &vertex_input_state,
			p_input_assembly_state: input_assembly.as_vulkan(),
			p_tessellation_state: tesselation.as_ref().map(|t| t.as_vulkan() as *const _).unwrap_or(std::ptr::null()),
			//
			p_viewport_state: &viewport_state,
			p_rasterization_state: rasterization.as_vulkan(),
			p_multisample_state: multisample.as_vulkan(),
			p_depth_stencil_state: depth_stencil_state.as_ref().map(|t| t as *const _).unwrap_or(std::ptr::null()),
			p_color_blend_state: color_blend.as_vulkan(),
			p_dynamic_state: &dynamic_state,
			//
			layout: layout.handle(),
			render_pass: render_subpass.render_pass().handle(),
			subpass: render_subpass.index(),
			base_pipeline_handle: vk::Pipeline::null(),
			base_pipeline_index: 0,
			..Default::default()
		};

		let handle = unsafe {
			match device.handle().create_graphics_pipelines(vk::PipelineCache::null(), &[infos], None) {
				Ok(handles) => handles.into_iter().next().unwrap(),
				Err((handles, e)) => {
					for handle in handles {
						device.handle().destroy_pipeline(handle, None);
					}

					return Err(e.into())
				}
			}
		};

		Ok(Self {
			device: device.clone(),
			render_subpass: render_subpass,
			handle,
			shader_modules,
			layout,
			vertex_input: PhantomData,
			dynamic_states: PhantomData
		})
	}

	pub fn handle(&self) -> Handle {
		self.handle
	}

	pub fn render_subpass(&self) -> &framebuffer::render_pass::subpass::Reference {
		&self.render_subpass
	}

	pub fn shader_modules(&self) -> &[Arc<shader::Module>] {
		&self.shader_modules
	}

	pub fn layout(&self) -> &L {
		&self.layout
	}
}

impl<
	L: Layout,
	I: VertexInput,
	V: dynamic_state::ViewportsScissors,
	R: dynamic_state::Rasterization,
	B: dynamic_state::BlendConstants,
	D: dynamic_state::DepthBounds,
	S: dynamic_state::StencilTest
> Drop for Raw<L, I, V, R, B, D, S> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline(self.handle, None)
		}
	}
}