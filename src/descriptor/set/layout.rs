use std::{
	sync::Arc
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
	// Descriptor,
};

pub type Handle = vk::DescriptorSetLayout;

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
pub trait Layout: resource::Reference<Handle=Handle> {}

/// Creates a descriptor set layout from a
/// list of bindings declared with the following syntax:
/// 
/// ```text
/// n [count] : type => [shader_stage_1, ..., shader_stage_k]
/// ```
/// 
/// If `count` is `1`, then `[1]` can be omitted.
/// 
/// ## Example
/// 
/// ```
/// use magma::descriptor::Type;
/// use magma::pipeline::shader::Stage;
/// 
/// descriptor_set_layout! {
///   pub struct MyDescriptorSetLayout {
///     0 : Type::UniformBuffer => [Stage::Vertex],
///     1 : Type::Sampler => [Stage::Fragment]
///   }
/// }
/// ```
#[macro_export]
macro_rules! descriptor_set_layout {
	{
		$vis:vis struct $id:ident {
			$($loc:literal $([$count:literal])* : $ty:expr => [$($stage:expr),*]),*
		}
	} => {
		$vis struct $id($crate::descriptor::set::layout::Raw);

		unsafe impl $crate::resource::AbstractReference for $id {
			fn uid(&self) -> u64 {
				self.0.handle().as_raw()
			}
		}

		unsafe impl $crate::resource::Reference {
			type Handle = $crate::descriptor::set::layout::Handle;

			fn handle(&self) -> Self::Handle {
				self.0.handle()
			}
		}

		impl $crate::descriptor::set::Layout for $id {
			pub fn new(device: std::sync::Arc<$crate::Device>) -> Self {
				Self($crate::descriptor::set::layout::Raw::new(
					device,
					&[
						$(
							Binding::new($loc, $ty, descriptor_set_layout(@count $($count)*), [$($stage),*].into_iter().collect())
						),*
					]
				))
			}
		}
	};
	(@count [$count:literal]) => { $count };
	(@count) => { 1 }
}

/// Typed descriptor set layout.
pub trait TypedLayout {
	/// The untyped layout.
	type Untyped;
}

/// Property of a typed layout having a binding
/// at location `N`.
pub unsafe trait Bound<const N: u32> {
	/// Binding type.
	type Type;
}

/// Raw layout instance.
pub struct Raw {
	device: Arc<Device>,
	handle: Handle
}

impl Raw {
	/// Create a new layout instance for the given device.
	pub fn new(device: &Arc<Device>, bindings: &[Binding]) -> Result<Self, OomError> {
		let infos = vk::DescriptorSetLayoutCreateInfo {
			binding_count: bindings.len() as u32,
			p_bindings: bindings.as_ptr() as *const _,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_descriptor_set_layout(&infos, None)?
		};

		Ok(Self {
			device: device.clone(),
			handle
		})
	}

	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl Drop for Raw {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_descriptor_set_layout(self.handle, None);
		}
	}
}

// /// Property for a given layout having defining the given descriptor.
// pub unsafe trait HasDescriptor<D: Descriptor> {
// 	/// Descriptor binding.
// 	const BINDING: u32;
// }

/// List of desctriptor set layouts.
/// 
/// ## Safety
/// 
/// The `Allocated` type must be an array/tuple of `DescriptorSet` types
/// whose layout type `L` parameter matches the associated layout.
pub unsafe trait Layouts {
	/// Layout handles.
	type Handles<'a>: AsRef<[Handle]>;

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

unsafe impl<L: Layout> Layouts for L {
	type Handles<'a> = [Handle; 1];

	fn handles<'a>(&'a self) -> Self::Handles<'a> {
		[self.handle()]
	}
}

/// No layouts.
unsafe impl Layouts for () {
	type Handles<'a> = &'a [Handle];

	fn handles(&self) -> &[Handle] {
		&[]
	}
}

// /// The empty set layouts is compatible with anything.
// unsafe impl<L: Layouts> CompatibleWith<L> for () {}