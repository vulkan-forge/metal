use std::sync::Arc;
use crate::{
	Device,
	DeviceOwned
};
use super::{
	Error,
	MemoryRequirements,
	HostVisible
};

/// Allocator only allocating memory to host visible memory.
pub struct Allocator<A: super::Allocator> {
	/// Underlying allocator.
	allocator: A
}

impl<A: super::Allocator> Allocator<A> {
	pub fn filtered_memory_requirements(&self, memory_requirements: MemoryRequirements) -> MemoryRequirements {
		memory_requirements.filter_memory_types(self.device().physical_device(), |memory_type| memory_type.is_host_visible())
	}

	pub fn allocate(&self, memory_requirements: MemoryRequirements) -> Result<HostVisible<A::Slot>, Error> {
		let memory_requirements = self.filtered_memory_requirements(memory_requirements);
		Ok(HostVisible::try_from(self.allocator.allocate(memory_requirements)?).ok().unwrap())
	}
}

unsafe impl<A: super::Allocator> super::Allocator for Allocator<A> {
	type Slot = A::Slot;

	fn prepare(&self, memory_requirements: MemoryRequirements) {
		let memory_requirements = self.filtered_memory_requirements(memory_requirements);
		self.allocator.prepare(memory_requirements)
	}

	/// Allocate some memory.
	fn allocate(&self, memory_requirements: MemoryRequirements) -> Result<Self::Slot, Error> {
		Ok(self.allocate(memory_requirements)?.unwrap())
	}

	/// Reallocate host-visible memory.
	fn reallocate(&self, slot: HostVisible<Self::Slot>, memory_requirements: MemoryRequirements) -> Result<HostVisible<Self::Slot>, Error> {
		self.allocator.reallocate(slot, memory_requirements)
	}
}

impl<A: super::Allocator> DeviceOwned for Allocator<A> {
	fn device(&self) -> &Arc<Device> {
		self.allocator.device()
	}
}