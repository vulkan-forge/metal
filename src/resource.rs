pub use std::{
	sync::Arc,
	hash::{
		Hash,
		Hasher
	}
};

/// GPU resource.
pub unsafe trait Resource {
	/// Unique identifier of the resource.
	/// 
	/// ## Safety
	/// 
	/// This must be unique across a given device.
	fn uid(&self) -> u64;
}

unsafe impl<B: std::ops::Deref> Resource for B where B::Target: Resource {
	#[inline]
	fn uid(&self) -> u64 {
		self.deref().uid()
	}
}

pub struct Ref<'a>(Box<dyn 'a + Resource>);

impl<'a> Ref<'a> {
	#[inline]
	pub fn uid(&self) -> u64 {
		self.0.uid()
	}
}

impl<'a> PartialEq for Ref<'a> {
	#[inline]
	fn eq(&self, other: &Ref) -> bool {
		self.uid() == other.uid()
	}
}

impl<'a> Eq for Ref<'a> {}

impl<'a> Hash for Ref<'a> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.uid().hash(h)
	}
}

impl<'a, R: 'a + Resource> From<R> for Ref<'a> {
	fn from(r: R) -> Self {
		Self(Box::new(r))
	}
}