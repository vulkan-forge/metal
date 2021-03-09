use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	rc::Rc,
	collections::HashSet
};
use crate::{
	resource,
	OomError,
	Device,
	DeviceOwned,
	framebuffer,
	Framebuffer,
	Image,
	pipeline,
	format,
	mem,
	command::{
		pool,
		Pool
	},
	Resource
};
use super::{
	Buffer,
	BufferCopy
};

pub struct Recorder<'a, B: Send + Buffer> {
	pub(crate) buffer: B,
	pub(crate) resources: HashSet<resource::SendRef<'a>>
}

impl<'a, B: Send + Buffer> Recorder<'a, B> {
	pub fn begin_render_pass<I: Send + Sync + Image + 'static>(
		&mut self,
		render_pass: &Arc<framebuffer::RenderPass>,
		framebuffer: &Arc<Framebuffer<I>>,
		(x, y, width, height): (i32, i32, u32, u32),
		clear_values: &[format::ClearValue]
	) {
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
	}

	pub fn end_render_pass(&mut self) {
		unsafe {
			self.buffer.device().handle().cmd_end_render_pass(self.buffer.handle())
		}
	}

	pub fn bind_graphics_pipeline<P: Sync + Send + pipeline::GraphicsPipeline>(&mut self, pipeline: &Arc<P>) {
		unsafe {
			self.buffer.device().handle().cmd_bind_pipeline(self.buffer.handle(), vk::PipelineBindPoint::GRAPHICS, pipeline.handle())
		}

		self.resources.insert(pipeline.clone().into());
	}

	pub fn bind_vertex_buffers(&mut self, first_binding: u32, vertex_buffers: mem::Buffers<'a>, offsets: &[u64]) {
		assert_eq!(vertex_buffers.len(), offsets.len());

		unsafe {
			self.buffer.device().handle().cmd_bind_vertex_buffers(
				self.buffer.handle(),
				first_binding,
				vertex_buffers.as_vulkan(),
				offsets
			)
		}

		for buffer in vertex_buffers {
			self.resources.insert(buffer);
		}
	}

	pub fn bind_index_buffer<I: 'a + Send + mem::IndexBuffer>(&mut self, index_buffer: I, offset: u64) {
		unsafe {
			self.buffer.device().handle().cmd_bind_index_buffer(self.buffer.handle(), index_buffer.handle(), offset, index_buffer.index_type())
		}

		self.resources.insert(index_buffer.into());
	}

	pub fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
		unsafe {
			self.buffer.device().handle().cmd_draw(self.buffer.handle(), vertex_count, instance_count, first_vertex, first_instance)
		}
	}

	pub fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
		unsafe {
			self.buffer.device().handle().cmd_draw_indexed(
				self.buffer.handle(),
				index_count,
				instance_count,
				first_index,
				vertex_offset,
				first_instance
			)
		}
	}

	pub fn copy_buffer<S: 'a + Send + mem::Buffer, D: 'a + Send + mem::Buffer>(&mut self, src: S, dst: D, regions: &[BufferCopy]) {
		unsafe {
			self.buffer.device().handle().cmd_copy_buffer(self.buffer.handle(), src.handle(), dst.handle(), regions)
		}

		self.resources.insert(src.into());
		self.resources.insert(dst.into());
	}

	// pub fn push_constants<T: Copy>(
	// 	&mut self,
	// 	layout: &Arc<pipeline::Layout>,
	// 	shader_stages: pipeline::shader::Stages,
	// 	value: T
	// ) {
	// 	// ...

	// 	unsafe {
	// 		self.buffer.device().handle().cmd_push_constants(
	// 			self.buffer.handle(),
	// 			layout.handle(),
	// 			shader_stages.into_vulkan(),
	// 			offset,
	// 			size,
	// 			values
	// 		)
	// 	}
	// }
}