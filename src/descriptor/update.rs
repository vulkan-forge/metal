//! Update interface.

use std::sync::Arc;
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	Device,
	resource::Reference
};
use super::{
	Descriptor,
	Set,
	set,
	Write,
	WriteInfo
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

	// /// Write to the given descriptor set.
	// /// 
	// /// ## Safety
	// /// 
	// /// The caller must ensure that the input descriptor set will not be used
	// /// before the changes are applied, either by calling `apply` or
	// /// by dropping the `Update` instance.
	// pub unsafe fn write<'s, D: Descriptor, S: Set, V>(
	// 	&mut self,
	// 	descriptor_set: &S,
	// 	offset: u32,
	// 	value: &V
	// ) where
	// 	S::Layout: set::layout::HasDescriptor<D>,
	// 	S: Write<D, V>
	// {
	// 	let mut p_image_info = std::ptr::null();
	// 	let mut p_buffer_info = std::ptr::null();
	// 	let p_texel_buffer_view = std::ptr::null();

	// 	let info = S::prepare(value);

	// 	let count = match &info {
	// 		WriteInfo::Image(info) => {
	// 			p_image_info = info;
	// 			1
	// 		},
	// 		WriteInfo::Images(infos) => {
	// 			p_image_info = infos.as_ptr();
	// 			infos.len() as u32
	// 		},
	// 		WriteInfo::Buffer(info) => {
	// 			p_buffer_info = info;
	// 			1
	// 		},
	// 		WriteInfo::Buffers(infos) => {
	// 			p_buffer_info = infos.as_ptr();
	// 			infos.len() as u32
	// 		}
	// 	};

	// 	assert!(offset + count <= D::COUNT);

	// 	self.writes.push(vk::WriteDescriptorSet {
	// 		dst_set: descriptor_set.handle(),
	// 		dst_binding: <S::Layout as set::layout::HasDescriptor<D>>::BINDING,
	// 		dst_array_element: offset,
	// 		descriptor_count: count,
	// 		descriptor_type: D::TYPE.into_vulkan(),
	// 		p_image_info,
	// 		p_buffer_info,
	// 		p_texel_buffer_view,
	// 		..Default::default()
	// 	})
	// }

	pub fn apply(self) {
		std::mem::drop(self)
	}

	pub fn update_set<'a, S: Set>(&'a mut self, set: &'a mut set::Instance<S>) -> UpdateSet<'a, S> {
		UpdateSet {
			update: self,
			set
		}
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

pub struct UpdateSet<'a, S: Set> {
	update: &'a mut Update,
	set: &'a mut set::Instance<S>,
}

impl<'a, S: Set> UpdateSet<'a, S> {
	fn write(&mut self, set: S) {
		set.write(self);
		self.set.set_data(set);
	}

	pub fn write_descriptor<D: Descriptor, V>(
		&mut self,
		offset: u32,
		value: &V
	) where
		S::Layout: set::layout::HasDescriptor<D>,
		S: Write<D, V>
	{
		let mut p_image_info = std::ptr::null();
		let mut p_buffer_info = std::ptr::null();
		let p_texel_buffer_view = std::ptr::null();

		let info = S::prepare(value);

		let count = match &info {
			WriteInfo::Image(info) => {
				p_image_info = info;
				1
			},
			WriteInfo::Images(infos) => {
				p_image_info = infos.as_ptr();
				infos.len() as u32
			},
			WriteInfo::Buffer(info) => {
				p_buffer_info = info;
				1
			},
			WriteInfo::Buffers(infos) => {
				p_buffer_info = infos.as_ptr();
				infos.len() as u32
			}
		};

		assert!(offset + count <= D::COUNT);

		self.update.writes.push(vk::WriteDescriptorSet {
			dst_set: self.set.handle(),
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
}