// use ash::vk;
// use cc_traits::Iter;
// use std::sync::Arc;
// use crate::device::Queue;

// // Sharing mode.
// pub enum SharingMode {
// 	// The resource is used is only one queue family.
// 	Exclusive,

// 	// The resource is used in multiple queue families.
// 	// Can be slower than `Exclusive`.
// 	Concurrent(Vec<Arc<Queue>>)
// }

// impl<'a, Q> SharingMode<'a, Q> where Q: Iter<'a, Item=Arc<Queue>> {
// 	pub fn vulkan_sharing_mode(&self) -> vk::SharingMode {
// 		match self {
// 			SharingMode::Exclusive => vk::SharingMode::EXCLUSIVE,
// 			SharingMode::Concurrent(_) => vk::SharingMode::CONCURRENT
// 		}
// 	}
// }