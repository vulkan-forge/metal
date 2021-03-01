use ash::vk;

mod usage;
mod unbound;
mod bound;
mod typed;
mod index;

pub use usage::*;
pub use unbound::*;
pub use bound::*;
pub use typed::*;
pub use index::*;

/// Buffer.
pub unsafe trait Buffer: crate::Resource {
	fn handle(&self) -> vk::Buffer;
}

unsafe impl<B: std::ops::Deref> Buffer for B where B::Target: Buffer {
	#[inline]
	fn handle(&self) -> vk::Buffer {
		self.deref().handle()
	}
}

/// Typed buffer.
pub unsafe trait TypedBuffer: Buffer {
	/// Buffer item type.
	type Item;
}

unsafe impl<B: std::ops::Deref> TypedBuffer for B where B::Target: TypedBuffer {
	type Item = <B::Target as TypedBuffer>::Item;
}

pub struct Buffers<'a> {
	handles: Vec<vk::Buffer>,
	resources: Vec<crate::resource::Ref<'a>>
}

impl<'a> Buffers<'a> {
	pub fn new() -> Self {
		Self {
			handles: Vec::new(),
			resources: Vec::new()
		}
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

impl<'a> IntoIterator for Buffers<'a> {
	type Item = crate::resource::Ref<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::Ref<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}