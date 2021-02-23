use ash::{
	vk,
	version::InstanceV1_0
};
use crate::instance::PhysicalDevice;

mod clear_value;

pub use clear_value::ClearValue;

/// The properties of an image format that are supported by a physical device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FormatProperties {
	/// Features available for images with linear tiling.
	pub linear_tiling_features: FormatFeatures,

	/// Features available for images with optimal tiling.
	pub optimal_tiling_features: FormatFeatures,

	/// Features available for buffers.
	pub buffer_features: FormatFeatures,
}

macro_rules! formats {
	($($name:ident => $vk:ident [$bdim:expr] [$sz:expr]),+) => (
		/// An enumeration of all the possible formats.
		#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
		#[repr(i32)]
		#[allow(missing_docs)]
		#[allow(non_camel_case_types)]
		pub enum Format {
			$($name = vk::Format::$vk.as_raw(),)+
		}

		impl Format {
			/// Returns the size in bytes of an element of this format.
			/// For block based formats this will be the size of a single block.
			/// Returns `None` if the size is irrelevant.
			#[inline]
			pub fn size(&self) -> Option<usize> {
				match *self {
					$(
						Format::$name => $sz,
					)+
				}
			}

			/// Returns (width, heigh) of the dimensions for block based formats.
			/// For non block formats will return (1,1)
			#[inline]
			pub fn block_dimensions(&self) -> (u32, u32) {
				match *self {
					$(
						Format::$name => $bdim,
					)+
				}
			}

			/// Returns the `Format` corresponding to a Vulkan constant.
			pub(crate) fn from_vulkan(val: vk::Format) -> Option<Format> {
				match val {
					$(
						vk::Format::$vk => Some(Format::$name),
					)+
					_ => None,
				}
			}

			/// Returns the Vulkan constant corresponding to the `Format`.
			pub(crate) fn into_vulkan(self) -> vk::Format {
				vk::Format::from_raw(self as i32)
			}

			/// Retrieves the properties of a format when used by a certain device.
			#[inline]
			pub fn properties(&self, device: PhysicalDevice) -> FormatProperties {
				let vk_properties = unsafe {
					device.instance().handle.get_physical_device_format_properties(
						device.handle(),
						self.into_vulkan()
					)
				};

				FormatProperties {
					linear_tiling_features: FormatFeatures::from_vulkan(vk_properties.linear_tiling_features),
					optimal_tiling_features: FormatFeatures::from_vulkan(vk_properties.optimal_tiling_features),
					buffer_features: FormatFeatures::from_vulkan(vk_properties.buffer_features),
				}
			}
		}
	);
}

