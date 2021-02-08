use ash::vk;

/// How the presentation engine should interpret the data.
///
/// # A quick lesson about color spaces
///
/// ## What is a color space?
///
/// Each pixel of a monitor is made of three components: one red, one green, and one blue. In the
/// past, computers would simply send to the monitor the intensity of each of the three components.
///
/// This proved to be problematic, because depending on the brand of the monitor the colors would
/// not exactly be the same. For example on some monitors, a value of `[1.0, 0.0, 0.0]` would be a
/// bit more orange than on others.
///
/// In order to standardize this, there exist what are called *color spaces*: sRGB, AdobeRGB,
/// DCI-P3, scRGB, etc. When you manipulate RGB values in a specific color space, these values have
/// a precise absolute meaning in terms of color, that is the same across all systems and monitors.
///
/// > **Note**: Color spaces are orthogonal to concept of RGB. *RGB* only indicates what is the
/// > representation of the data, but not how it is interpreted. You can think of this a bit like
/// > text encoding. An *RGB* value is a like a byte, in other words it is the medium by which
/// > values are communicated, and a *color space* is like a text encoding (eg. UTF-8), in other
/// > words it is the way the value should be interpreted.
///
/// The most commonly used color space today is sRGB. Most monitors today use this color space,
/// and most images files are encoded in this color space.
///
/// ## Pixel formats and linear vs non-linear
///
/// In Vulkan all images have a specific format in which the data is stored. The data of an image
/// consists of pixels in RGB but contains no information about the color space (or lack thereof)
/// of these pixels. You are free to store them in whatever color space you want.
///
/// But one big practical problem with color spaces is that they are sometimes not linear, and in
/// particular the popular sRGB color space is not linear. In a non-linear color space, a value of
/// `[0.6, 0.6, 0.6]` for example is **not** twice as bright as a value of `[0.3, 0.3, 0.3]`. This
/// is problematic, because operations such as taking the average of two colors or calculating the
/// lighting of a texture with a dot product are mathematically incorrect and will produce
/// incorrect colors.
///
/// > **Note**: If the texture format has an alpha component, it is not affected by the color space
/// > and always behaves linearly.
///
/// In order to solve this Vulkan also provides image formats with the `Srgb` suffix, which are
/// expected to contain RGB data in the sRGB color space. When you sample an image with such a
/// format from a shader, the implementation will automatically turn the pixel values into a linear
/// color space that is suitable for linear operations (such as additions or multiplications).
/// When you write to a framebuffer attachment with such a format, the implementation will
/// automatically perform the opposite conversion. These conversions are most of the time performed
/// by the hardware and incur no additional cost.
///
/// ## Color space of the swapchain
///
/// The color space that you specify when you create a swapchain is how the implementation will
/// interpret the raw data inside of the image.
///
/// > **Note**: The implementation can choose to send the data in the swapchain image directly to
/// > the monitor, but it can also choose to write it in an intermediary buffer that is then read
/// > by the operating system or windowing system. Therefore the color space that the
/// > implementation supports is not necessarily the same as the one supported by the monitor.
///
/// It is *your* job to ensure that the data in the swapchain image is in the color space
/// that is specified here, otherwise colors will be incorrect.
/// The implementation will never perform any additional automatic conversion after the colors have
/// been written to the swapchain image.
///
/// # How do I handle this correctly?
///
/// The easiest way to handle color spaces in a cross-platform program is:
///
/// - Always request the `SrgbNonLinear` color space when creating the swapchain.
/// - Make sure that all your image files use the sRGB color space, and load them in images whose
///   format has the `Srgb` suffix. Only use non-sRGB image formats for intermediary computations
///   or to store non-color data.
/// - Swapchain images should have a format with the `Srgb` suffix.
///
/// > **Note**: It is unclear whether the `SrgbNonLinear` color space is always supported by the
/// > the implementation or not. See https://github.com/KhronosGroup/Vulkan-Docs/issues/442.
///
/// > **Note**: Lots of developers are confused by color spaces. You can sometimes find articles
/// > talking about gamma correction and suggestion to put your colors to the power 2.2 for
/// > example. These are all hacks and you should use the sRGB pixel formats instead.
///
/// If you follow these three rules, then everything should render the same way on all platforms.
///
/// Additionally you can try detect whether the implementation supports any additional color space
/// and perform a manual conversion to that color space from inside your shader.
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpace {
	SrgbNonLinear = vk::ColorSpaceKHR::SRGB_NONLINEAR.as_raw(),
	DisplayP3NonLinear = vk::ColorSpaceKHR::DISPLAY_P3_NONLINEAR_EXT.as_raw(),
	ExtendedSrgbLinear = vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT.as_raw(),
	DciP3Linear = vk::ColorSpaceKHR::DCI_P3_LINEAR_EXT.as_raw(),
	DciP3NonLinear = vk::ColorSpaceKHR::DCI_P3_NONLINEAR_EXT.as_raw(),
	Bt709Linear = vk::ColorSpaceKHR::BT709_LINEAR_EXT.as_raw(),
	Bt709NonLinear = vk::ColorSpaceKHR::BT709_NONLINEAR_EXT.as_raw(),
	Bt2020Linear = vk::ColorSpaceKHR::BT2020_LINEAR_EXT.as_raw(),
	Hdr10St2084 = vk::ColorSpaceKHR::HDR10_ST2084_EXT.as_raw(),
	DolbyVision = vk::ColorSpaceKHR::DOLBYVISION_EXT.as_raw(),
	Hdr10Hlg = vk::ColorSpaceKHR::HDR10_HLG_EXT.as_raw(),
	AdobeRgbLinear = vk::ColorSpaceKHR::ADOBERGB_LINEAR_EXT.as_raw(),
	AdobeRgbNonLinear = vk::ColorSpaceKHR::ADOBERGB_NONLINEAR_EXT.as_raw(),
	PassThrough = vk::ColorSpaceKHR::PASS_THROUGH_EXT.as_raw(),
}

