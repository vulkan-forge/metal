use ash::vk;
use crate::Format;
// use super::{
// 	input_assembly,
// 	InputAssembly
// };

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Rate {
	/// Specifies that vertex attribute addressing is a function of the vertex index.
	Vertex = vk::VertexInputRate::VERTEX.as_raw(),

	/// Specifies that vertex attribute addressing is a function of the instance index.
	Instance = vk::VertexInputRate::INSTANCE.as_raw()
}

impl Rate {
	#[inline]
	pub(crate) const fn into_vulkan(self) -> vk::VertexInputRate {
		vk::VertexInputRate::from_raw(self as i32)
	}
}

#[repr(transparent)]
pub struct Binding(vk::VertexInputBindingDescription); // This MUST be homomorphic to `vk::VertexInputBindingDescription`.

impl Binding {
	pub const fn new(binding: u32, stride: u32, input_rate: Rate) -> Binding {
		Binding(vk::VertexInputBindingDescription {
			binding,
			stride,
			input_rate: input_rate.into_vulkan()
		})
	}
}

#[repr(transparent)]
pub struct Attribute(vk::VertexInputAttributeDescription); // This MUST be homomorphic to `vk::VertexInputAttributeDescription`.

impl Attribute {
	pub const fn new(location: u32, binding: u32, format: Format, offset: u32) -> Attribute {
		Attribute(vk::VertexInputAttributeDescription {
			location,
			binding,
			format: format.into_vulkan(),
			offset
		})
	}
}

pub unsafe trait Bindings {
	/// Input bindings list.
	const LIST: &'static [Binding];
}

/// Vertex input.
pub unsafe trait VertexInput {
	/// Input bindings list.
	type Bindings: Bindings;

	/// Input attributes.
	const ATTRIBUTES: &'static [Attribute];
}

/// No input bindings.
unsafe impl Bindings for () {
	const LIST: &'static [Binding] = &[];
}

/// No vertex input.
unsafe impl VertexInput for () {
	type Bindings = ();
	const ATTRIBUTES: &'static [Attribute] = &[];
}

/// Vertex input type.
/// 
/// Such type provides an `FIELDS` constant that
/// provides the offset and format of each field of the
/// structure.
/// 
/// This trait can be easily and safely implemented
/// using the [`vertex_input_type!`] macro.
/// 
/// ## Safety
/// 
/// The `Fields` type **must** be a structure where:
/// - for each field of the implementor type structure,
///   there is a `(usize, Format)` field in the `Offsets` type with
///   the exact same name whose value is a tuple containing the offset
///   and format of the original field.
/// - there is no more fields that in the original type.
pub unsafe trait Type {
	/// Type storing the offset and format of every field of the structure.
	type Fields;

	/// Constant exposing the offset and format of every field of the structure.
	const FIELDS: Self::Fields;
}

