use ash::vk;

// Sharing mode.
pub enum SharingMode<Q> where Q: Iterator<Item = u32> {
	// The resource is used is only one queue family.
	Exclusive,

	// The resource is used in multiple queue families.
	// Can be slower than `Exclusive`.
	Concurrent(Q)
}

impl<Q: Iterator<Item=u32>> SharingMode<Q> {
	pub fn vulkan_sharing_mode(&self) -> vk::SharingMode {
		match self {
			SharingMode::Exclusive => vk::SharingMode::EXCLUSIVE,
			SharingMode::Concurrent(_) => vk::SharingMode::CONCURRENT
		}
	}
}