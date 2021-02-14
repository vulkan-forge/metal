use ash::vk;

/// Layout of an image.
///
/// In the Vulkan API, each mipmap level of each array layer is in one of the layouts of this enum.
///
/// Unless you use some sort of high-level shortcut function, an image always starts in either
/// the `Undefined` or the `Preinitialized` layout.
/// Before you can use an image for a given purpose, you must ensure that the image in question is
/// in the layout required for that purpose. For example if you want to write data to an image, you
/// must first transition the image to the `TransferDstOptimal` layout. The `General` layout can
/// also be used as a general-purpose fit-all layout, but using it will result in slower operations.
///
/// Transitioning between layouts can only be done through a GPU-side operation that is part of
/// a command buffer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Layout {
	Undefined = vk::ImageLayout::UNDEFINED.as_raw(),
	General = vk::ImageLayout::GENERAL.as_raw(),
	ColorAttachmentOptimal = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL.as_raw(),
	DepthStencilAttachmentOptimal = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL.as_raw(),
	DepthStencilReadOnlyOptimal = vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL.as_raw(),
	ShaderReadOnlyOptimal = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL.as_raw(),
	TransferSrcOptimal = vk::ImageLayout::TRANSFER_SRC_OPTIMAL.as_raw(),
	TransferDstOptimal = vk::ImageLayout::TRANSFER_DST_OPTIMAL.as_raw(),
	Preinitialized = vk::ImageLayout::PREINITIALIZED.as_raw(),
	PresentSrc = vk::ImageLayout::PRESENT_SRC_KHR.as_raw(),
}

impl Layout {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::ImageLayout {
		vk::ImageLayout::from_raw(self as i32)
	}
}