impl ColorSpace {
	#[inline]
	pub(crate) fn from_vulkan(val: vk::ColorSpaceKHR) -> Self {
		match val {
			vk::ColorSpaceKHR::SRGB_NONLINEAR => ColorSpace::SrgbNonLinear,
			vk::ColorSpaceKHR::DISPLAY_P3_NONLINEAR_EXT => ColorSpace::DisplayP3NonLinear,
			vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT => ColorSpace::ExtendedSrgbLinear,
			vk::ColorSpaceKHR::DCI_P3_LINEAR_EXT => ColorSpace::DciP3Linear,
			vk::ColorSpaceKHR::DCI_P3_NONLINEAR_EXT => ColorSpace::DciP3NonLinear,
			vk::ColorSpaceKHR::BT709_LINEAR_EXT => ColorSpace::Bt709Linear,
			vk::ColorSpaceKHR::BT709_NONLINEAR_EXT => ColorSpace::Bt709NonLinear,
			vk::ColorSpaceKHR::BT2020_LINEAR_EXT => ColorSpace::Bt2020Linear,
			vk::ColorSpaceKHR::HDR10_ST2084_EXT => ColorSpace::Hdr10St2084,
			vk::ColorSpaceKHR::DOLBYVISION_EXT => ColorSpace::DolbyVision,
			vk::ColorSpaceKHR::HDR10_HLG_EXT => ColorSpace::Hdr10Hlg,
			vk::ColorSpaceKHR::ADOBERGB_LINEAR_EXT => ColorSpace::AdobeRgbLinear,
			vk::ColorSpaceKHR::ADOBERGB_NONLINEAR_EXT => ColorSpace::AdobeRgbNonLinear,
			vk::ColorSpaceKHR::PASS_THROUGH_EXT => ColorSpace::PassThrough,
			_ => panic!("Wrong value for color space enum"),
		}
	}

	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::ColorSpaceKHR {
		vk::ColorSpaceKHR::from_raw(self as i32)
	}
}