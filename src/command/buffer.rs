use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	rc::Rc,
	any::Any,
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

pub struct Buffer<'a> {
	pool: Rc<Pool>,
	resources: HashSet<resource::Ref<'a>>,
	handle: vk::CommandBuffer
}

impl<'a> Buffer<'a> {
	pub(crate) fn new(pool: &Rc<Pool>, handle: vk::CommandBuffer) -> Self {
		Buffer {
			pool: pool.clone(),
			resources: HashSet::new(),
			handle
		}
	}

	pub(crate) fn handle(&self) -> vk::CommandBuffer {
		self.handle
	}

	pub fn record<F>(&mut self, f: F) -> Result<(), RecordError> where F: FnOnce(Recorder<'_, 'a>) -> () {
		let infos = vk::CommandBufferBeginInfo {
			flags: vk::CommandBufferUsageFlags::empty(), // TODO
			p_inheritance_info: std::ptr::null(), // no inheritance for primary buffers.
			..Default::default()
		};

		unsafe {
			self.device().handle().begin_command_buffer(self.handle, &infos)?
		}

		f(Recorder {
			buffer: self
		});

		unsafe {
			self.device().handle().end_command_buffer(self.handle)?
		}

		Ok(())
	}
}

impl<'a> DeviceOwned for Buffer<'a> {
	fn device(&self) -> &Arc<Device> {
		self.pool.device()
	}
}

impl<'a> Drop for Buffer<'a> {
	fn drop(&mut self) {
		unsafe {
			self.pool.device().handle().free_command_buffers(self.pool.handle(), &[self.handle])
		}
	}
}

pub struct Recorder<'b, 'a> {
	buffer: &'b mut Buffer<'a>,
}

impl<'b, 'a> Recorder<'b, 'a> {
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
			self.buffer.device().handle().cmd_begin_render_pass(self.buffer.handle, &infos, vk::SubpassContents::INLINE)
		}

		self.buffer.resources.insert(render_pass.clone().into());
		self.buffer.resources.insert(framebuffer.clone().into());
	}

	pub fn end_render_pass(&mut self) {
		unsafe {
			self.buffer.device().handle().cmd_end_render_pass(self.buffer.handle)
		}
	}

	pub fn bind_graphics_pipeline(&mut self, pipeline: &Arc<pipeline::Graphics>) {
		unsafe {
			self.buffer.device().handle().cmd_bind_pipeline(self.buffer.handle, vk::PipelineBindPoint::GRAPHICS, pipeline.handle())
		}

		self.buffer.resources.insert(pipeline.clone().into());
	}

	pub fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
		unsafe {
			self.buffer.device().handle().cmd_draw(self.buffer.handle, vertex_count, instance_count, first_vertex, first_instance)
		}
	}

	pub fn copy_buffer<S: 'a + crate::Buffer, D: 'a + crate::Buffer>(&mut self, src: &Arc<S>, dst: &Arc<D>, regions: &[BufferCopy]) {
		unsafe {
			self.buffer.device().handle().cmd_copy_buffer(self.buffer.handle, src.handle(), dst.handle(), regions)
		}

		self.buffer.resources.insert(src.clone().into());
		self.buffer.resources.insert(dst.clone().into());
	}
}