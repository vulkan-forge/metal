use std::ops::{
	Deref,
	DerefMut
};
pub use super::Reference;

// pub mod index;

// pub use index::*;

/// Sub buffer reader.
pub unsafe trait Read: Reference {
	/// Byte offset in the buffer.
	fn byte_offset(&self) -> u64;

	/// Byte length of the subbuffer range.
	fn byte_len(&self) -> u64;
}

/// Anything that deref into a buffer can be considered as a buffer.
unsafe impl<B: Deref> Read for B where B::Target: Read {
	fn byte_offset(&self) -> u64 {
		Deref::deref(self).byte_offset()
	}

	fn byte_len(&self) -> u64 {
		Deref::deref(self).byte_len()
	}
}

pub unsafe trait Write: Read {
	// ...
}

unsafe impl<B: DerefMut> Write for B where B::Target: Write { }

pub unsafe trait TypedRead: Read {
	type Item;

	/// Length of the buffer (in the number of element of type `Self::Item`).
	fn len(&self) -> u64;
}

unsafe impl<B: Deref> TypedRead for B where B::Target: TypedRead {
	type Item = <B::Target as TypedRead>::Item;

	fn len(&self) -> u64 {
		Deref::deref(self).len()
	}
}

pub unsafe trait TypedWrite: TypedRead {
	// ...
}

unsafe impl<B: DerefMut> TypedWrite for B where B::Target: TypedWrite { }