use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	collections::HashSet,
	marker::PhantomData
};
use crate::{
	resource::{
		self,
		Reference
	},
	framebuffer,
	Framebuffer,
	pipeline::{
		self,
		vertex_input::VertexInput,
		input_assembly::InputAssembly
	},
	descriptor,
	format,
	mem
};
use super::{
	Buffer,
	BufferCopy
};

pub struct LocalRecorder<'a, B: Buffer> {
	pub(crate) buffer: B,
	pub(crate) resources: HashSet<resource::Ref<'a>>
}

impl<'a, B: Buffer> LocalRecorder<'a, B> {
	pub fn begin_render_pass<'r, A: AsRef<[vk::ImageView]> + 'static, C: pipeline::layout::PushConstants>(
		&'r mut self,
		render_pass: &Arc<framebuffer::RenderPass>,
		framebuffer: &Arc<Framebuffer<A>>,
		(x, y, width, height): (i32, i32, u32, u32),
		clear_values: &[format::ClearValue]
	) -> RenderPass<'r, 'a, B, pipeline::layout::NoSets<C>> {
		let infos = vk::RenderPassBeginInfo {
			render_pass: render_pass.handle(),
			framebuffer: framebuffer.handle(),
			render_area: vk::Rect2D {
				offset: vk::Offset2D { x, y },
				extent: vk::Extent2D { width, height }
			},
			clear_value_count: clear_values.len() as u32,
			p_clear_values: clear_values.as_ptr() as *const _,
			..Default::default()
		};

		unsafe {
			self.buffer.device().handle().cmd_begin_render_pass(self.buffer.handle(), &infos, vk::SubpassContents::INLINE)
		}

		self.resources.insert(render_pass.clone().into());
		self.resources.insert(framebuffer.clone().into());

		RenderPass {
			recorder: self,
			layout: PhantomData
		}
	}

	pub fn copy_buffer<S: 'a + mem::buffer::sub::Read, D: 'a + mem::buffer::sub::Write>(&mut self, src: S, dst: D, regions: &[BufferCopy]) {
		unsafe {
			self.buffer.device().handle().cmd_copy_buffer(self.buffer.handle(), src.handle(), dst.handle(), regions)
		}

		self.resources.insert(src.into());
		self.resources.insert(dst.into());
	}
}

/// Record a render pass.
/// 
/// The render pass ends when the `RenderPassRecorder` is dropped.
pub struct RenderPass<'r, 'a, B: Buffer, L: pipeline::Layout> {
	recorder: &'r mut LocalRecorder<'a, B>,
	layout: PhantomData<L>
}

impl<'r, 'a, B: Buffer, L: pipeline::Layout> RenderPass<'r, 'a, B, L> {
	fn into_raw_parts(self) -> &'r mut LocalRecorder<'a, B> {
		let recorder = unsafe { std::ptr::read(&self.recorder) };
		std::mem::forget(self);
		recorder
	}
}

impl<'r, 'a, B: Buffer, L: pipeline::Layout> RenderPass<'r, 'a, B, L> {
	/// Bind a new pipeline.
	pub fn bind_pipeline<'p, P, S, T>(
		&'p mut self,
		pipeline: &Arc<P>,
		dynamic_states: S
	) -> Pipeline<'p, 'a, B, P>
	where
		P: pipeline::Graphics<Layout=T>,
		S: pipeline::dynamic_state::Set<P>
	{
		unsafe {
			self.recorder.buffer.device().handle().cmd_bind_pipeline(
				self.recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				pipeline.handle()
			);

			if let Some(viewports) = dynamic_states.viewports() {
				self.recorder.buffer.device().handle().cmd_set_viewport(
					self.recorder.buffer.handle(),
					0,
					std::mem::transmute(viewports) // safe thanks to #[repr(transparent)] for Viewport.
				)
			}

			if let Some(scissors) = dynamic_states.scissors() {
				self.recorder.buffer.device().handle().cmd_set_scissor(
					self.recorder.buffer.handle(),
					0,
					std::mem::transmute(scissors) // safe thanks to #[repr(transparent)] for Scissors.
				)
			}
		}

		Pipeline {
			recorder: self.recorder,
			active_pipeline: pipeline.clone()
		}
	}

	pub fn bind_descriptor_sets<M, T>(
		self,
		layout: M,
		transition: T
	) -> RenderPass<'r, 'a, B, M>
	where
		M: 'a + pipeline::Layout,
		T: descriptor::set::Transition<'a, L::DescriptorSets, M::DescriptorSets>
	{
		let recorder = self.into_raw_parts();

		unsafe {
			recorder.buffer.device().handle().cmd_bind_descriptor_sets(
				recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				layout.handle(),
				transition.first_set(),
				transition.descriptor_sets().as_ref(),
				transition.dynamic_offsets().as_ref()
			)
		};

		recorder.resources.insert(layout.into());
		let new_sets = transition.into_descriptor_sets();
		recorder.resources.extend(new_sets);

		RenderPass {
			recorder,
			layout: PhantomData
		}
	}
}

