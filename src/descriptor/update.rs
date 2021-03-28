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
	set
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

	pub fn write<S: Set, D: Descriptor>(
		&mut self,
		descriptor_set: &S,
		_descriptor: D,
		offset: u32,
		value: D::Value
	) where
		S::Layout: set::layout::HasDescriptor<D>
	{
		let mut p_image_info = std::ptr::null();
		let mut p_buffer_info = std::ptr::null();
		let p_texel_buffer_view = std::ptr::null();

		let info = D::write(value);

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
		if let Some(device) = self.device {
			unsafe {
				device.handle().update_descriptor_sets(&self.writes, &self.copies)
			}
		}
	}
}