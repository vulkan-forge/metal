use ash::vk;

pub struct MemoryRequirements(pub(crate) vk::MemoryRequirements);

impl MemoryRequirements {
	#[inline]
	pub fn size(&self) -> u64 {
		self.0.size
	}

	#[inline]
	pub fn alignment(&self) -> u64 {
		self.0.alignment
	}

	#[inline]
	pub fn memory_type_bits(&self) -> u32 {
		self.0.memory_type_bits
	}

	#[inline]
	pub fn contains_memory_type_index(&self, index: u32) -> bool {
		self.0.memory_type_bits & (1u32 << index) != 0
	}
}
