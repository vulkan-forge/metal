use ash::vk;
use crate::buffer;
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
	pub fn satisfies(&self, reqs: &buffer::MemoryRequirements) -> bool {
		reqs.contains_memory_type_index(self.index)
	}

	#[inline]
	pub fn is_device_local(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
	}

	#[inline]
	pub fn is_host_visible(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
	}

	#[inline]
	pub fn is_host_coherent(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
	}

	#[inline]
	pub fn is_host_cached(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::HOST_CACHED)
	}

	#[inline]
	pub fn is_lazily_alocated(&self) -> bool {
		self.memory_type.property_flags.contains(vk::MemoryPropertyFlags::LAZILY_ALLOCATED)
	}
}