formats! {
	R4G4UnormPack8 => R4G4_UNORM_PACK8 [(1, 1)] [Some(1)],
	R4G4B4A4UnormPack16 => R4G4B4A4_UNORM_PACK16 [(1, 1)] [Some(2)],
	B4G4R4A4UnormPack16 => B4G4R4A4_UNORM_PACK16 [(1, 1)] [Some(2)],
	R5G6B5UnormPack16 => R5G6B5_UNORM_PACK16 [(1, 1)] [Some(2)],
	B5G6R5UnormPack16 => B5G6R5_UNORM_PACK16 [(1, 1)] [Some(2)],
	R5G5B5A1UnormPack16 => R5G5B5A1_UNORM_PACK16 [(1, 1)] [Some(2)],
	B5G5R5A1UnormPack16 => B5G5R5A1_UNORM_PACK16 [(1, 1)] [Some(2)],
	A1R5G5B5UnormPack16 => A1R5G5B5_UNORM_PACK16 [(1, 1)] [Some(2)],
	R8Unorm => R8_UNORM [(1, 1)] [Some(1)],
	R8Snorm => R8_SNORM [(1, 1)] [Some(1)],
	R8Uscaled => R8_USCALED [(1, 1)] [Some(1)],
	R8Sscaled => R8_SSCALED [(1, 1)] [Some(1)],
	R8Uint => R8_UINT [(1, 1)] [Some(1)],
	R8Sint => R8_SINT [(1, 1)] [Some(1)],
	R8Srgb => R8_SRGB [(1, 1)] [Some(1)],
	R8G8Unorm => R8G8_UNORM [(1, 1)] [Some(2)],
	R8G8Snorm => R8G8_SNORM [(1, 1)] [Some(2)],
	R8G8Uscaled => R8G8_USCALED [(1, 1)] [Some(2)],
	R8G8Sscaled => R8G8_SSCALED [(1, 1)] [Some(2)],
	R8G8Uint => R8G8_UINT [(1, 1)] [Some(2)],
	R8G8Sint => R8G8_SINT [(1, 1)] [Some(2)],
	R8G8Srgb => R8G8_SRGB [(1, 1)] [Some(2)],
	R8G8B8Unorm => R8G8B8_UNORM [(1, 1)] [Some(3)],
	R8G8B8Snorm => R8G8B8_SNORM [(1, 1)] [Some(3)],
	R8G8B8Uscaled => R8G8B8_USCALED [(1, 1)] [Some(3)],
	R8G8B8Sscaled => R8G8B8_SSCALED [(1, 1)] [Some(3)],
	R8G8B8Uint => R8G8B8_UINT [(1, 1)] [Some(3)],
	R8G8B8Sint => R8G8B8_SINT [(1, 1)] [Some(3)],
	R8G8B8Srgb => R8G8B8_SRGB [(1, 1)] [Some(3)],
	B8G8R8Unorm => B8G8R8_UNORM [(1, 1)] [Some(3)],
	B8G8R8Snorm => B8G8R8_SNORM [(1, 1)] [Some(3)],
	B8G8R8Uscaled => B8G8R8_USCALED [(1, 1)] [Some(3)],
	B8G8R8Sscaled => B8G8R8_SSCALED [(1, 1)] [Some(3)],
	B8G8R8Uint => B8G8R8_UINT [(1, 1)] [Some(3)],
	B8G8R8Sint => B8G8R8_SINT [(1, 1)] [Some(3)],
	B8G8R8Srgb => B8G8R8_SRGB [(1, 1)] [Some(3)],
	R8G8B8A8Unorm => R8G8B8A8_UNORM [(1, 1)] [Some(4)],
	R8G8B8A8Snorm => R8G8B8A8_SNORM [(1, 1)] [Some(4)],
	R8G8B8A8Uscaled => R8G8B8A8_USCALED [(1, 1)] [Some(4)],
	R8G8B8A8Sscaled => R8G8B8A8_SSCALED [(1, 1)] [Some(4)],
	R8G8B8A8Uint => R8G8B8A8_UINT [(1, 1)] [Some(4)],
	R8G8B8A8Sint => R8G8B8A8_SINT [(1, 1)] [Some(4)],
	R8G8B8A8Srgb => R8G8B8A8_SRGB [(1, 1)] [Some(4)],
	B8G8R8A8Unorm => B8G8R8A8_UNORM [(1, 1)] [Some(4)],
	B8G8R8A8Snorm => B8G8R8A8_SNORM [(1, 1)] [Some(4)],
	B8G8R8A8Uscaled => B8G8R8A8_USCALED [(1, 1)] [Some(4)],
	B8G8R8A8Sscaled => B8G8R8A8_SSCALED [(1, 1)] [Some(4)],
	B8G8R8A8Uint => B8G8R8A8_UINT [(1, 1)] [Some(4)],
	B8G8R8A8Sint => B8G8R8A8_SINT [(1, 1)] [Some(4)],
	B8G8R8A8Srgb => B8G8R8A8_SRGB [(1, 1)] [Some(4)],
	A8B8G8R8UnormPack32 => A8B8G8R8_UNORM_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8SnormPack32 => A8B8G8R8_SNORM_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8UscaledPack32 => A8B8G8R8_USCALED_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8SscaledPack32 => A8B8G8R8_SSCALED_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8UintPack32 => A8B8G8R8_UINT_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8SintPack32 => A8B8G8R8_SINT_PACK32 [(1, 1)] [Some(4)],
	A8B8G8R8SrgbPack32 => A8B8G8R8_SRGB_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10UnormPack32 => A2R10G10B10_UNORM_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10SnormPack32 => A2R10G10B10_SNORM_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10UscaledPack32 => A2R10G10B10_USCALED_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10SscaledPack32 => A2R10G10B10_SSCALED_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10UintPack32 => A2R10G10B10_UINT_PACK32 [(1, 1)] [Some(4)],
	A2R10G10B10SintPack32 => A2R10G10B10_SINT_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10UnormPack32 => A2B10G10R10_UNORM_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10SnormPack32 => A2B10G10R10_SNORM_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10UscaledPack32 => A2B10G10R10_USCALED_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10SscaledPack32 => A2B10G10R10_SSCALED_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10UintPack32 => A2B10G10R10_UINT_PACK32 [(1, 1)] [Some(4)],
	A2B10G10R10SintPack32 => A2B10G10R10_SINT_PACK32 [(1, 1)] [Some(4)],
	R16Unorm => R16_UNORM [(1, 1)] [Some(2)],
	R16Snorm => R16_SNORM [(1, 1)] [Some(2)],
	R16Uscaled => R16_USCALED [(1, 1)] [Some(2)],
	R16Sscaled => R16_SSCALED [(1, 1)] [Some(2)],
	R16Uint => R16_UINT [(1, 1)] [Some(2)],
	R16Sint => R16_SINT [(1, 1)] [Some(2)],
	R16Sfloat => R16_SFLOAT [(1, 1)] [Some(2)],
	R16G16Unorm => R16G16_UNORM [(1, 1)] [Some(4)],
	R16G16Snorm => R16G16_SNORM [(1, 1)] [Some(4)],
	R16G16Uscaled => R16G16_USCALED [(1, 1)] [Some(4)],
	R16G16Sscaled => R16G16_SSCALED [(1, 1)] [Some(4)],
	R16G16Uint => R16G16_UINT [(1, 1)] [Some(4)],
	R16G16Sint => R16G16_SINT [(1, 1)] [Some(4)],
	R16G16Sfloat => R16G16_SFLOAT [(1, 1)] [Some(4)],
	R16G16B16Unorm => R16G16B16_UNORM [(1, 1)] [Some(6)],
	R16G16B16Snorm => R16G16B16_SNORM [(1, 1)] [Some(6)],
	R16G16B16Uscaled => R16G16B16_USCALED [(1, 1)] [Some(6)],
	R16G16B16Sscaled => R16G16B16_SSCALED [(1, 1)] [Some(6)],
	R16G16B16Uint => R16G16B16_UINT [(1, 1)] [Some(6)],
	R16G16B16Sint => R16G16B16_SINT [(1, 1)] [Some(6)],
	R16G16B16Sfloat => R16G16B16_SFLOAT [(1, 1)] [Some(6)],
	R16G16B16A16Unorm => R16G16B16A16_UNORM [(1, 1)] [Some(8)],
	R16G16B16A16Snorm => R16G16B16A16_SNORM [(1, 1)] [Some(8)],
	R16G16B16A16Uscaled => R16G16B16A16_USCALED [(1, 1)] [Some(8)],
	R16G16B16A16Sscaled => R16G16B16A16_SSCALED [(1, 1)] [Some(8)],
	R16G16B16A16Uint => R16G16B16A16_UINT [(1, 1)] [Some(8)],
	R16G16B16A16Sint => R16G16B16A16_SINT [(1, 1)] [Some(8)],
	R16G16B16A16Sfloat => R16G16B16A16_SFLOAT [(1, 1)] [Some(8)],
	R32Uint => R32_UINT [(1, 1)] [Some(4)],
	R32Sint => R32_SINT [(1, 1)] [Some(4)],
	R32Sfloat => R32_SFLOAT [(1, 1)] [Some(4)],
	R32G32Uint => R32G32_UINT [(1, 1)] [Some(8)],
	R32G32Sint => R32G32_SINT [(1, 1)] [Some(8)],
	R32G32Sfloat => R32G32_SFLOAT [(1, 1)] [Some(8)],
	R32G32B32Uint => R32G32B32_UINT [(1, 1)] [Some(12)],
	R32G32B32Sint => R32G32B32_SINT [(1, 1)] [Some(12)],
	R32G32B32Sfloat => R32G32B32_SFLOAT [(1, 1)] [Some(12)],
	R32G32B32A32Uint => R32G32B32A32_UINT [(1, 1)] [Some(16)],
	R32G32B32A32Sint => R32G32B32A32_SINT [(1, 1)] [Some(16)],
	R32G32B32A32Sfloat => R32G32B32A32_SFLOAT [(1, 1)] [Some(16)],
	R64Uint => R64_UINT [(1, 1)] [Some(8)],
	R64Sint => R64_SINT [(1, 1)] [Some(8)],
	R64Sfloat => R64_SFLOAT [(1, 1)] [Some(8)],
	R64G64Uint => R64G64_UINT [(1, 1)] [Some(16)],
	R64G64Sint => R64G64_SINT [(1, 1)] [Some(16)],
	R64G64Sfloat => R64G64_SFLOAT [(1, 1)] [Some(16)],
	R64G64B64Uint => R64G64B64_UINT [(1, 1)] [Some(24)],
	R64G64B64Sint => R64G64B64_SINT [(1, 1)] [Some(24)],
	R64G64B64Sfloat => R64G64B64_SFLOAT [(1, 1)] [Some(24)],
	R64G64B64A64Uint => R64G64B64A64_UINT [(1, 1)] [Some(32)],
	R64G64B64A64Sint => R64G64B64A64_SINT [(1, 1)] [Some(32)],
	R64G64B64A64Sfloat => R64G64B64A64_SFLOAT [(1, 1)] [Some(32)],
	B10G11R11UfloatPack32 => B10G11R11_UFLOAT_PACK32 [(1, 1)] [Some(4)],
	E5B9G9R9UfloatPack32 => E5B9G9R9_UFLOAT_PACK32 [(1, 1)] [Some(4)],
	D16Unorm => D16_UNORM [(1, 1)] [Some(2)],
	X8_D24UnormPack32 => X8_D24_UNORM_PACK32 [(1, 1)] [Some(4)],
	D32Sfloat => D32_SFLOAT [(1, 1)] [Some(4)],
	S8Uint => S8_UINT [(1, 1)] [Some(1)],
	D16Unorm_S8Uint => D16_UNORM_S8_UINT [(1, 1)] [None],
	D24Unorm_S8Uint => D24_UNORM_S8_UINT [(1, 1)] [None],
	D32Sfloat_S8Uint => D32_SFLOAT_S8_UINT [(1, 1)] [None],
	BC1_RGBUnormBlock => BC1_RGB_UNORM_BLOCK [(4, 4)] [Some(8)],
	BC1_RGBSrgbBlock => BC1_RGB_SRGB_BLOCK [(4, 4)] [Some(8)],
	BC1_RGBAUnormBlock => BC1_RGBA_UNORM_BLOCK [(4, 4)] [Some(8)],
	BC1_RGBASrgbBlock => BC1_RGBA_SRGB_BLOCK [(4, 4)] [Some(8)],
	BC2UnormBlock => BC2_UNORM_BLOCK [(4, 4)] [Some(16)],
	BC2SrgbBlock => BC2_SRGB_BLOCK [(4, 4)] [Some(16)],
	BC3UnormBlock => BC3_UNORM_BLOCK [(4, 4)] [Some(16)],
	BC3SrgbBlock => BC3_SRGB_BLOCK [(4, 4)] [Some(16)],
	BC4UnormBlock => BC4_UNORM_BLOCK [(4, 4)] [Some(8)],
	BC4SnormBlock => BC4_SNORM_BLOCK [(4, 4)] [Some(8)],
	BC5UnormBlock => BC5_UNORM_BLOCK [(4, 4)] [Some(16)],
	BC5SnormBlock => BC5_SNORM_BLOCK [(4, 4)] [Some(16)],
	BC6HUfloatBlock => BC6H_UFLOAT_BLOCK [(4, 4)] [Some(16)],
	BC6HSfloatBlock => BC6H_SFLOAT_BLOCK [(4, 4)] [Some(16)],
	BC7UnormBlock => BC7_UNORM_BLOCK [(4, 4)] [Some(16)],
	BC7SrgbBlock => BC7_SRGB_BLOCK [(4, 4)] [Some(16)],
	ETC2_R8G8B8UnormBlock => ETC2_R8G8B8_UNORM_BLOCK [(4, 4)] [Some(8)],
	ETC2_R8G8B8SrgbBlock => ETC2_R8G8B8_SRGB_BLOCK [(4, 4)] [Some(8)],
	ETC2_R8G8B8A1UnormBlock => ETC2_R8G8B8A1_UNORM_BLOCK [(4, 4)] [Some(8)],
	ETC2_R8G8B8A1SrgbBlock => ETC2_R8G8B8A1_SRGB_BLOCK [(4, 4)] [Some(8)],
	ETC2_R8G8B8A8UnormBlock => ETC2_R8G8B8A8_UNORM_BLOCK [(4, 4)] [Some(16)],
	ETC2_R8G8B8A8SrgbBlock => ETC2_R8G8B8A8_SRGB_BLOCK [(4, 4)] [Some(16)],
	EAC_R11UnormBlock => EAC_R11_UNORM_BLOCK [(4, 4)] [Some(8)],
	EAC_R11SnormBlock => EAC_R11_SNORM_BLOCK [(4, 4)] [Some(8)],
	EAC_R11G11UnormBlock => EAC_R11G11_UNORM_BLOCK [(4, 4)] [Some(16)],
	EAC_R11G11SnormBlock => EAC_R11G11_SNORM_BLOCK [(4, 4)] [Some(16)],
	ASTC_4x4UnormBlock => ASTC_4X4_UNORM_BLOCK [(4, 4)] [Some(16)],
	ASTC_4x4SrgbBlock => ASTC_4X4_SRGB_BLOCK [(4, 4)] [Some(16)],
	ASTC_5x4UnormBlock => ASTC_5X4_UNORM_BLOCK [(5, 4)] [Some(16)],
	ASTC_5x4SrgbBlock => ASTC_5X4_SRGB_BLOCK [(5, 4)] [Some(16)],
	ASTC_5x5UnormBlock => ASTC_5X5_UNORM_BLOCK [(5, 5)] [Some(16)],
	ASTC_5x5SrgbBlock => ASTC_5X5_SRGB_BLOCK [(5, 5)] [Some(16)],
	ASTC_6x5UnormBlock => ASTC_6X5_UNORM_BLOCK [(6, 5)] [Some(16)],
	ASTC_6x5SrgbBlock => ASTC_6X5_SRGB_BLOCK [(6, 5)] [Some(16)],
	ASTC_6x6UnormBlock => ASTC_6X6_UNORM_BLOCK [(6, 6)] [Some(16)],
	ASTC_6x6SrgbBlock => ASTC_6X6_SRGB_BLOCK [(6, 6)] [Some(16)],
	ASTC_8x5UnormBlock => ASTC_8X5_UNORM_BLOCK [(8, 5)] [Some(16)],
	ASTC_8x5SrgbBlock => ASTC_8X5_SRGB_BLOCK [(8, 5)] [Some(16)],
	ASTC_8x6UnormBlock => ASTC_8X6_UNORM_BLOCK [(8, 6)] [Some(16)],
	ASTC_8x6SrgbBlock => ASTC_8X6_SRGB_BLOCK [(8, 6)] [Some(16)],
	ASTC_8x8UnormBlock => ASTC_8X8_UNORM_BLOCK [(8, 8)] [Some(16)],
	ASTC_8x8SrgbBlock => ASTC_8X8_SRGB_BLOCK [(8, 8)] [Some(16)],
	ASTC_10x5UnormBlock => ASTC_10X5_UNORM_BLOCK [(10, 5)] [Some(16)],
	ASTC_10x5SrgbBlock => ASTC_10X5_SRGB_BLOCK [(10, 5)] [Some(16)],
	ASTC_10x6UnormBlock => ASTC_10X6_UNORM_BLOCK [(10, 6)] [Some(16)],
	ASTC_10x6SrgbBlock => ASTC_10X6_SRGB_BLOCK [(10, 6)] [Some(16)],
	ASTC_10x8UnormBlock => ASTC_10X8_UNORM_BLOCK [(10, 8)] [Some(16)],
	ASTC_10x8SrgbBlock => ASTC_10X8_SRGB_BLOCK [(10, 8)] [Some(16)],
	ASTC_10x10UnormBlock => ASTC_10X10_UNORM_BLOCK [(10, 10)] [Some(16)],
	ASTC_10x10SrgbBlock => ASTC_10X10_SRGB_BLOCK [(10, 10)] [Some(16)],
	ASTC_12x10UnormBlock => ASTC_12X10_UNORM_BLOCK [(12, 10)] [Some(16)],
	ASTC_12x10SrgbBlock => ASTC_12X10_SRGB_BLOCK [(12, 10)] [Some(16)],
	ASTC_12x12UnormBlock => ASTC_12X12_UNORM_BLOCK [(12, 12)] [Some(16)],
	ASTC_12x12SrgbBlock => ASTC_12X12_SRGB_BLOCK [(12, 12)] [Some(16)]
}

