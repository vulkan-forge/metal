use ash::vk;
use std::ops::BitOr;

/// Describes how an image is going to be used. This is **not** just an optimization.
///
/// If you try to use an image in a way that you didn't declare, a panic will happen.
///
/// If `transient_attachment` is true, then only `color_attachment`, `depth_stencil_attachment`
/// and `input_attachment` can be true as well. The rest must be false or an error will be returned
/// when creating the image.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Usage {
	/// Can be used as a source for transfers. Includes blits.
	pub transfer_source: bool,

	/// Can be used as a destination for transfers. Includes blits.
	pub transfer_destination: bool,

	/// Can be sampled from a shader.
	pub sampled: bool,

	/// Can be used as an image storage in a shader.
	pub storage: bool,

	/// Can be attached as a color attachment to a framebuffer.
	pub color_attachment: bool,

	/// Can be attached as a depth, stencil or depth-stencil attachment to a framebuffer.
	pub depth_stencil_attachment: bool,

	/// Indicates that this image will only ever be used as a temporary framebuffer attachment.
	/// As soon as you leave a render pass, the content of transient images becomes undefined.
	///
	/// This is a hint to the Vulkan implementation that it may not need allocate any memory for
	/// this image if the image can live entirely in some cache.
	pub transient_attachment: bool,

	/// Can be used as an input attachment. In other words, you can draw to it in a subpass then
	/// read from it in a following pass.
	pub input_attachment: bool,
}

impl Usage {
	/// Builds a `Usage` with all values set to true. Note that using the returned value will
	/// produce an error because of `transient_attachment` being true.
	#[inline]
	pub fn all() -> Usage {
		Usage {
			transfer_source: true,
			transfer_destination: true,
			sampled: true,
			storage: true,
			color_attachment: true,
			depth_stencil_attachment: true,
			transient_attachment: true,
			input_attachment: true,
		}
	}

	/// Builds a `Usage` with all values set to false. Useful as a default value.
	///
	/// # Example
	///
	/// ```rust
	/// use vulkano::image::Usage as Usage;
	///
	/// let _usage = Usage {
	///     transfer_destination: true,
	///     sampled: true,
	///     .. Usage::none()
	/// };
	/// ```
	#[inline]
	pub fn none() -> Usage {
		Usage {
			transfer_source: false,
			transfer_destination: false,
			sampled: false,
			storage: false,
			color_attachment: false,
			depth_stencil_attachment: false,
			transient_attachment: false,
			input_attachment: false,
		}
	}

	/// Builds a Usage with color_attachment set to true and the rest to false.
	#[inline]
	pub fn color_attachment() -> Usage {
		Usage {
			transfer_source: false,
			transfer_destination: false,
			sampled: false,
			storage: false,
			color_attachment: true,
			depth_stencil_attachment: false,
			transient_attachment: false,
			input_attachment: false,
		}
	}

	/// Builds a Usage with depth_stencil_attachment set to true and the rest to false.
	#[inline]
	pub fn depth_stencil_attachment() -> Usage {
		Usage {
			transfer_source: false,
			transfer_destination: false,
			sampled: false,
			storage: false,
			color_attachment: false,
			depth_stencil_attachment: true,
			transient_attachment: false,
			input_attachment: false,
		}
	}

	/// Builds a Usage with color_attachment and transient_attachment set to true and the rest to false.
	#[inline]
	pub fn transient_color_attachment() -> Usage {
		Usage {
			transfer_source: false,
			transfer_destination: false,
			sampled: false,
			storage: false,
			color_attachment: true,
			depth_stencil_attachment: false,
			transient_attachment: true,
			input_attachment: false,
		}
	}

	/// Builds a Usage with depth_stencil_attachment and transient_attachment set to true and the rest to false.
	#[inline]
	pub fn transient_depth_stencil_attachment() -> Usage {
		Usage {
			transfer_source: false,
			transfer_destination: false,
			sampled: false,
			storage: false,
			color_attachment: false,
			depth_stencil_attachment: true,
			transient_attachment: true,
			input_attachment: false,
		}
	}

	#[inline]
	pub(crate) fn to_vulkan(&self) -> vk::ImageUsageFlags {
		let mut result = vk::ImageUsageFlags::empty();
		if self.transfer_source {
			result |= vk::ImageUsageFlags::TRANSFER_SRC;
		}
		if self.transfer_destination {
			result |= vk::ImageUsageFlags::TRANSFER_DST;
		}
		if self.sampled {
			result |= vk::ImageUsageFlags::SAMPLED;
		}
		if self.storage {
			result |= vk::ImageUsageFlags::STORAGE;
		}
		if self.color_attachment {
			result |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
		}
		if self.depth_stencil_attachment {
			result |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
		}
		if self.transient_attachment {
			result |= vk::ImageUsageFlags::TRANSIENT_ATTACHMENT;
		}
		if self.input_attachment {
			result |= vk::ImageUsageFlags::INPUT_ATTACHMENT;
		}
		result
	}

	#[inline]
	pub(crate) fn from_vulkan(val: vk::ImageUsageFlags) -> Usage {
		Usage {
			transfer_source: val.contains(vk::ImageUsageFlags::TRANSFER_SRC),
			transfer_destination: val.contains(vk::ImageUsageFlags::TRANSFER_DST),
			sampled: val.contains(vk::ImageUsageFlags::SAMPLED),
			storage: val.contains(vk::ImageUsageFlags::STORAGE),
			color_attachment: val.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT),
			depth_stencil_attachment: val.contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
			transient_attachment: val.contains(vk::ImageUsageFlags::TRANSIENT_ATTACHMENT),
			input_attachment: val.contains(vk::ImageUsageFlags::INPUT_ATTACHMENT),
		}
	}
}

impl BitOr for Usage {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self {
		Usage {
			transfer_source: self.transfer_source || rhs.transfer_source,
			transfer_destination: self.transfer_destination || rhs.transfer_destination,
			sampled: self.sampled || rhs.sampled,
			storage: self.storage || rhs.storage,
			color_attachment: self.color_attachment || rhs.color_attachment,
			depth_stencil_attachment: self.depth_stencil_attachment || rhs.depth_stencil_attachment,
			transient_attachment: self.transient_attachment || rhs.transient_attachment,
			input_attachment: self.input_attachment || rhs.input_attachment,
		}
	}
}