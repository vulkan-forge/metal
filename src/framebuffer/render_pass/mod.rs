use std::{
	sync::Arc,
	hash::{
		Hash,
		Hasher
	}
};
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	OomError,
	Device,
	DeviceOwned,
	resource
};

pub mod subpass;
mod attachment;

pub use subpass::{
	Subpass,
	SubpassRef,
	Subpasses
};
pub use attachment::{
	Attachment,
	Attachments
};

/// Describes what the implementation should do with an attachment at the start of the subpass.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LoadOp {
	/// The content of the attachment will be loaded from memory. This is what you want if you want
	/// to draw over something existing.
	///
	/// While this is the most intuitive option, it is also the slowest because it uses a lot of
	/// memory bandwidth.
	Load = vk::AttachmentLoadOp::LOAD.as_raw(),

	/// The content of the attachment will be filled by the implementation with a uniform value
	/// that you must provide when you start drawing.
	///
	/// This is what you usually use at the start of a frame, in order to reset the content of
	/// the color, depth and/or stencil buffers.
	///
	/// See the `draw_inline` and `draw_secondary` methods of `PrimaryComputeBufferBuilder`.
	Clear = vk::AttachmentLoadOp::CLEAR.as_raw(),

	/// The attachment will have undefined content.
	///
	/// This is what you should use for attachments that you intend to entirely cover with draw
	/// commands.
	/// If you are going to fill the attachment with a uniform value, it is better to use `Clear`
	/// instead.
	DontCare = vk::AttachmentLoadOp::DONT_CARE.as_raw(),
}

impl LoadOp {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::AttachmentLoadOp {
		vk::AttachmentLoadOp::from_raw(self as i32)
	}
}

/// Describes what the implementation should do with an attachment after all the subpasses have
/// completed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StoreOp {
	/// The attachment will be stored. This is what you usually want.
	///
	/// While this is the most intuitive option, it is also slower than `DontCare` because it can
	/// take time to write the data back to memory.
	Store = vk::AttachmentStoreOp::STORE.as_raw(),

	/// What happens is implementation-specific.
	///
	/// This is purely an optimization compared to `Store`. The implementation doesn't need to copy
	/// from the internal cache to the memory, which saves memory bandwidth.
	///
	/// This doesn't mean that the data won't be copied, as an implementation is also free to not
	/// use a cache and write the output directly in memory. In other words, the content of the
	/// image will be undefined.
	DontCare = vk::AttachmentStoreOp::DONT_CARE.as_raw(),
}

impl StoreOp {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::AttachmentStoreOp {
		vk::AttachmentStoreOp::from_raw(self as i32)
	}
}

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError),
}

impl From<vk::Result> for CreationError {
	#[inline]
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub struct RenderPassBuilder<'a> {
	attachments: &'a Attachments,
	subpasses: Vec<vk::SubpassDescription>,
	dependencies: Vec<vk::SubpassDependency>
}

impl<'a> RenderPassBuilder<'a> {
	pub fn new(attachments: &'a Attachments) -> RenderPassBuilder<'a> {
		RenderPassBuilder {
			attachments,
			subpasses: Vec::new(),
			dependencies: Vec::new()
		}
	}

	pub fn add<S>(&mut self, subpass: S) -> u32 where S: Into<SubpassRef<'a>> {
		// TODO check attachment references.
		let index = self.subpasses.len() as u32;
		self.subpasses.push(subpass.into().into_vulkan());
		index
	}

	pub fn add_dependency(&mut self, dependency: subpass::Dependency) {
		// TODO check subpass references.
		self.dependencies.push(dependency.into_vulkan())
	}

	pub fn build(self, device: &Arc<Device>) -> Result<RenderPass, CreationError> {
		let infos = vk::RenderPassCreateInfo {
			attachment_count: self.attachments.len(),
			p_attachments: self.attachments.as_ptr(),
			subpass_count: self.subpasses.len() as u32,
			p_subpasses: self.subpasses.as_ptr(),
			dependency_count: self.dependencies.len() as u32,
			p_dependencies: self.dependencies.as_ptr(),
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_render_pass(&infos, None)?
		};

		Ok(RenderPass {
			device: device.clone(),
			handle,
			subpass_count: infos.subpass_count
		})
	}
}

pub struct RenderPass {
	device: Arc<Device>,
	handle: vk::RenderPass,
	subpass_count: u32
}

impl RenderPass {
	#[inline]
	pub fn subpass(self: &Arc<Self>, index: u32) -> Option<subpass::Reference> {
		if index < self.subpass_count {
			Some(subpass::Reference::new(self, index))
		} else {
			None
		}
	}
}

impl DeviceOwned for RenderPass {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}

unsafe impl resource::AbstractReference for RenderPass {
	fn uid(&self) -> u64 {
		use ash::vk::Handle;
		self.handle.as_raw()
	}
}

unsafe impl resource::Reference for RenderPass {
	type Handle = vk::RenderPass;

	fn handle(&self) -> vk::RenderPass {
		self.handle
	}
}

impl PartialEq for RenderPass {
	fn eq(&self, other: &RenderPass) -> bool {
		self.handle == other.handle
	}
}

impl Eq for RenderPass {}

impl Hash for RenderPass {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.handle.hash(h)
	}
}

impl Drop for RenderPass {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_render_pass(self.handle, None)
		}
	}
}