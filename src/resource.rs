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

pub struct Ref<'a>(Arc<dyn 'a + Resource>);

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

impl<'a, R: 'a + Resource> From<Arc<R>> for Ref<'a> {
	fn from(r: Arc<R>) -> Ref<'a> {
		Ref(r)
	}
}