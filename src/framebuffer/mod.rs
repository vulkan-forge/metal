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
	resource::{
		self,
		Reference
	}
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

pub struct Framebuffer<A: AsRef<[vk::ImageView]> = image::LocalViews<'static>> {
	device: Arc<Device>,
	render_pass: Arc<RenderPass>,
	attachments: A,
	handle: vk::Framebuffer
}

impl<A: AsRef<[vk::ImageView]>> Framebuffer<A> {
	pub fn new(
		device: &Arc<Device>,
		render_pass: &Arc<RenderPass>,
		attachments: A,
		size: (u32, u32),
		layers: u32
	) -> Result<Self, CreationError> {
		let vk_attachements = attachments.as_ref();

		let infos = vk::FramebufferCreateInfo {
			render_pass: render_pass.handle(),
			attachment_count: vk_attachements.len() as u32,
			p_attachments: vk_attachements.as_ptr(),
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
			attachments,
			handle
		})
	}

	pub fn attachments(&self) -> &A {
		&self.attachments
	}

	pub fn render_pass(&self) -> &Arc<RenderPass> {
		&self.render_pass
	}
}

unsafe impl<A: AsRef<[vk::ImageView]>> resource::AbstractReference for Framebuffer<A> {
	fn uid(&self) -> u64 {
		use ash::vk::Handle;
		self.handle.as_raw()
	}
}

unsafe impl<A: AsRef<[vk::ImageView]>> resource::Reference for Framebuffer<A> {
	type Handle = vk::Framebuffer;

	fn handle(&self) -> vk::Framebuffer {
		self.handle
	}
}

impl<A: AsRef<[vk::ImageView]>> Drop for Framebuffer<A> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_framebuffer(self.handle, None)
		}
	}
}