/// Create a vertex input type.
/// 
/// The produced type uses the C representation (`#[repr(C)]`).
/// 
/// ## Example
/// 
/// ```
/// vertex_input_type! {
///   /// Position + normal input.
///   pub struct PositionNormal {
///     /// Position of the vertex.
///     pub position: [f32; 3],
/// 
///     /// Normal of the vertex.
///     pub normal: [f32; 3]
///   }
/// }
/// ```
#[macro_export]
macro_rules! vertex_input_type {
	{
		$(#[$meta:meta])*
		$vis:vis struct $id:ident {
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis
				$field_id:ident : $field_ty:ty
			),*
		}
	} => {
		$(#[$meta])*
		#[repr(C)]
		$vis struct $id {
			$(
				$(#[$field_meta])*
				$field_vis
                $field_id : $field_ty
			),*
		}

		#[allow(nonstandard_style)]
		const _: () = {
			pub struct Fields {
				$(
					$field_vis
					$field_id: (usize, $crate::Format)
				),*
			}

			pub struct ConstOffsets;

			unsafe impl $crate::pipeline::vertex_input::Type for $id {
				type Fields = Fields;

				const FIELDS: Fields = Fields {
					$(
						$field_id: (ConstOffsets::$field_id, <$field_ty as $crate::FormattedType>::FORMAT)
					),*
				};
			}

			const END_OF_PREV_FIELD: usize = 0;

			$crate::vertex_input_type!(
				@offsets [$($field_id : $field_ty,)*]
			);
		};
	};
	(@offsets []) => { () };
	(@offsets [$field_id:ident : $field_ty:ty, $($other_ids:ident : $other_tys:ty,)*]) => {
		impl ConstOffsets {
			const $field_id: usize = {
				let align = $crate::std::mem::align_of::<$field_ty>();
				let trail = END_OF_PREV_FIELD % align;
				END_OF_PREV_FIELD + (align - trail) * [1, 0][(trail == 0) as usize]
			};
		}

		const _: () = {
			const END_OF_PREV_FIELD: usize = ConstOffsets::$field_id + $crate::std::mem::size_of::<$field_ty>();
			$crate::vertex_input_type!(
				@offsets [$($other_ids : $other_tys,)*]
			);
		};
	};
}

/// Specifies the type of the `N`th binding
/// in a binding map type.
/// 
/// In a vertex input description,
/// bindings are specified using a map,
/// associating integers to `Binding` instances.
/// To statically check the well-formedness of the
/// vertex input using Rust's type system,
/// we need to represent this map as a type.
/// This can be done with any type (here called `Bindings`) by using
/// this trait to specify the type of every binding,
/// where `<Bindings as Bind<N>>::Target` is the type
/// of each item of the binding of index `N`.
/// 
/// Because only one implementation of `Bind<N>` is possible
/// for each `N`, this guaranies that each binding
/// has a unique type.
/// 
/// ## Example
/// 
/// ```
/// pub struct Bindings;
/// 
/// impl magma::pipeline::vertex_input::Bindings for Bindings {
///   const 
/// }
/// 
/// unsafe impl Bind<0> for Bindings {
///   const RATE: usize = 
/// 
///   type Item = [f32; 3];
/// }
/// 
/// unsafe impl Bind<1> for Bindings {
///   type Item = f32;
/// }
/// ```
/// 
/// This example can be simplified using the
/// [`vertex_input_bindings!`] macro as follows:
/// 
/// ```
/// magma::vertex_input_bindings! {
///   pub struct Bindings {
///     0 => [f32; 3],
///     1 => f32
///   }
/// }
/// ```
pub trait Bind<const N: usize> {
	type Item;
}

/// Defines a type that represent a vertex input bindings map.
/// 
/// A vertex input bindings map type statically indexes
/// each binding and specifying the type of each binding item type,
/// using the [`Bind`] trait.
/// 
/// ## Example
/// 
/// ```
/// magma::vertex_input_bindings! {
///   pub struct Bindings {
///     0 => [f32; 3],
///     1 => f32
///   }
/// }
/// ```
#[macro_export]
macro_rules! vertex_input_bindings {
	{
		$(#[$meta:meta])*
		$vis:vis struct $id:ident {
			$(
				$index:literal => $binding_ty:ty $([$rate:ident])*
			),*
		}
	} => {
		$vis struct $id;

		unsafe impl $crate::pipeline::vertex_input::Bindings for $id {
			const LIST: &'static [$crate::pipeline::vertex_input::Binding] = &[
				$(
					$crate::pipeline::vertex_input::Binding::new(
						$index,
						$crate::std::mem::size_of::<$binding_ty>() as u32,
						$crate::vertex_input_bindings!( @rate $([$rate])* )
					)
				),*
			];
		}

		$(
			impl $crate::pipeline::vertex_input::Bind<$index> for $id {
				type Item = $binding_ty;
			}
		)*
	};
	{ @rate [instance] } => { $crate::pipeline::vertex_input::Rate::Instance };
	{ @rate [vertex] } => { $crate::pipeline::vertex_input::Rate::Vertex };
	{ @rate } => { $crate::pipeline::vertex_input::Rate::Vertex };
}

/// Creates a vertex input's attributes.
/// 
/// ## Example
/// 
/// ```
/// vertex_input_type! {
///   pub struct PositionNormal {
///     pub position: [f32; 3],
///     pub normal: [f32; 3]
///   }
/// }
/// 
/// vertex_input! {
///   pub struct InputAttributes for (InputBinding) {
///     0 => 0.position,
///     1 => 0.normal
///   }
/// }
/// ```
#[macro_export]
macro_rules! vertex_input {
	{
		$(#[$meta:meta])*
		$vis:vis struct $id:ident for $bindings_ty:ty {
			$($location:literal => $binding_index:literal $($accessor:tt)*),*
		}
	} => {
		$(#[$meta])*
		$vis struct $id;

		unsafe impl $crate::pipeline::VertexInput for $id {
			type Bindings = $bindings_ty;

			const ATTRIBUTES: &'static [$crate::pipeline::vertex_input::Attribute] = &[
				$(
					{
						let (offset, format) = $crate::vertex_input!( @accessor [$bindings_ty, $binding_index] $($accessor)* );
						$crate::pipeline::vertex_input::Attribute::new(
							$location,
							$binding_index,
							format,
							offset
						)
					}
				),*
			];
		}
	};
	{ @accessor [$bindings_ty:ty, $binding_index:literal] . $binding_field:ty } => { // access a field.
		<<$bindings_ty as $crate::pipeline::vertex_input::Bind<$binding_index>>::Item as $crate::pipeline::vertex_input::Type>::FIELDS.$binding_field
	};
	{ @accessor [$bindings_ty:ty, $binding_index:literal] } => { // access the item itself.
		(0, <<$bindings_ty as $crate::pipeline::vertex_input::Bind<$binding_index>>::Item as $crate::FormattedType>::FORMAT)
	};
}