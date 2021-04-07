use std::{
	ffi::c_void,
	fmt
};
use crate::{
	DeviceOwned,
	device
};

pub mod buffer;
mod memory_requirements;
pub mod staging;

pub use buffer::{
	Buffers,
	LocalBuffers
};
pub use memory_requirements::MemoryRequirements;

#[derive(Debug)]
pub enum Error {
	OutOfMemory,
	Map(device::memory::MapError),
	Allocation(device::AllocationError)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::OutOfMemory => write!(f, "out of memory."),
			Error::Map(e) => write!(f, "map error: {}", e),
			Error::Allocation(e) => write!(f, "allocation error: {}", e)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn 'static + std::error::Error)> {
		match self {
			Error::OutOfMemory => None,
			Error::Map(e) => Some(e),
			Error::Allocation(e) => Some(e)
		}
	}
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
	fn prepare(&self, memory_requirements: MemoryRequirements);

	/// Allocate some memory.
	fn allocate(&self, memory_requirements: MemoryRequirements) -> Result<Self::Slot, Error>;

	/// Reallocate host-visible memory.
	fn reallocate(&self, slot: HostVisible<Self::Slot>, memory_requirements: MemoryRequirements) -> Result<HostVisible<Self::Slot>, Error>;
}

unsafe impl<A: 'static + std::ops::Deref + DeviceOwned> Allocator for A where A::Target: Allocator {
	type Slot = <A::Target as Allocator>::Slot;

	fn prepare(&self, memory_requirements: MemoryRequirements) {
		std::ops::Deref::deref(self).prepare(memory_requirements)
	}

	/// Allocate some memory.
	fn allocate(&self, memory_requirements: MemoryRequirements) -> Result<Self::Slot, Error> {
		std::ops::Deref::deref(self).allocate(memory_requirements)
	}

	/// Reallocate host-visible memory.
	fn reallocate(&self, slot: HostVisible<Self::Slot>, memory_requirements: MemoryRequirements) -> Result<HostVisible<Self::Slot>, Error> {
		std::ops::Deref::deref(self).reallocate(slot, memory_requirements)
	}
}

pub unsafe trait Slot: 'static {
	fn memory(&self) -> &device::Memory;

	fn offset(&self) -> u64;

	fn size(&self) -> u64;

	fn ptr(&self) -> Option<*mut c_void>;
}

unsafe impl<S: 'static + Slot + ?Sized> Slot for Box<S> {
	fn memory(&self) -> &device::Memory {
		self.as_ref().memory()
	}

	fn offset(&self) -> u64 {
		self.as_ref().offset()
	}

	fn size(&self) -> u64 {
		self.as_ref().size()
	}

	fn ptr(&self) -> Option<*mut c_void> {
		self.as_ref().ptr()
	}
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

	#[inline]
	pub fn unwrap(self) -> S {
		self.0
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