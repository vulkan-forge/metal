use std::{
	ffi::c_void
};
use crate::{
	DeviceOwned,
	device
};

pub mod buffer;
mod memory_requirements;

pub use buffer::{
	Buffer,
	TypedBuffer,
	IndexBuffer,
	Buffers
};
pub use memory_requirements::MemoryRequirements;

#[derive(Debug)]
pub enum Error {
	OutOfMemory,
	Map(device::memory::MapError),
	Allocation(device::AllocationError)
}

impl From<device::memory::MapError> for Error {
	fn from(e: device::memory::MapError) -> Self {
		Self::Map(e)
	}
}

impl From<device::AllocationError> for Error {
	fn from(e: device::AllocationError) -> Self {
		Self::Allocation(e)
	}
}

pub unsafe trait Allocator: 'static + DeviceOwned {
	type Slot: Slot;

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
	fn allocate(&mut self, memory_requirements: MemoryRequirements) -> Result<Self::Slot, Error>;
}

pub unsafe trait Slot: 'static {
	fn memory(&self) -> &device::Memory;

	fn offset(&self) -> u64;

	fn size(&self) -> u64;

	fn ptr(&self) -> Option<*mut c_void>;
}

pub struct HostVisible<S: Slot>(S);

impl<S: Slot> HostVisible<S> {
	#[inline]
	pub fn try_from(s: S) -> Result<Self, S> {
		if s.ptr().is_some() {
			Ok(HostVisible(s))
		} else {
			Err(s)
		}
	}

	#[inline]
	pub fn ptr(&self) -> *mut c_void {
		self.0.ptr().unwrap()
	}
}

unsafe impl<S: Slot> Slot for HostVisible<S> {
	#[inline]
	fn memory(&self) -> &device::Memory {
		self.0.memory()
	}

	#[inline]
	fn offset(&self) -> u64 {
		self.0.offset()
	}

	#[inline]
	fn size(&self) -> u64 {
		self.0.size()
	}

	#[inline]
	fn ptr(&self) -> Option<*mut c_void> {
		self.0.ptr()
	}
}