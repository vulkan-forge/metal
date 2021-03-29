use std::{
	sync::Arc,
	marker::PhantomData
};
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	Device,
	Resource,
	OomError
};
use super::super::{
	Descriptor,
};

pub type RawHandle = vk::DescriptorSetLayout;

pub type Binding = vk::DescriptorSetLayoutBinding;

/// Descriptor set layout.
/// 
/// ## Safety
/// 
/// The `BINDINGS` const must match the layout bindings.
pub unsafe trait Layout: Sized + Resource<Handle=RawHandle> {
	const BINDINGS: &'static [vk::DescriptorSetLayoutBinding];
}

/// Layout instance, for a given device.
pub struct Instance<L: Layout> {
	device: Arc<Device>,
	handle: RawHandle,
	layout: PhantomData<L>
}

impl<L: Layout> Instance<L> {
	/// Create a new layout instance for the given device.
	pub fn new(device: &Arc<Device>) -> Result<Self, OomError> {
		let infos = vk::DescriptorSetLayoutCreateInfo {
			binding_count: L::BINDINGS.len() as u32,
			p_bindings: L::BINDINGS.as_ptr(),
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_descriptor_set_layout(&infos, None)?
		};

		Ok(Instance {
			device: device.clone(),
			handle,
			layout: PhantomData
		})
	}
}

impl<L: Layout> Drop for Instance<L> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_descriptor_set_layout(self.handle, None);
		}
	}
}

/// Property for a given layout having defining the given descriptor.
pub unsafe trait HasDescriptor<D: Descriptor> {
	/// Descriptor binding.
	const BINDING: u32;
}

/// List of desctriptor set layouts.
/// 
/// ## Safety
/// 
/// The `Allocated` type must be an array/tuple of `DescriptorSet` types
/// whose layout type `L` parameter matches the associated layout.
pub unsafe trait Layouts {
	/// Layout handles.
	type Handles<'a>: AsRef<[RawHandle]>;

	fn handles<'a>(&'a self) -> Self::Handles<'a>;
}

/// No layouts.
unsafe impl Layouts for () {
	type Handles<'a> = &'a [RawHandle];

	fn handles(&self) -> &[RawHandle] {
		&[]
	}
}