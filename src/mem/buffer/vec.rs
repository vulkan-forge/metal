use std::{
	marker::PhantomData,
	sync::Arc,
	ops::{
		Deref,
		DerefMut
	}
};
use crate::{
	Device,
	DeviceOwned,
	mem::{
		self,
		Allocator,
		HostVisible,
		buffer::{
			self,
			Usages,
			Unbound
		},
		staging
	},
	sync::SharingQueues
};

pub enum Error {
	BufferCreation(buffer::CreationError),
	Memory(mem::Error)
}

impl From<buffer::CreationError> for Error {
	fn from(e: buffer::CreationError) -> Self {
		Self::BufferCreation(e)
	}
}

impl From<mem::Error> for Error {
	fn from(e: mem::Error) -> Self {
		Self::Memory(e)
	}
}

struct Inner<A: Allocator> {
	buffer: Unbound,
	slot: HostVisible<A::Slot>
}

pub struct Vec<T, A: Allocator> {
	allocator: staging::Allocator<A>,
	usage: Usages,
	sharing_mode: SharingQueues,
	inner: Option<Inner<A>>,
	len: u64,
	t: PhantomData<T>
}

impl<T, A: Allocator> Vec<T, A> {
	pub fn device(&self) -> &Arc<Device> {
		self.allocator.device()
	}

	pub fn usage(&self) -> Usages {
		self.usage
	}

	pub fn capacity(&self) -> u64 {
		match &self.inner {
			Some(inner) => inner.buffer.len(),
			None => 0
		}
	}

	fn ptr(&self) -> *const T {
		self.inner.as_ref().map(|inner| inner.slot.ptr() as *const T).unwrap_or(std::ptr::null())
	}

	fn mut_ptr(&mut self) -> *mut T {
		self.inner.as_ref().map(|inner| inner.slot.ptr() as *mut T).unwrap_or(std::ptr::null_mut())
	}
}

impl<T: Copy, A: Allocator> Vec<T, A> {
	pub fn new<U: Into<Usages>, S: Into<SharingQueues>>(allocator: staging::Allocator<A>, initial_capacity: u64, usage: U, sharing_queues: S) -> Result<Self, Error> {
		let mut this = Self {
			allocator,
			usage: usage.into(),
			sharing_mode: sharing_queues.into(),
			inner: None,
			len: 0,
			t: PhantomData
		};

		this.ensure_capacity(initial_capacity)?;
		Ok(this)
	}
	
	fn ensure_capacity(&mut self, capacity: u64) -> Result<(), Error> {
		if capacity > self.capacity() {
			let mut new_capacity = self.capacity();
			while new_capacity < capacity {
				new_capacity *= 2;
			}

			let layout = std::alloc::Layout::new::<T>();

			let new_buffer = Unbound::new(
				self.device(),
				new_capacity * layout.size() as u64,
				self.usage(),
				self.sharing_mode.clone()
			)?;

			let memory_requirements = new_buffer.memory_requirements().align_to(layout.align() as u64);
	
			let new_slot = match self.inner.take() {
				Some(inner) => {
					self.allocator.reallocate(inner.slot, memory_requirements)?
				},
				None => {
					self.allocator.allocate(memory_requirements)?
				}
			};
	
			self.inner = Some(Inner {
				buffer: new_buffer,
				slot: new_slot
			});
		}

		Ok(())
	}

	pub fn resize(&mut self, new_len: u64, value: T) -> Result<(), Error> {
		self.ensure_capacity(new_len)?;

		for i in self.len..new_len {
			self[i as usize] = value;
		}

		self.len = new_len;
		Ok(())
	}

	pub fn push(&mut self, value: T) -> Result<(), Error> {
		self.ensure_capacity(self.capacity() + 1)?;

		let i = self.len as usize;
		self[i] = value;
		self.len += 1;

		Ok(())
	}
}

impl<T, A: Allocator> Deref for Vec<T, A> {
	type Target = [T];

	fn deref(&self) -> &[T] {
		unsafe {
			std::slice::from_raw_parts(self.ptr(), self.len as usize)
		}
	}
}

impl<T, A: Allocator> DerefMut for Vec<T, A> {
	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {
			std::slice::from_raw_parts_mut(self.mut_ptr(), self.len as usize)
		}
	}
}