use ash::vk;
use crate::resource;

mod usage;
pub mod sub;

mod unbound;
mod bound;
mod typed;
// mod index;
pub mod vec;
pub mod array;

pub use usage::*;

pub use unbound::*;
pub use bound::*;
pub use typed::*;
// pub use index::*;
pub use vec::Vec;
pub use array::Array;

/// Raw buffer handle.
pub type Handle = vk::Buffer;

pub trait Reference = resource::Reference<Handle=Handle>;

// pub struct LocalBuffers<'a> {
// 	handles: StdVec<vk::Buffer>,
// 	resources: StdVec<resource::Ref<'a>>
// }

// impl<'a> LocalBuffers<'a> {
// 	pub fn new() -> Self {
// 		Self {
// 			handles: StdVec::new(),
// 			resources: StdVec::new()
// 		}
// 	}

// 	pub fn is_empty(&self) -> bool {
// 		self.handles.is_empty()
// 	}

// 	pub fn len(&self) -> usize {
// 		self.handles.len()
// 	}

// 	pub fn push<B: 'a + Reference>(&mut self, buffer: B) {
// 		self.handles.push(buffer.handle());
// 		self.resources.push(buffer.into());
// 	}

// 	pub(crate) fn as_vulkan(&self) -> &[vk::Buffer] {
// 		&self.handles
// 	}
// }

// impl<'a> IntoIterator for LocalBuffers<'a> {
// 	type Item = resource::Ref<'a>;
// 	type IntoIter = std::vec::IntoIter<resource::Ref<'a>>;

// 	fn into_iter(self) -> Self::IntoIter {
// 		self.resources.into_iter()
// 	}
// }

// pub struct Buffers<'a> {
// 	handles: StdVec<vk::Buffer>,
// 	resources: StdVec<resource::SendRef<'a>>
// }

// impl<'a> Buffers<'a> {
// 	pub fn new() -> Self {
// 		Self {
// 			handles: StdVec::new(),
// 			resources: StdVec::new()
// 		}
// 	}

// 	pub fn is_empty(&self) -> bool {
// 		self.handles.is_empty()
// 	}

// 	pub fn len(&self) -> usize {
// 		self.handles.len()
// 	}

// 	pub fn push<B: 'a + Send + Reference>(&mut self, buffer: B) {
// 		self.handles.push(buffer.handle());
// 		self.resources.push(buffer.into());
// 	}

// 	pub(crate) fn as_vulkan(&self) -> &[vk::Buffer] {
// 		&self.handles
// 	}
// }

// impl<'a> IntoIterator for Buffers<'a> {
// 	type Item = resource::SendRef<'a>;
// 	type IntoIter = std::vec::IntoIter<resource::SendRef<'a>>;

// 	fn into_iter(self) -> Self::IntoIter {
// 		self.resources.into_iter()
// 	}
// }