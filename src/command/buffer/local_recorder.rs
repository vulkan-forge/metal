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
	resource,
	Resource,
	framebuffer,
	Framebuffer,
	Image,
	pipeline,
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
	pub fn begin_render_pass<'r, I: Image + 'static>(
		&'r mut self,
		render_pass: &Arc<framebuffer::RenderPass>,
		framebuffer: &Arc<Framebuffer<I>>,
		(x, y, width, height): (i32, i32, u32, u32),
		clear_values: &[format::ClearValue]
	) -> RenderPass<'r, 'a, B, pipeline::layout::Empty> {
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
			active_layout: PhantomData
		}
	}

	pub fn copy_buffer<S: 'a + mem::Buffer, D: 'a + mem::Buffer>(&mut self, src: S, dst: D, regions: &[BufferCopy]) {
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
	active_layout: PhantomData<L>
}

impl<'r, 'a, B: Buffer, L: pipeline::Layout> RenderPass<'r, 'a, B, L> {
	fn into_raw_parts(self) -> &'r mut LocalRecorder<'a, B> {
		let recorder = unsafe { std::ptr::read(&self.recorder) };
		std::mem::forget(self);
		recorder
	}
}

impl<'r, 'a, B: Buffer, L: pipeline::Layout> RenderPass<'r, 'a, B, L> {
	pub fn bind_descriptor_sets<M, T>(
		self,
		layout: M,
		transition: T
	) -> RenderPass<'r, 'a, B, M>
	where
		M: 'a + pipeline::Layout,
		T: pipeline::layout::set::Transition<L::Sets, M::Sets>
	{
		let recorder = self.into_raw_parts();

		unsafe {
			recorder.buffer.device().handle().cmd_bind_descriptor_sets(
				recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				layout.handle(),
				transition.first_set(),
				transition.descriptor_sets(),
				transition.dynamic_offsets()
			)
		};

		recorder.resources.insert(layout.into());

		RenderPass {
			recorder,
			active_layout: PhantomData
		}
	}

	pub fn draw<P, C, V>(
		&mut self,
		pipeline: &Arc<P>,
		push_constants: C,
		vertex_input: V,
		vertex_count: u32,
		instance_count: u32,
		first_vertex: u32,
		first_instance: u32
	) where
		P: pipeline::GraphicsPipeline,
		P::Layout: pipeline::layout::CompatibleWith<L>,
		C: pipeline::layout::push_constant::Setter<<P::Layout as pipeline::Layout>::PushConstants>,
		V: pipeline::vertex_input::Bind<'a, P::VertexInput>
	{
		unsafe {
			self.recorder.buffer.device().handle().cmd_bind_pipeline(
				self.recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				pipeline.handle()
			);

			for (range, data) in push_constants.ranges() {
				self.recorder.buffer.device().handle().cmd_push_constants(
					self.recorder.buffer.handle(),
					pipeline.layout().handle(),
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

	pub fn draw_indexed<P, C, V, I>(
		&mut self,
		pipeline: &Arc<P>,
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
		P: pipeline::GraphicsPipeline,
		P::Layout: pipeline::layout::CompatibleWith<L>,
		C: pipeline::layout::push_constant::Setter<<P::Layout as pipeline::Layout>::PushConstants>,
		V: pipeline::vertex_input::Bind<'a, P::VertexInput>,
		I: 'a + mem::IndexBuffer,
	{
		unsafe {
			self.recorder.buffer.device().handle().cmd_bind_pipeline(
				self.recorder.buffer.handle(),
				vk::PipelineBindPoint::GRAPHICS,
				pipeline.handle()
			);

			for (range, data) in push_constants.ranges() {
				self.recorder.buffer.device().handle().cmd_push_constants(
					self.recorder.buffer.handle(),
					pipeline.layout().handle(),
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

			self.recorder.buffer.device().handle().cmd_draw_indexed(
				self.recorder.buffer.handle(),
				index_count,
				instance_count,
				first_index,
				vertex_offset,
				first_instance
			)
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