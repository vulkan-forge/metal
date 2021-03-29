//! Update interface.

use std::sync::Arc;
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::Device;
use super::{
	Descriptor,
	Write,
	Set,
	set,
	Writer
};

/// Descriptor set update.
pub struct Update {
	device: Option<Arc<Device>>,
	writes: Vec<vk::WriteDescriptorSet>,
	copies: Vec<vk::CopyDescriptorSet>
}

impl Update {
	pub fn new() -> Self {
		Self {
			device: None,
			writes: Vec::new(),
			copies: Vec::new()
		}
	}

	/// Write to the given descriptor set.
	/// 
	/// ## Safety
	/// 
	/// The caller must ensure that the input descriptor set will not be used
	/// before the changes are applied, either by calling `apply` or
	/// by dropping the `Update` instance.
	pub unsafe fn write<D: Descriptor, S: Set, V>(
		&mut self,
		descriptor_set: &mut S,
		offset: u32,
		value: &V
	) where
		S::Layout: set::layout::HasDescriptor<D>,
		V: Writer<S, D>
	{
		let mut p_image_info = std::ptr::null();
		let mut p_buffer_info = std::ptr::null();
		let p_texel_buffer_view = std::ptr::null();

		let info = value.prepare();

		let count = match &info {
			Write::Image(info) => {
				p_image_info = info;
				1
			},
			Write::Images(infos) => {
				p_image_info = infos.as_ptr();
				infos.len() as u32
			},
			Write::Buffer(info) => {
				p_buffer_info = info;
				1
			},
			Write::Buffers(infos) => {
				p_buffer_info = infos.as_ptr();
				infos.len() as u32
			}
		};

		assert!(offset + count <= D::COUNT);

		self.writes.push(vk::WriteDescriptorSet {
			dst_set: descriptor_set.handle(),
			dst_binding: <S::Layout as set::layout::HasDescriptor<D>>::BINDING,
			dst_array_element: offset,
			descriptor_count: count,
			descriptor_type: D::TYPE.into_vulkan(),
			p_image_info,
			p_buffer_info,
			p_texel_buffer_view,
			..Default::default()
		})
	}

	pub fn apply(self) {
		std::mem::drop(self)
	}
}

impl Drop for Update {
	fn drop(&mut self) {
		if let Some(device) = self.device.take() {
			unsafe {
				device.handle().update_descriptor_sets(&self.writes, &self.copies)
			}
		}
	}
}