/// The features supported by images with a particular format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
pub struct FormatFeatures {
	pub sampled_image: bool,
	pub storage_image: bool,
	pub storage_image_atomic: bool,
	pub uniform_texel_buffer: bool,
	pub storage_texel_buffer: bool,
	pub storage_texel_buffer_atomic: bool,
	pub vertex_buffer: bool,
	pub color_attachment: bool,
	pub color_attachment_blend: bool,
	pub depth_stencil_attachment: bool,
	pub blit_src: bool,
	pub blit_dst: bool,
	pub sampled_image_filter_linear: bool,
	pub transfer_src: bool,
	pub transfer_dst: bool,
	pub midpoint_chroma_samples: bool,
	pub sampled_image_ycbcr_conversion_linear_filter: bool,
	pub sampled_image_ycbcr_conversion_separate_reconstruction_filter: bool,
	pub sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: bool,
	pub sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: bool,
	pub disjoint: bool,
	pub cosited_chroma_samples: bool,
	pub sampled_image_filter_minmax: bool,
	pub img_sampled_image_filter_cubic: bool,
	pub khr_acceleration_structure_vertex_buffer: bool,
	pub ext_fragment_density_map: bool,
}

impl FormatFeatures {
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn from_vulkan(val: vk::FormatFeatureFlags) -> FormatFeatures {
		FormatFeatures {
			sampled_image: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE),
			storage_image: val.contains(vk::FormatFeatureFlags::STORAGE_IMAGE),
			storage_image_atomic: val.contains(vk::FormatFeatureFlags::STORAGE_IMAGE_ATOMIC),
			uniform_texel_buffer: val.contains(vk::FormatFeatureFlags::UNIFORM_TEXEL_BUFFER),
			storage_texel_buffer: val.contains(vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER),
			storage_texel_buffer_atomic: val.contains(vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER_ATOMIC),
			vertex_buffer: val.contains(vk::FormatFeatureFlags::VERTEX_BUFFER),
			color_attachment: val.contains(vk::FormatFeatureFlags::COLOR_ATTACHMENT),
			color_attachment_blend: val.contains(vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND),
			depth_stencil_attachment: val.contains(vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT),
			blit_src: val.contains(vk::FormatFeatureFlags::BLIT_SRC),
			blit_dst: val.contains(vk::FormatFeatureFlags::BLIT_DST),
			sampled_image_filter_linear: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR),
			transfer_src: val.contains(vk::FormatFeatureFlags::TRANSFER_SRC),
			transfer_dst: val.contains(vk::FormatFeatureFlags::TRANSFER_DST),
			midpoint_chroma_samples: val.contains(vk::FormatFeatureFlags::MIDPOINT_CHROMA_SAMPLES),
			sampled_image_ycbcr_conversion_linear_filter: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER),
			sampled_image_ycbcr_conversion_separate_reconstruction_filter: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER),
			sampled_image_ycbcr_conversion_chroma_reconstruction_explicit: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT),
			sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE),
			disjoint: val.contains(vk::FormatFeatureFlags::DISJOINT),
			cosited_chroma_samples: val.contains(vk::FormatFeatureFlags::COSITED_CHROMA_SAMPLES),
			sampled_image_filter_minmax: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_MINMAX),
			img_sampled_image_filter_cubic: val.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_CUBIC_IMG),
			khr_acceleration_structure_vertex_buffer: val.contains(vk::FormatFeatureFlags::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR),
			ext_fragment_density_map: val.contains(vk::FormatFeatureFlags::FRAGMENT_DENSITY_MAP_EXT),
		}
	}
}