impl<'r, 'a, B: Buffer, L: pipeline::Layout> Drop for RenderPass<'r, 'a, B, L> {
	fn drop(&mut self) {
		unsafe {
			self.recorder.buffer.device().handle().cmd_end_render_pass(self.recorder.buffer.handle())
		}
	}
}

/// Record pipeline commands.
pub struct Pipeline<'r, 'a, B: Buffer, P: pipeline::Graphics> {
	recorder: &'r mut LocalRecorder<'a, B>,
	active_pipeline: Arc<P>
}

impl<'r, 'a, B: Buffer, P: pipeline::Graphics> Pipeline<'r, 'a, B, P> {
	pub fn bind_descriptor_sets<T>(
		&mut self,
		transition: T
	)
	where
		T: descriptor::set::Transition<'a, <P::Layout as pipeline::Layout>::DescriptorSets, <P::Layout as pipeline::Layout>::DescriptorSets>
	{
		unsafe {
			self.recorder.buffer.device().handle().cmd_bind_descriptor_sets(
				self.recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				self.active_pipeline.layout().handle(),
				transition.first_set(),
				transition.descriptor_sets().as_ref(),
				transition.dynamic_offsets().as_ref()
			)
		};

		let new_sets = transition.into_descriptor_sets();
		self.recorder.resources.extend(new_sets);
	}

	pub fn draw<C, V>(
		&mut self,
		push_constants: C,
		vertex_input: V,
		vertex_count: u32,
		instance_count: u32,
		first_vertex: u32,
		first_instance: u32
	) where
		C: pipeline::layout::push_constant::Setter<<P::Layout as pipeline::Layout>::PushConstants>,
		V: pipeline::vertex_input::Bind<'a, P::VertexInput>
	{
		unsafe {
			for (range, data) in push_constants.ranges().as_ref() {
				self.recorder.buffer.device().handle().cmd_push_constants(
					self.recorder.buffer.handle(),
					self.active_pipeline.layout().handle(),
					range.0.stage_flags,
					range.0.offset,
					std::slice::from_raw_parts(*data, range.0.size as usize)
				)
			}

			let (first_binding, vertex_buffers, offsets) = vertex_input.get();
			if !vertex_buffers.is_empty() {
				self.recorder.buffer.device().handle().cmd_bind_vertex_buffers(
					self.recorder.buffer.handle(),
					first_binding,
					vertex_buffers.as_vulkan(),
					offsets.as_ref()
				);
			}

			self.recorder.buffer.device().handle().cmd_draw(
				self.recorder.buffer.handle(),
				vertex_count,
				instance_count,
				first_vertex,
				first_instance
			)
		}
	}

	/// Note: when using list topologies (`PointList`, `LineList` and `TriangleList`), 
	/// `index_count` is the number of element in that list (the number of points/lines/faces).
	/// For instance, if the topology is `TriangleList`,
	/// then `index_count` must be the number of input indexes divided by 3.
	pub fn draw_indexed<C, V, I>(
		&mut self,
		push_constants: C,
		vertex_input: V,
		index_buffer: I,
		offset: u64,
		index_count: u32,
		instance_count: u32,
		first_index: u32,
		vertex_offset: i32,
		first_instance: u32
	) where
		C: pipeline::layout::push_constant::Setter<<P::Layout as pipeline::Layout>::PushConstants>,
		V: pipeline::vertex_input::Bind<'a, P::VertexInput>,
		I: 'a + mem::buffer::sub::IndexRead<<<P::VertexInput as VertexInput>::Assembly as InputAssembly>::Topology>,
	{
		unsafe {
			for (range, data) in push_constants.ranges().as_ref() {
				self.recorder.buffer.device().handle().cmd_push_constants(
					self.recorder.buffer.handle(),
					self.active_pipeline.layout().handle(),
					range.0.stage_flags,
					range.0.offset,
					std::slice::from_raw_parts(*data, range.0.size as usize)
				)
			}

			let (first_binding, vertex_buffers, offsets) = vertex_input.get();
			if !vertex_buffers.is_empty() {
				self.recorder.buffer.device().handle().cmd_bind_vertex_buffers(
					self.recorder.buffer.handle(),
					first_binding,
					vertex_buffers.as_vulkan(),
					offsets.as_ref()
				);
			}

			self.recorder.buffer.device().handle().cmd_bind_index_buffer(
				self.recorder.buffer.handle(),
				index_buffer.handle(),
				offset,
				index_buffer.index_type()
			);

			// log::debug!("draw indexed: {} indexes of type {:?}", index_count * index_buffer.index_per_item(), index_buffer.index_type());

			self.recorder.buffer.device().handle().cmd_draw_indexed(
				self.recorder.buffer.handle(),
				index_count * index_buffer.index_per_item(),
				instance_count,
				first_index,
				vertex_offset,
				first_instance
			)
		}
	}
}