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

pub struct SendRef<'a>(Box<dyn 'a + Send + AbstractResource>);

assert_impl_all!(SendRef<'static>: Send);

impl<'a> SendRef<'a> {
	#[inline]
	pub fn uid(&self) -> u64 {
		self.0.uid()
	}
}

impl<'a> PartialEq for SendRef<'a> {
	#[inline]
	fn eq(&self, other: &SendRef) -> bool {
		self.uid() == other.uid()
	}
}

impl<'a> Eq for SendRef<'a> {}

impl<'a> Hash for SendRef<'a> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.uid().hash(h)
	}
}

impl<'a, R: 'a + Send + AbstractResource> From<R> for SendRef<'a> {
	fn from(r: R) -> Self {
		Self(Box::new(r))
	}
}