use std::{
	marker::PhantomData,
	sync::Arc,
	ops::{
		Deref,
		DerefMut
	},
	fmt
};
use ash::vk;
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
			Unbound,
			Bound
		},
		staging
	},
	sync::SharingQueues
};

#[derive(Debug)]
pub enum Error {
	BufferCreation(buffer::CreationError),
	Bind(buffer::BindError),
	Memory(mem::Error)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::BufferCreation(e) => write!(f, "buffer creation failed: {}", e),
			Error::Bind(e) => write!(f, "bind failed: {}", e),
			Error::Memory(e) => write!(f, "memory error: {}", e)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn 'static + std::error::Error)> {
		match self {
			Error::BufferCreation(e) => Some(e),
			Error::Bind(e) => Some(e),
			Error::Memory(e) => Some(e)
		}
	}
}

impl From<buffer::CreationError> for Error {
	fn from(e: buffer::CreationError) -> Self {
		Self::BufferCreation(e)
	}
}

impl From<buffer::BindError> for Error {
	fn from(e: buffer::BindError) -> Self {
		Self::Bind(e)
	}
}

impl From<mem::Error> for Error {
	fn from(e: mem::Error) -> Self {
		Self::Memory(e)
	}
}

struct Inner<A: Allocator> {
	buffer: Bound<HostVisible<A::Slot>>,
	capacity: u64
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
			Some(inner) => {
				inner.capacity
			},
			None => 0
		}
	}

	fn ptr(&self) -> *const T {
		self.inner.as_ref().map(|inner| inner.buffer.memory_slot().ptr() as *const T).unwrap_or(std::ptr::null())
	}

	fn mut_ptr(&mut self) -> *mut T {
		self.inner.as_ref().map(|inner| inner.buffer.memory_slot().ptr() as *mut T).unwrap_or(std::ptr::null_mut())
	}

	pub fn into_typed(self) -> Result<buffer::Typed<T, HostVisible<A::Slot>>, Error> {
		match self.inner {
			Some(inner) => {
				Ok(unsafe { inner.buffer.into_typed() })
				// Ok(buffer::Typed::from_raw_parts(inner.buffer, Box::new(inner.slot.unwrap())))
			},
			None => {
				let layout = std::alloc::Layout::new::<T>();

				let buffer = Unbound::new(
					self.device(),
					0,
					self.usage(),
					self.sharing_mode.clone()
				)?;

				let memory_requirements = buffer.memory_requirements().align_to(layout.align() as u64);

				let slot = self.allocator.allocate(memory_requirements)?;

				Ok(unsafe { buffer.bind(slot).map_err(|(_, e)| e)?.into_typed() })
			}
		}
	}
}

impl<T: Copy, A: Allocator> Vec<T, A> {
	pub fn new<U: Into<Usages>, S: Into<SharingQueues>>(allocator: staging::Allocator<A>, initial_capacity: u64, usage: U, sharing_queues: S) -> Result<Self, Error> {
		log::info!("new vec");
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

			if new_capacity == 0 {
				new_capacity = capacity
			} else {
				while new_capacity < capacity {
					log::info!("{} < {}", new_capacity, capacity);
					new_capacity *= 2;
				}
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
					self.allocator.reallocate(inner.buffer.unbind(), memory_requirements)?
				},
				None => {
					self.allocator.allocate(memory_requirements)?
				}
			};

			let bound_buffer = unsafe {
				match new_buffer.bind(new_slot) {
					Ok(bound_buffer) => bound_buffer,
					Err((_, e)) => {
						return Err(e.into())
					}
				}
			};
	
			self.inner = Some(Inner {
				buffer: bound_buffer,
				capacity: new_capacity,
			});
		}

		Ok(())
	}

	pub fn resize(&mut self, new_len: u64, value: T) -> Result<(), Error> {
		self.ensure_capacity(new_len)?;

		self.len = new_len;
		for i in self.len..new_len {
			self[i as usize] = value;
		}

		Ok(())
	}

	pub fn push(&mut self, value: T) -> Result<(), Error> {
		let i = self.len as usize;
		self.len += 1;

		self.ensure_capacity(self.len)?;

		self[i] = value;

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
