use std::{
	fmt,
	ffi::CStr
};
use crate::Unbuildable;

extensions! {
	khr_surface: KhrSurface => b"VK_KHR_surface\0",
	khr_display: KhrDisplay => b"VK_KHR_display\0",
	khr_xlib_surface: KhrXlibSurface => b"VK_KHR_xlib_surface\0",
	khr_xcb_surface: KhrXcbSurface => b"VK_KHR_xcb_surface\0",
	khr_wayland_surface: KhrWaylandSurface => b"VK_KHR_wayland_surface\0",
	khr_android_surface: KhrAndroidSurface => b"VK_KHR_android_surface\0",
	khr_win32_surface: KhrWin32Surface => b"VK_KHR_win32_surface\0",
	ext_debug_utils: ExtDebugUtils => b"VK_EXT_debug_utils\0",
	mvk_ios_surface: MkvIosSurface => b"VK_MVK_ios_surface\0",
	mvk_macos_surface: MkvMacosSurface => b"VK_MVK_macos_surface\0",
	nn_vi_surface: NnViSurface => b"VK_NN_vi_surface\0",
	ext_swapchain_colorspace: ExtSwapchainColorspace => b"VK_EXT_swapchain_colorspace\0",
	khr_get_physical_device_properties2: KhrGetPhysicalDeviceproperties2 => b"VK_KHR_get_physical_device_properties2\0",
	khr_get_surface_capabilities2: KhrGetSurfaceCapabilities2 => b"VK_KHR_get_surface_capabilities2\0",
}
