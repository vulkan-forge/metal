pub use std::{
	hash::{
		Hash,
		Hasher
	}
};
use crate::sync::future::Futures;

/// Abstract handle.
pub trait Handle = ash::vk::Handle;

pub struct Any<R: AbstractReference + ?Sized>(Box<R>);

impl<R: AbstractReference + ?Sized> Any<R> {
	#[inline]
	pub fn uid(&self) -> u64 {
		self.0.uid()
	}

	#[inline]
	pub fn range(&self) -> Option<Range> {
		self.0.range()
	}

	#[inline]
	pub fn borrow_condition(&self) -> Option<BorrowCondition> {
		self.0.borrow_condition()
	}

	pub fn aliases<S: AbstractReference + ?Sized>(&self, other: &S) -> bool {
		aliases(&self.0, other)
	}

	#[inline]
	pub fn check_borrow_rules<P: Futures>(&self, past: &P) {
		match self.borrow_condition() {
			Some(BorrowCondition::PastUse) => {
				if !past.uses(&self.0) {
					panic!("cannot borrow here: resorce may be in use.")
				}
			},
			None => ()
		}
	}
}

impl<R: AbstractReference + ?Sized> PartialEq for Any<R> {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.uid() == other.uid() && self.range() == other.range()
	}
}

impl<R: AbstractReference + ?Sized> Eq for Any<R> {}

impl<R: AbstractReference + ?Sized> Hash for Any<R> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.uid().hash(h);
		self.range().hash(h)
	}
}

pub type Ref<'a> = Any<dyn 'a + AbstractReference>;
pub type SendRef<'a> = Any<dyn 'a + Send + AbstractReference>;
pub type SyncRef<'a> = Any<dyn 'a + Send + Sync + AbstractReference>;

impl<'a, R: 'a + AbstractReference> From<R> for Ref<'a> {
	fn from(r: R) -> Self {
		Any(Box::new(r))
	}
}

impl<'a, R: 'a + Send + AbstractReference> From<R> for SendRef<'a> {
	fn from(r: R) -> Self {
		Any(Box::new(r))
	}
}

impl<'a, R: 'a + Send + Sync + AbstractReference> From<R> for SyncRef<'a> {
	fn from(r: R) -> Self {
		Any(Box::new(r))
	}
}

/// Subresource range.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
	offset: u64,
	len: u64
}

impl Range {
	pub fn aliases(&self, other: Range) -> bool {
		self.offset + self.len > other.offset && other.offset + other.len > self.offset
	}

	pub fn opt_aliases(a: Option<Range>, b: Option<Range>) -> bool {
		match (a, b) {
			(Some(a), Some(b)) => {
				a.aliases(b)
			},
			_ => false
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BorrowCondition {
	PastUse
}

/// Untyped resource reference.
/// 
/// ## Safety
/// 
/// The `uid` must not change and be unique across all the living resources of a given device.
/// `range` must not change and must be included in the resource span.
pub unsafe trait AbstractReference {
	/// Resource reference.
	fn uid(&self) -> u64;

	/// Subresource range (if any).
	fn range(&self) -> Option<Range> {
		None
	}

	fn borrow_condition(&self) -> Option<BorrowCondition> {
		None
	}
}

pub fn aliases<A: AbstractReference + ?Sized, B: AbstractReference + ?Sized>(a: &A, b: &B) -> bool {
	a.uid() == b.uid() && Range::opt_aliases(a.range(), b.range())
}

unsafe impl<R: std::ops::Deref> AbstractReference for R where R::Target: AbstractReference {
	fn uid(&self) -> u64 {
		std::ops::Deref::deref(self).uid()
	}

	fn range(&self) -> Option<Range> {
		std::ops::Deref::deref(self).range()
	}

	fn borrow_condition(&self) -> Option<BorrowCondition> {
		std::ops::Deref::deref(self).borrow_condition()
	}
}

/// Resource reference.
pub unsafe trait Reference: AbstractReference {
	type Handle: Handle;

	fn handle(&self) -> Self::Handle;
}

unsafe impl<R: std::ops::Deref> Reference for R where R::Target: Reference {
	type Handle = <R::Target as Reference>::Handle;

	fn handle(&self) -> Self::Handle {
		std::ops::Deref::deref(self).handle()
	}
}

// /// GPU resource reader.
// /// 
// /// ## Safety
// /// 
// /// Implementor must ensure that `validate` return a correct result.
// pub unsafe trait Read: Reference {
// 	// ...
// }

// unsafe impl<R: std::ops::Deref> Read for R where R::Target: Read {
// 	// ...
// }

// /// GPU resource writer.
// /// 
// /// ## Safety
// /// 
// /// Implementor must ensure that `validate` return a correct result.
// pub unsafe trait Write: Reference {
// 	// ...
// }

// unsafe impl<R: std::ops::Deref> Write for R where R::Target: Write {
// 	// ...
// }