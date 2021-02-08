use std::{
	ffi::c_void,
	convert::TryFrom
};
use crate::{
	DeviceOwned,
	device,
	buffer::MemoryRequirements
};

mod buffer;
pub use buffer::*;

pub unsafe trait Slot {
	fn memory(&self) -> &device::Memory;

	fn offset(&self) -> u64;

	fn size(&self) -> u64;
}

pub unsafe trait HostVisibleSlot: Slot {
	fn ptr(&self) -> Result<Option<*mut c_void>, device::memory::MapError>;
}

pub unsafe trait Allocator: DeviceOwned {
	type Slot: Slot + From<Self::HostVisibleSlot>;
	type HostVisibleSlot: HostVisibleSlot + TryFrom<Self::Slot, Error=Self::Slot>;

	/// Prepare a memory allocation.
	///
	/// It is not required to use this method before the allocation.
	/// However when multiple allocations must be done in a short period of time,
	/// calling this method for each allocation before any actual allocation can help the
	/// allocator better manage the memory.
	///
	/// The typical scenario for this is a loading sequence.
	/// You can build your buffers and images, prepare each allocation and register them in a stack.
	/// Once all allocations are prepared, use `allocate` for each item of the stack.
	fn prepare(&mut self, memory_requirements: MemoryRequirements);

	/// Allocate some memory.
	fn allocate(&mut self, memory_requirements: MemoryRequirements) -> Self::Slot;
}
