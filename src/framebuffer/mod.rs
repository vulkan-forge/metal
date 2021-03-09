use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	convert::TryFrom,
	sync::Arc
};
use crate::{
	OomError,
	Device,
	image,
	Image,
	Resource
};
pub mod render_pass;
pub use render_pass::{
	RenderPass,
	RenderPassBuilder
};

#[derive(Debug, Clone, Copy)]
pub struct SampleCount(vk::SampleCountFlags);

#[derive(Debug, Clone, Copy)]
pub struct InvalidSampleCount;

impl TryFrom<u8> for SampleCount {
	type Error = InvalidSampleCount;
	
	#[inline]
	fn try_from(c: u8) -> Result<SampleCount, InvalidSampleCount> {
		let f = match c {
			1 => vk::SampleCountFlags::TYPE_1,
			2 => vk::SampleCountFlags::TYPE_2,
			4 => vk::SampleCountFlags::TYPE_4,
			8 => vk::SampleCountFlags::TYPE_8,
			16 => vk::SampleCountFlags::TYPE_16,
			32 => vk::SampleCountFlags::TYPE_32,
			64 => vk::SampleCountFlags::TYPE_64,
			_ => return Err(InvalidSampleCount)
		};

		Ok(SampleCount(f))
	}
}

impl SampleCount {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::SampleCountFlags {
		self.0
	}
}

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

pub struct Framebuffer<I: Image> {
	device: Arc<Device>,
	render_pass: Arc<RenderPass>,
	views: Vec<Arc<image::View<I>>>,
	handle: vk::Framebuffer
}

impl<I: Image> Framebuffer<I> {
	pub fn new(
		device: &Arc<Device>,
		render_pass: &Arc<RenderPass>,
		views: Vec<Arc<image::View<I>>>,
		size: (u32, u32),
		layers: u32
	) -> Result<Framebuffer<I>, CreationError> {
		let vk_attachments: Vec<_> = views.iter().map(|v| v.handle()).collect();

		let infos = vk::FramebufferCreateInfo {
			render_pass: render_pass.handle(),
			attachment_count: vk_attachments.len() as u32,
			p_attachments: vk_attachments.as_ptr(),
			width: size.0,
			height: size.1,
			layers,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_framebuffer(&infos, None)?
		};

		Ok(Framebuffer {
			device: device.clone(),
			render_pass: render_pass.clone(),
			views: views,
			handle
		})
	}

	pub fn views(&self) -> &[Arc<image::View<I>>] {
		&self.views
	}

	pub fn render_pass(&self) -> &Arc<RenderPass> {
		&self.render_pass
	}
}

unsafe impl<I: Image> crate::Resource for Framebuffer<I> {
	type Handle = vk::Framebuffer;

	fn handle(&self) -> vk::Framebuffer {
		self.handle
	}
}

impl<I: Image> Drop for Framebuffer<I> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_framebuffer(self.handle, None)
		}
	}
}