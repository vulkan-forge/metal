use ash::vk;
use crate::mem::MemoryRequirements;
use super::PhysicalDevice;

pub struct MemoryType<'a> {
	physical_device: PhysicalDevice<'a>,
	index: u32,
	memory_type: vk::MemoryType
}

impl<'a> MemoryType<'a> {
	#[inline]
	pub(crate) fn new(physical_device: PhysicalDevice<'a>, index: u32) -> MemoryType<'a> {
		MemoryType {
			physical_device,
			index,
			memory_type: physical_device.p.memory_properties.memory_types[index as usize]
		}
	}

	#[inline]
	pub fn index(&self) -> u32 {
		self.index
	}

	#[inline]
	pub fn satisfies(&self, reqs: &MemoryRequirements) -> bool {
		reqs.contains_memory_type_index(self.index)
	}

	/// Returns true if the memory type is located on the device.
	/// 
	/// This is the most efficient for GPU accesses.
	#[inline]
	pub fn is_device_local(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
	}

	/// Returns true if the memory type can be accessed by the host.
	#[inline]
	pub fn is_host_visible(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
	}

	/// Returns `true` if modifications made by the host or the GPU on this memory type are instantaneously visible to the other party.
	/// Otherwise, changes have to be flushed.
	#[inline]
	pub fn is_host_coherent(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
	}

	/// Returns true if memory of this memory type is cached by the host.
	/// 
	/// Host memory accesses to cached memory is faster than for uncached memory.
	/// However you are not guaranteed that it is coherent.
	/// 
	/// Host memory accesses to uncached memory are slower than to cached memory,
	/// however uncached memory is always host coherent.
	#[inline]
	pub fn is_host_cached(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_CACHED)
	}

	/// Returns true if allocations made to this memory type is lazy.
	/// 
	/// This means that no actual allocation is performed.
	/// Instead memory is automatically allocated by the Vulkan implementation.
	/// 
	/// Memory of this type can only be used on images created with a certain flag.
	/// Memory of this type is never host-visible.
	#[inline]
	pub fn is_lazily_allocated(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::LAZILY_ALLOCATED)
	}
}
