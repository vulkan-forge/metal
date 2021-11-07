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

/// Untyped shader module descriptor set layout
/// well typed by the typed shader module descriptor set layout
/// `T`.
/// 
/// The `T` layout type is a well typed version of `Self` if
/// - it matches `Self` (every binding defined in `T` is defined in `Self`)
/// - every binding defined in `Self` is defined in `T` and its
///   definition matches the constraints defined in `Self`
///   (descriptor type, count and accessing shader stages).
pub unsafe trait WellTypedBy<T> {}

/// Typed shader module descriptor set layout
/// that matches the untyped shader module descriptor set layout
/// `T`.
/// This means that all the bindings in `Self` are defined
/// in `T` with the same item count.
pub unsafe trait Matches<T> {}

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
/// n => type [count] (shader_stage_1, ..., shader_stage_k)
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
///     0 => UniformBuffer (Vertex),
///     1 => Sampler => (Fragment)
///   }
/// }
/// ```
#[macro_export]
macro_rules! untyped_descriptor_set_layout {
	{
		$vis:vis struct $id:ident {
			$($loc:literal => $ty:tt ($($stage:ident),*)),*
		}
	} => {
		$vis struct $id($crate::descriptor::set::layout::Raw);

		unsafe impl $crate::resource::Reference for $id {
			type Handle = $crate::descriptor::set::layout::Handle;

			fn handle(&self) -> Self::Handle {
				self.0.handle()
			}
		}

		impl $crate::descriptor::set::Layout for $id {}

		$(
			unsafe impl $crate::descriptor::set::layout::BindUntypedLocation<$loc> for $id {
				const TYPE: $crate::descriptor::Type = $crate::untyped_descriptor_set_layout!(@ty $ty);
				const COUNT: u32 = $crate::untyped_descriptor_set_layout!(@count $ty);
				const STAGES: $crate::pipeline::shader::Stages = $crate::pipeline::shader::Stages::from_array([
					$(
						$crate::pipeline::shader::Stage::$stage
					),*
				]);
			}
		)*

		/// The `T` layout type is a well typed version of `Self` if
		/// - it matches `Self` (every binding defined in `T` is defined in `Self`)
		/// - every binding defined in `Self` is defined in `T` and its
		///   definition matches the constraints defined in `Self`
		///   (descriptor type, count and accessing shader stages).
		unsafe impl<T> $crate::descriptor::set::layout::WellTypedBy<T> for $id where
			T: $crate::descriptor::set::layout::Matches<Self>,
			$(
				T: $crate::descriptor::set::layout::BindLocation<$loc>,
				<T as $crate::descriptor::set::layout::BindLocation<$loc>>::Type:
					$crate::descriptor::ty::WellTyped<
						{ $crate::untyped_descriptor_set_layout!(@ty $ty) },
						{ $crate::untyped_descriptor_set_layout!(@count $ty) },
						{
							$crate::pipeline::shader::Stages::from_array([
								$(
									$crate::pipeline::shader::Stage::$stage
								),*
							])
						}
					>
			),*
		{}

		impl $id {
			pub fn new(device: &std::sync::Arc<$crate::Device>) -> Result<Self, $crate::OomError> {
				Ok(Self($crate::descriptor::set::layout::Raw::new(
					device,
					&[
						$(
							$crate::descriptor::set::layout::Binding::new(
								$loc,
								$crate::untyped_descriptor_set_layout!(@ty $ty),
								$crate::untyped_descriptor_set_layout!(@count $ty),
								$crate::pipeline::shader::Stages::from_array([
									$(
										$crate::pipeline::shader::Stage::$stage
									),*
								])
							)
						),*
					]
				)?))
			}
		}
	};
	(@ty [$ty:ident; $count:literal]) => { $crate::descriptor::Type::$ty };
	(@ty $ty:ident) => { $crate::descriptor::Type::$ty };
	(@count [$ty:ident; $count:literal]) => { $count };
	(@count $ty:ident) => { 1u32 };
}

