use std::{
	fmt,
	ffi::CStr
};
use crate::Unbuildable;

extensions! {
	khr_swapchain: KhrSwapchain => b"VK_KHR_swapchain\0",
	khr_display_swapchain: KhrDisplaySwapchain => b"VK_KHR_display_swapchain\0",
	khr_sampler_mirror_clamp_to_edge: KhrSamplerMirrorClampToEdge => b"VK_KHR_sampler_mirror_clamp_to_edge\0",
	khr_maintenance1: KhrMaintenance1 => b"VK_KHR_maintenance1\0",
	khr_get_memory_requirements2: KhrGetMemoryRequirements2 => b"VK_KHR_get_memory_requirements2\0",
	khr_dedicated_allocation: KhrDedicatedAllocation => b"VK_KHR_dedicated_allocation\0",
	khr_incremental_present: KhrIncrementalPresent => b"VK_KHR_incremental_present\0",
	khr_16bit_storage: Khr16bitsStorage => b"VK_KHR_16bit_storage\0",
	khr_storage_buffer_storage_class: KhrStorageBufferStorageClass => b"VK_KHR_storage_buffer_storage_class\0",
	ext_debug_utils: ExtDebugUtils => b"VK_EXT_debug_utils\0",
	khr_multiview: KhrMultiview => b"VK_KHR_multiview\0",
	ext_full_screen_exclusive: ExtFullScreenExclusive => b"VK_EXT_full_screen_exclusive\0",
}
