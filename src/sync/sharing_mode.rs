use ash::vk;

// Sharing mode.
pub enum SharingMode {
	// The resource is used is only one queue family.
	Exclusive,

	// The resource is used in multiple queue families.
	// Can be slower than `Exclusive`.
	Concurrent(Vec<u32>)
}

impl SharingMode {
	pub fn vulkan_sharing_mode(&self) -> vk::SharingMode {
		match self {
			SharingMode::Exclusive => vk::SharingMode::EXCLUSIVE,
			SharingMode::Concurrent(_) => vk::SharingMode::CONCURRENT
		}
	}
}