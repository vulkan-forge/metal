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
	Resource
};
use super::{
	shader,
	Stages,
	Layout,
	VertexInput,
	InputAssembly,
	Tesselation,
	Viewport,
	Scissor,
	Rasterization,
	Multisample,
	DepthTest,
	StencilTest,
	ColorBlend,
	DynamicStates
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

pub trait GraphicsPipeline: Resource<Handle=vk::Pipeline> {
	type Layout: Layout;
	type VertexInput: VertexInput;
	type DynamicStates: DynamicStates;

	fn layout(&self) -> &Self::Layout;
}

pub struct Graphics<L: Layout, I: VertexInput, D: DynamicStates> {
	device: Arc<Device>,
	render_subpass: framebuffer::render_pass::subpass::Reference,
	handle: vk::Pipeline,
	shaders: Vec<Arc<shader::Module>>,
	layout: L,
	vertex_input: PhantomData<I>,
	dynamic_states: PhantomData<D>
}

impl<L: Layout, I: VertexInput, D: DynamicStates> Graphics<L, I, D> {
	/// Creates a new graphics pipeline.
	pub fn new<S: Stages, const V: usize>(
		device: &Arc<Device>,
		stages: &S,
		vertex_input: I,
		tesselation: Option<Tesselation>,
		viewports: [Viewport; V],
		scissors: [Scissor; V],
		rasterization: Rasterization,
		multisample: Multisample,
		depth_test: Option<DepthTest>,
		stencil_tests: Option<(StencilTest, StencilTest)>,
		color_blend: ColorBlend,
		layout: L,
		render_subpass: framebuffer::render_pass::subpass::Reference
	) -> Result<Graphics<L, I, D>, CreationError> {
		let mut shaders = Vec::new();
		let mut vk_stages = Vec::new();
		stages.for_each(|stage| {
			vk_stages.push(vk::PipelineShaderStageCreateInfo {
				stage: stage.ty.into_vulkan(),
				module: stage.entry_point.module().handle(),
				p_name: stage.entry_point.name().as_ptr(),
				p_specialization_info: std::ptr::null(),
				..Default::default()
			});

			shaders.push(stage.entry_point.module().clone())
		});

		let viewport_state = vk::PipelineViewportStateCreateInfo {
			viewport_count: viewports.len() as u32,
			p_viewports: viewports.as_ptr() as *const _,
			scissor_count: scissors.len() as u32,
			p_scissors: scissors.as_ptr() as *const _,
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

		let vk_dynamic_states = D::vulkan();
		let dynamic_state = vk::PipelineDynamicStateCreateInfo {
			dynamic_state_count: vk_dynamic_states.len() as u32,
			p_dynamic_states: vk_dynamic_states.as_ptr() as *const _,
			..Default::default()
		};

		let mut vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default();
		vertex_input_state.vertex_binding_description_count = vertex_input.bindings().len() as u32;
		vertex_input_state.p_vertex_binding_descriptions = vertex_input.bindings().as_ptr() as *const _;
		vertex_input_state.vertex_attribute_description_count = vertex_input.attributes().len() as u32;
		vertex_input_state.p_vertex_attribute_descriptions = vertex_input.attributes().as_ptr() as *const _;

		let infos = vk::GraphicsPipelineCreateInfo {
			// Shader stages
			stage_count: vk_stages.len() as u32,
			p_stages: vk_stages.as_ptr(),
			//
			p_vertex_input_state: &vertex_input_state,
			p_input_assembly_state: &I::Assembly::vulkan(),
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
			shaders,
			layout,
			vertex_input: PhantomData,
			dynamic_states: PhantomData
		})
	}

	pub fn render_subpass(&self) -> &framebuffer::render_pass::subpass::Reference {
		&self.render_subpass
	}
}

unsafe impl<L: Layout, I: VertexInput, D: DynamicStates> crate::Resource for Graphics<L, I, D> {
	type Handle = vk::Pipeline;

	fn handle(&self) -> vk::Pipeline {
		self.handle
	}
}

impl<L: Layout, I: VertexInput, D: DynamicStates> GraphicsPipeline for Graphics<L, I, D> {
	type Layout = L;
	type VertexInput = I;
	type DynamicStates = D;

	fn layout(&self) -> &L {
		&self.layout
	}
}

// impl<S, D: dynamic_state::WithViewport> Graphics<D> {
// 	pub fn set_viewports(&mut self, viewports: &[Viewport]) {
// 		panic!("TODO")
// 	}
// }

// impl<S, D: dynamic_state::WithViewport> Graphics<D> {
// 	pub fn set_scissors(&mut self, scissors: &[Scissor]) {
// 		panic!("TODO")
// 	}
// }

impl<L: Layout, I: VertexInput, D: DynamicStates> Drop for Graphics<L, I, D> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline(self.handle, None)
		}
	}
}