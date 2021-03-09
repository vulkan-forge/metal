pub use std::{
	sync::Arc,
	hash::{
		Hash,
		Hasher
	}
};
use super::{
	AbstractResource,
	Resource
};

pub struct Ref<'a>(Box<dyn 'a + AbstractResource>);

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

impl<'a, R: 'a + AbstractResource> From<R> for Ref<'a> {
	fn from(r: R) -> Self {
		Self(Box::new(r))
	}
}