#[macro_export]
macro_rules! descriptor_set_layout {
	{
		$vis:vis struct $id:ident {
			$($loc:literal => $ty:ident ($($stage:ident),*)),*
		}
	} => {
		$vis struct $id;

		unsafe impl<T> $crate::descriptor::set::layout::Matches<T> for $id where
			$(
				T: $crate::descriptor::set::layout::BindUntypedLocation<$loc>
			),*
		{}
	};
}

pub unsafe trait BindUntypedLocation<const N: u32> {
	const TYPE: crate::descriptor::Type;
	const COUNT: u32;
	const STAGES: shader::Stages;
}

/// Property of a typed layout having a binding
/// at location `N`.
pub unsafe trait BindLocation<const N: u32> {
	/// Binding type.
	type Type;
}

pub unsafe trait BindSet<const N: u32> {
	type Type;
}

/// Raw layout instance.
pub struct Raw {
	device: Arc<Device>,
	handle: Handle
}

impl Raw {
	/// Create a new descriptor set layout instance for the given device.
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

/// List of descriptor set layouts.
/// 
/// ## Safety
/// 
/// The handles **must** never change and refer to valid
/// descriptor set layouts.
pub unsafe trait Layouts {
	type Handles<'a>: AsRef<[Handle]>;

	fn handles(&self) -> Self::Handles<'_>;
}

/// Creates a list of untyped descriptor set layouts.
/// 
/// ## Example
/// 
/// ```
/// untyped_descriptor_set_layouts! {
///   pub struct MyDescriptorSetLayouts {
///     0 : MySet0Layout,
///     1 : MySet1Layout
///   }
/// }
/// ```
#[macro_export]
macro_rules! untyped_descriptor_set_layouts {
	{
		$vis:vis struct $id:ident {
			$($loc:literal : $ty:ty),*
		}
	} => {
		$vis struct $id {
			layouts: (
				$(std::sync::Arc<$ty>),*
			),
			handles: [
				$crate::descriptor::set::layout::Handle;
				$crate::untyped_descriptor_set_layouts!(@count [$($loc),*])
			]
		}

		$(
			unsafe impl $crate::descriptor::set::layout::BindSet<$loc> for $id {
				type Type = $ty;
			}
		)*

		unsafe impl $crate::descriptor::set::Layouts for $id {
			type Handles<'a> = &'a [$crate::descriptor::set::layout::Handle];

			fn handles(&self) -> &[$crate::descriptor::set::layout::Handle] {
				&self.handles
			}
		}

		// unsafe impl<T> $crate::descriptor::set::layout::WellTypedBy<T> for $id where
		// 	T: $crate::descriptor::set::layout::Matches<Self>,
		// 	$(
		// 		T: $crate::descriptor::set::layout::BindTypedLocation<$loc>,
		// 		<T as $crate::descriptor::set::layout::BindTypedLocation<$loc>>::Type:
		// 			$crate::descriptor::ty::ArrayLen<$count>
		// 	),*
		// {}
	};
	(@count []) => { 0usize };
	(@count [$a:literal]) => { 1usize };
	(@count [$a:literal, $($b:literal),+]) => { 1usize + $crate::untyped_descriptor_set_layouts!(@count [$($b),+]) };
}

#[macro_export]
macro_rules! descriptor_set_layouts {
	{
		$vis:vis struct $id:ident {
			$($loc:literal : $ty:ty),*
		}
	} => {
		$vis struct $id;

		$(
			unsafe impl $crate::descriptor::set::layout::BindSet<$loc> for $id {
				type Type = $ty;
			}
		)*

		unsafe impl<T> $crate::descriptor::set::layout::Matches<T> for $id where
			$(
				T: $crate::descriptor::set::layout::BindSet<$loc>,
				$ty: $crate::descriptor::set::layout::Matches<<T as $crate::descriptor::set::layout::BindSet<$loc>>::Type>
			),*
		{}
	};
}

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