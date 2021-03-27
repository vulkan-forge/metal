use ash::vk;
use crate::{
	format::Format,
	image,
	framebuffer::SampleCount
};
use super::{
	LoadOp,
	StoreOp
};

/// Describes an attachment that will be used in a render pass.
#[derive(Debug, Clone, Copy)]
pub struct Attachment {
	/// Format of the image that is going to be bound.
	pub format: Format,
	/// Number of samples of the image that is going to be bound.
	pub samples: SampleCount,

	/// What the implementation should do with that attachment at the start of the render pass.
	pub load: LoadOp,
	/// What the implementation should do with that attachment at the end of the render pass.
	pub store: StoreOp,

	/// Equivalent of `load` for the stencil component of the attachment, if any. Irrelevant if
	/// there is no stencil component.
	pub stencil_load: LoadOp,
	/// Equivalent of `store` for the stencil component of the attachment, if any. Irrelevant if
	/// there is no stencil component.
	pub stencil_store: StoreOp,

	/// Layout that the image is going to be in at the start of the renderpass.
	pub initial_layout: image::Layout,

	/// Layout that the image will be transitioned to at the end of the renderpass.
	pub final_layout: image::Layout,
}

impl Attachment {
	#[inline]
	pub(crate) fn into_vulkan(&self) -> vk::AttachmentDescription {
		vk::AttachmentDescription {
			flags: vk::AttachmentDescriptionFlags::empty(), // TODO
			format: self.format.into_vulkan(),
			samples: self.samples.into_vulkan(),
			load_op: self.load.into_vulkan(),
			store_op: self.store.into_vulkan(),
			stencil_load_op: self.stencil_load.into_vulkan(),
			stencil_store_op: self.stencil_store.into_vulkan(),
			initial_layout: self.initial_layout.into_vulkan(),
			final_layout: self.final_layout.into_vulkan(),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct UnformatedReference(u32);

impl UnformatedReference {
	pub(crate) fn new(index: u32) -> UnformatedReference {
		UnformatedReference(index)
	}

	pub fn with_layout(self, layout: image::Layout) -> Reference {
		Reference(
			vk::AttachmentReference {
				attachment: self.0,
				layout: layout.into_vulkan()
			}
		)
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Reference(vk::AttachmentReference);

impl Reference {
	pub fn as_vulkan(&self) -> *const vk::AttachmentReference {
		&self.0
	}
}

#[derive(Clone, Debug)]
pub struct Attachments(Vec<vk::AttachmentDescription>);

impl Attachments {
	#[inline]
	pub fn new() -> Attachments {
		Attachments(Vec::new())
	}

	#[inline]
	pub fn len(&self) -> u32 {
		self.0.len() as u32
	}

	#[inline]
	pub fn add(&mut self, desc: Attachment) -> UnformatedReference {
		let index = self.len();
		self.0.push(desc.into_vulkan());
		UnformatedReference::new(index)
	}

	#[inline]
	pub(crate) fn as_ptr(&self) -> *const vk::AttachmentDescription {
		self.0.as_ptr()
	}
}