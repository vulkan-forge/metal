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
	OomError,
	pipeline::shader,
	resource
};
use super::super::{
	Type,
	Descriptor,
};

pub type RawHandle = vk::DescriptorSetLayout;

#[repr(transparent)]
pub struct Binding(vk::DescriptorSetLayoutBinding);

impl Binding {
	pub const fn new(index: u32, ty: Type, count: u32, stage_flags: shader::Stages) -> Binding {
		Binding(vk::DescriptorSetLayoutBinding {
			binding: index,
			descriptor_type: ty.into_vulkan(),
			descriptor_count: count,
			stage_flags: stage_flags.into_vulkan(),
			p_immutable_samplers: std::ptr::null()
		})
	}
}

/// Descriptor set layout.
/// 
/// ## Safety
/// 
/// The `BINDINGS` const must match the layout bindings.
pub unsafe trait Layout: Sized {
	const BINDINGS: &'static [Binding];
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
			p_bindings: L::BINDINGS.as_ptr() as *const _,
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

	pub fn handle(&self) -> RawHandle {
		self.handle
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

/// Sets layouts compatibility marker.
/// 
/// This correspond to the notion of ["compatible for set N"](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#descriptorsets-compatibility)
/// in the vulkan specification.
pub unsafe trait CompatibleWith<L>: Layouts {
	// ...
}

unsafe impl<L: Layouts> CompatibleWith<L> for L {}

unsafe impl<L: Layout> Layouts for Instance<L> {
	type Handles<'a> = &'a [RawHandle];

	fn handles<'a>(&'a self) -> Self::Handles<'a> {
		std::slice::from_ref(&self.handle)
	}
}

/// No layouts.
unsafe impl Layouts for () {
	type Handles<'a> = &'a [RawHandle];

	fn handles(&self) -> &[RawHandle] {
		&[]
	}
}

// /// The empty set layouts is compatible with anything.
// unsafe impl<L: Layouts> CompatibleWith<L> for () {}