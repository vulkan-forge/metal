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
	mem
};
use super::Pool;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub enum RecordError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for RecordError {
	fn from(r: vk::Result) -> RecordError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => RecordError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => RecordError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub type BufferCopy = vk::BufferCopy;

pub type VulkanBuffer = vk::CommandBuffer;

/// Command buffer trait.
pub trait Buffer: Sized + DeviceOwned {
	fn handle(&self) -> VulkanBuffer;

	fn record<'a, F>(self, f: F) -> Result<Recorded<'a, Self>, RecordError> where F: FnOnce(&mut Recorder<'a, Self>) -> () {
		let infos = vk::CommandBufferBeginInfo {
			flags: vk::CommandBufferUsageFlags::empty(), // TODO
			p_inheritance_info: std::ptr::null(), // no inheritance for primary buffers.
			..Default::default()
		};

		unsafe {
			self.device().handle().begin_command_buffer(self.handle(), &infos)?
		}

		let mut recorder = Recorder {
			buffer: self,
			resources: HashSet::new()
		};

		f(&mut recorder);

		unsafe {
			recorder.buffer.device().handle().end_command_buffer(recorder.buffer.handle())?
		}

		Ok(Recorded {
			buffer: recorder.buffer,
			resources: recorder.resources
		})
	}
}

impl<'a, B: Buffer> Buffer for &'a B {
	#[inline]
	fn handle(&self) -> VulkanBuffer {
		(*self).handle()
	}
}

pub struct Recorded<'a, B: Buffer> {
	buffer: B,
	resources: HashSet<resource::Ref<'a>>
}

impl<'a, B: Buffer> Recorded<'a, B> {
	#[inline]
	pub(crate) fn handle(&self) -> vk::CommandBuffer {
		self.buffer.handle()
	}

	pub fn resources(&self) -> &HashSet<resource::Ref<'a>> {
		&self.resources
	}
}

pub struct Raw {
	pool: Rc<Pool>,
	handle: vk::CommandBuffer
}

impl Raw {
	pub(crate) fn new(pool: &Rc<Pool>, handle: vk::CommandBuffer) -> Self {
		Raw {
			pool: pool.clone(),
			handle
		}
	}
}

impl Buffer for Raw {
	fn handle(&self) -> vk::CommandBuffer {
		self.handle
	}
}

impl DeviceOwned for Raw {
	fn device(&self) -> &Arc<Device> {
		self.pool.device()
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.pool.device().handle().free_command_buffers(self.pool.handle(), &[self.handle])
		}
	}
}

pub struct Recorder<'a, B: Buffer> {
	buffer: B,
	resources: HashSet<resource::Ref<'a>>
}

impl<'a, B: Buffer> Recorder<'a, B> {
	pub fn begin_render_pass<I: Image + 'static>(
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

	pub fn bind_graphics_pipeline(&mut self, pipeline: &Arc<pipeline::Graphics>) {
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

	pub fn bind_index_buffer<I: 'a + mem::IndexBuffer>(&mut self, index_buffer: I, offset: u64) {
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

	pub fn copy_buffer<S: 'a + mem::Buffer, D: 'a + mem::Buffer>(&mut self, src: S, dst: D, regions: &[BufferCopy]) {
		unsafe {
			self.buffer.device().handle().cmd_copy_buffer(self.buffer.handle(), src.handle(), dst.handle(), regions)
		}

		self.resources.insert(src.into());
		self.resources.insert(dst.into());
	}
}