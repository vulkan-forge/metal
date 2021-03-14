use std::vec::Vec as StdVec;
use ash::vk;
use crate::resource::Proxy;

mod usage;
mod unbound;
mod bound;
mod typed;
mod index;
mod vec;

pub use usage::*;
pub use unbound::*;
pub use bound::*;
pub use typed::*;
pub use index::*;
pub use vec::*;

/// Buffer.
pub unsafe trait Buffer: crate::Resource<Handle=vk::Buffer> {
	// ...
}

unsafe impl<B: std::ops::Deref> Buffer for B where B::Target: Buffer {
	// ...
}

unsafe impl<B: Buffer> Buffer for Proxy<B> {
	// ...
}

/// Typed buffer.
pub unsafe trait TypedBuffer: Buffer {
	/// Buffer item type.
	type Item;
}

unsafe impl<B: std::ops::Deref> TypedBuffer for B where B::Target: TypedBuffer {
	type Item = <B::Target as TypedBuffer>::Item;
}

unsafe impl<B: TypedBuffer> TypedBuffer for Proxy<B> {
	type Item = B::Item;
}

pub struct LocalBuffers<'a> {
	handles: StdVec<vk::Buffer>,
	resources: StdVec<crate::resource::Ref<'a>>
}

impl<'a> LocalBuffers<'a> {
	pub fn new() -> Self {
		Self {
			handles: StdVec::new(),
			resources: StdVec::new()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.handles.is_empty()
	}

	pub fn len(&self) -> usize {
		self.handles.len()
	}

	pub fn push<B: 'a + Buffer>(&mut self, buffer: B) {
		self.handles.push(buffer.handle());
		self.resources.push(buffer.into());
	}

	pub(crate) fn as_vulkan(&self) -> &[vk::Buffer] {
		&self.handles
	}
}

impl<'a> IntoIterator for LocalBuffers<'a> {
	type Item = crate::resource::Ref<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::Ref<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}

pub struct Buffers<'a> {
	handles: StdVec<vk::Buffer>,
	resources: StdVec<crate::resource::SendRef<'a>>
}

impl<'a> Buffers<'a> {
	pub fn new() -> Self {
		Self {
			handles: StdVec::new(),
			resources: StdVec::new()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.handles.is_empty()
	}

	pub fn len(&self) -> usize {
		self.handles.len()
	}

	pub fn push<B: 'a + Send + Buffer>(&mut self, buffer: B) {
		self.handles.push(buffer.handle());
		self.resources.push(buffer.into());
	}

	pub(crate) fn as_vulkan(&self) -> &[vk::Buffer] {
		&self.handles
	}
}

impl<'a> IntoIterator for Buffers<'a> {
	type Item = crate::resource::SendRef<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::SendRef<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}