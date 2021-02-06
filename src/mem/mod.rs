pub trait Allocator: DeviceOwned {
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
	fn prepare(&mut self, memory_requirements: buffer::MemoryRequirement);

	/// Allocate some memory.
	fn allocate(&mut self, memory_requirements: buffer::MemoryRequirement) -> device::Memory;
}
