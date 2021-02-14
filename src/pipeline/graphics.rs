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
	framebuffer
};
use super::{
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
	DynamicStates,
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

pub struct Graphics<S, D> {
	device: Arc<Device>,
	render_subpass: framebuffer::render_pass::subpass::Reference,
	handle: vk::Pipeline,
	stages: S,
	dynamic_state: PhantomData<D>
}

impl<S: Stages, D: DynamicStates> Graphics<S, D> {
	/// Creates a new graphics pipeline.
	pub fn new(
		device: &Arc<Device>,
		stages: S,
		vertex_input_and_assembly: Option<(VertexInput, InputAssembly)>,
		tesselation: Option<Tesselation>,
		viewports: &[Viewport],
		scissors: &[Scissor],
		rasterization: Rasterization,
		multisample: Multisample,
		depth_test: Option<DepthTest>,
		stencil_tests: Option<(StencilTest, StencilTest)>,
		color_blend: ColorBlend,
		layout: &Arc<Layout>,
		render_subpass: framebuffer::render_pass::subpass::Reference
	) -> Result<Graphics<S, D>, CreationError> {
		let mut vk_stages = Vec::new();
		stages.for_each(|stage| vk_stages.push(vk::PipelineShaderStageCreateInfo {
			stage: stage.ty.into_vulkan(),
			module: stage.entry_point.module().handle(),
			p_name: stage.entry_point.name().as_ptr(),
			p_specialization_info: std::ptr::null(),
			..Default::default()
		}));

		let (p_vertex_input_state, p_input_assembly_state) = vertex_input_and_assembly.map(|(i, a)| (i.as_vulkan() as *const _, a.as_vulkan() as *const _)).unwrap_or((std::ptr::null(), std::ptr::null()));

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

		let mut vk_dynamic_states = Vec::new();
		D::for_each(|component| vk_dynamic_states.push(component.into_vulkan()));
		let dynamic_state = vk::PipelineDynamicStateCreateInfo {
			dynamic_state_count: vk_dynamic_states.len() as u32,
			p_dynamic_states: vk_dynamic_states.as_ptr() as *const _,
			..Default::default()
		};

		let infos = vk::GraphicsPipelineCreateInfo {
			// Shader stages
			stage_count: vk_stages.len() as u32,
			p_stages: vk_stages.as_ptr(),
			//
			p_vertex_input_state,
			p_input_assembly_state,
			p_tessellation_state: tesselation.as_ref().map(|t| t.as_vulkan() as *const _).unwrap_or(std::ptr::null()),
			//
			p_viewport_state: &viewport_state as *const _,
			p_rasterization_state: rasterization.as_vulkan() as *const _,
			p_multisample_state: multisample.as_vulkan() as *const _,
			p_depth_stencil_state: depth_stencil_state.as_ref().map(|t| t as *const _).unwrap_or(std::ptr::null()),
			p_color_blend_state: color_blend.as_vulkan() as *const _,
			p_dynamic_state: &dynamic_state as *const _,
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

		Ok(Graphics {
			device: device.clone(),
			render_subpass: render_subpass,
			handle,
			stages,
			dynamic_state: PhantomData
		})
	}

	pub fn stages(&self) -> &S {
		&self.stages
	}

	pub fn render_subpass(&self) -> &framebuffer::render_pass::subpass::Reference {
		&self.render_subpass
	}
}

impl<S, D: dynamic_state::WithViewport> Graphics<S, D> {
	pub fn set_viewport(&mut self, viewports: &[Viewport]) {
		panic!("TODO")
	}
}

impl<S, D> Drop for Graphics<S, D> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline(self.handle, None)
		}
	}
}