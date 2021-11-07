use std::{
	cell::Cell,
	ops::{
		Deref
	},
	rc::Rc,
	sync::{
		Arc,
		Mutex,
		MutexGuard
	}
};
use crate::{
	resource,
	mem::Slot
};
use super::{
	Handle,
	Typed,
	sub
};

pub struct Busy;

#[derive(Default)]
struct Lock {
	count: Cell<isize>
}

impl Lock {
	fn read(&self) -> bool {
		let current = self.count.get();

		if current >= 0 {
			self.count.set(current + 1);
			true
		} else {
			false
		}
	}

	fn write(&self) -> bool {
		let current = self.count.get();
		if current == 0 {
			self.count.set(-1);
			true
		} else {
			false
		}
	}

	fn release(&self) {
		let current = self.count.get();
		if current > 0 {
			self.count.set(current - 1)
		} else {
			self.count.set(0)
		}
	}
}

/// Array buffer where each element can be borrowed independently.
pub struct Array<T, S> {
	inner: Typed<T, S>,
	locks: Vec<Lock>
}

impl<T, S> Array<T, S> {
	pub fn new(buffer: Typed<T, S>) -> Self {
		let mut locks = Vec::new();
		locks.resize_with(buffer.len() as usize, || Lock::default());

		Array {
			inner: buffer,
			locks
		}
	}

	fn offset_of(&self, index: u32) -> u64 {
		(index as usize * std::mem::size_of::<T>()) as u64
	}
}

pub trait Reference: Sized {
	type Item;
	type Slot: Slot;

	type ReadGuard<'a>: Deref<Target=Array<Self::Item, Self::Slot>> where Self: 'a;

	fn read<'a>(&'a self) -> Self::ReadGuard<'a>;

	fn try_get(self, index: u32) -> Result<Read<Self>, Busy> {
		let (can_read, handle, offset, ptr) = {
			use resource::Reference;
			let guard = self.read();
			let can_read = guard.locks[index as usize].read();
			let handle = guard.inner.handle();
			let offset = guard.offset_of(index);
			let ptr = guard.inner.memory_slot().ptr().map(|ptr| unsafe { (ptr as *const Self::Item).offset(index as isize) });

			(can_read, handle, offset, ptr)
		};

		if can_read {
			Ok(Read {
				array: self,
				index,
				handle,
				offset,
				ptr
			})
		} else {
			Err(Busy)
		}
	}

	fn try_get_mut(self, index: u32) -> Result<Write<Self>, Busy> {
		let (can_write, handle, offset, ptr) = {
			use resource::Reference;
			let guard = self.read();
			let can_write = guard.locks[index as usize].write();
			let handle = guard.inner.handle();
			let offset = guard.offset_of(index);
			let ptr = guard.inner.memory_slot().ptr().map(|ptr| unsafe { (ptr as *mut Self::Item).offset(index as isize) });

			(can_write, handle, offset, ptr)
		};

		if can_write {
			Ok(Write {
				array: self,
				index,
				handle,
				offset,
				ptr
			})
		} else {
			Err(Busy)
		}
	}
}

impl<'a, T, S: Slot> Reference for &'a Array<T, S> {
	type Item = T;
	type Slot = S;
	type ReadGuard<'b> where Self: 'b = &'a Array<T, S>;

	fn read<'b>(&'b self) -> Self::ReadGuard<'b> {
		self
	}
}

impl<T: 'static, S: Slot + 'static> Reference for Rc<Array<T, S>> {
	type Item = T;
	type Slot = S;
	type ReadGuard<'b> = &'b Array<T, S>;

	fn read<'b>(&'b self) -> Self::ReadGuard<'b> {
		self
	}
}

impl<T: 'static, S: Slot + 'static> Reference for Arc<Mutex<Array<T, S>>> {
	type Item = T;
	type Slot = S;
	type ReadGuard<'b> = MutexGuard<'b, Array<T, S>>;

	fn read<'b>(&'b self) -> Self::ReadGuard<'b> {
		self.lock().expect("unable to lock array")
	}
}

impl<T: 'static, S: Slot + 'static> Reference for Arc<parking_lot::Mutex<Array<T, S>>> {
	type Item = T;
	type Slot = S;
	type ReadGuard<'b> = parking_lot::MutexGuard<'b, Array<T, S>>;

	fn read<'b>(&'b self) -> Self::ReadGuard<'b> {
		self.lock()
	}
}

pub struct Read<R: Reference> {
	/// Array reference.
	array: R,

	/// Index in the array.
	index: u32,

	/// Raw buffer handle.
	handle: Handle,

	/// Offset in the array.
	offset: u64,

	/// Pointer to the array item (if the inner buffer is host accessible).
	ptr: Option<*const R::Item>
}

impl<R: Reference> Read<R> {
	pub fn get(&self) -> Option<&R::Item> {
		self.ptr.map(|ptr| unsafe { &*ptr })
	}
}

// unsafe impl<R: Reference> resource::AbstractReference for Read<R> {
// 	fn uid(&self) -> u64 {
// 		use ash::vk::Handle;
// 		self.handle.as_raw()
// 	}
// }

unsafe impl<R: Reference> resource::Reference for Read<R> {
	type Handle = Handle;

	fn handle(&self) -> Handle {
		self.handle
	}
}

unsafe impl<R: Reference> sub::Read for Read<R> {
	fn byte_offset(&self) -> u64 {
		self.offset
	}

	fn byte_len(&self) -> u64 {
		std::mem::size_of::<R::Item>() as u64
	}
}

impl<R: Reference> Drop for Read<R> {
	fn drop(&mut self) {
		let guard = self.array.read();
		guard.locks[self.index as usize].release()
	}
}

pub struct Write<R: Reference> {
	/// Array reference.
	array: R,

	/// Index in the array.
	index: u32,

	/// Raw buffer handle.
	handle: Handle,

	/// Offset in the array.
	offset: u64,

	/// Pointer to the array item (if the inner buffer is host accessible).
	ptr: Option<*mut R::Item>
}

impl<R: Reference> Write<R> {
	pub fn get(&self) -> Option<&R::Item> {
		self.ptr.map(|ptr| unsafe { &*ptr })
	}

	pub fn get_mut(&mut self) -> Option<&mut R::Item> {
		self.ptr.map(|ptr| unsafe { &mut *ptr })
	}
}

// unsafe impl<R: Reference> resource::AbstractReference for Write<R> {
// 	fn uid(&self) -> u64 {
// 		use ash::vk::Handle;
// 		self.handle.as_raw()
// 	}
// }

unsafe impl<R: Reference> resource::Reference for Write<R> {
	type Handle = Handle;

	fn handle(&self) -> Handle {
		self.handle
	}
}

unsafe impl<R: Reference> sub::Read for Write<R> {
	fn byte_offset(&self) -> u64 {
		self.offset
	}

	fn byte_len(&self) -> u64 {
		std::mem::size_of::<R::Item>() as u64
	}
}

unsafe impl<R: Reference> sub::Write for Write<R> {}

impl<R: Reference> Drop for Write<R> {
	fn drop(&mut self) {
		let guard = self.array.read();
		guard.locks[self.index as usize].release()
	}
}