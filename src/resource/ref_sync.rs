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

pub struct SyncRef<'a>(Box<dyn 'a + Send + Sync + AbstractResource>);

assert_impl_all!(SyncRef<'static>: Send, Sync);

impl<'a> SyncRef<'a> {
	#[inline]
	pub fn uid(&self) -> u64 {
		self.0.uid()
	}
}

impl<'a> PartialEq for SyncRef<'a> {
	#[inline]
	fn eq(&self, other: &SyncRef) -> bool {
		self.uid() == other.uid()
	}
}

impl<'a> Eq for SyncRef<'a> {}

impl<'a> Hash for SyncRef<'a> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.uid().hash(h)
	}
}

impl<'a, R: 'a + Send + Sync + AbstractResource> From<R> for SyncRef<'a> {
	fn from(r: R) -> Self {
		Self(Box::new(r))
	}
}