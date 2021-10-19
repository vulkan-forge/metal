/// Untyped shader module descriptor set layout.
pub trait UntypedLayout {
	/// Static definition.
	const DEFINITION: Const;
}

/// Const descriptor set layout location definition.
pub struct ConstLocation {
	/// Location index.
	pub location: u32,

	/// Descriptor type.
	pub descriptor_ty: crate::descriptor::Type,

	/// Descriptor count.
	pub count: u32
}

impl ConstLocation {
	const fn eq(&self, other: &Self) -> bool {
		self.location == other.location && self.descriptor_ty as i32 == other.descriptor_ty as i32 && self.count == other.count
	}
}

/// Const descriptor set layout definition.
pub struct Const(pub &'static [ConstLocation]);

impl Const {
	pub const fn len(&self) -> usize {
		self.0.len()
	}

	pub const fn eq(&self, other: &Self) -> bool {
		if self.len() == other.len() {
			let mut i = 0;
			while i < self.len() {
				if !self.0[i].eq(&other.0[i]) {
					return false
				}

				i += 1;
			}

			true
		} else {
			false
		}
	}
}

/// Typed shader module descriptor set layout.
pub trait Layout {
	/// Static definition.
	type Definition;
}

#[macro_export]
macro_rules! untyped_shader_module_descriptor_set {
	{
		$vis:vis struct $id:ident {
			$($loc:literal => [$descriptor_ty:ident; $count:literal]),*
		}
	} => {
		$vis struct $id;

		impl $crate::pipeline::shader::module::descriptor::set::UntypedLayout for $id {
			const DEFINITION: $crate::pipeline::shader::module::descriptor::set::layout::Const = $crate::pipeline::shader::module::descriptor::set::layout::Const(&[
				$(
					$crate::pipeline::shader::module::descriptor::set::layout::ConstLocation {
						location: $loc,
						descriptor_ty: $crate::descriptor::Type::$descriptor_ty,
						count: $count
					}
				),*
			]);
		}
	}
}

/// Create a type that describes the typed layout
/// of a descriptor set accessed by the shader module.
/// The syntax is as follows:
/// 
/// ```text
/// $vis:vis struct $id:ident : $untyped_id:ty {
///   $($loc:literal => $ty:ty),*
/// }
/// ```
/// 
/// Each location type `$ty` must be a descriptor type
/// from the chosen from the `magma::descriptor::ty` module.
/// The typed layout must match the untyped layout specified
/// by `$untyped_id`.
/// Otherwise, an error is thrown at compile time.
/// 
/// ## Example
/// 
/// ```
/// magma::untyped_shader_module_descriptor_set! {
///   pub struct UntypedSetLayout {
///     0 => [UniformBuffer; 1]
///   }
/// }
/// 
/// magma::shader_module_descriptor_set! {
///   pub struct SetLayout : UntypedSetLayout {
///     0 => magma::descriptor::ty::UniformBuffer<Matrix4x4>
///   }
/// }
/// ```
#[macro_export]
macro_rules! shader_module_descriptor_set {
	{
		$vis:vis struct $id:ident : $untyped_id:ty {
			$($loc:literal => $ty:ty),*
		}
	} => {
		$vis struct $id;

		const _: () = {
			let expected_definition = <$untyped_id as $crate::pipeline::shader::module::descriptor::set::UntypedLayout>::DEFINITION;

			let found_definition = $crate::pipeline::shader::module::descriptor::set::layout::Const(&[
				$(
					$crate::pipeline::shader::module::descriptor::set::layout::ConstLocation {
						location: $loc,
						descriptor_ty: <$ty as $crate::descriptor::SizedType>::TYPE,
						count: <$ty as $crate::descriptor::SizedType>::COUNT
					}
				),*
			]);

			if !found_definition.eq(&expected_definition) {
				let msg = $crate::const_format::formatcp!(
					"{} set layout does not match the untyped set layout defined by {}",
					$crate::std::stringify!($id),
					$crate::std::stringify!($untyped_id)
				);
				$crate::std::panic!("{}", msg)
			}
		};

		impl $crate::pipeline::shader::module::descriptor::set::Layout for $id {
			type Definition = (
				$(
					[$ty; $loc]
				),*
			);
		}
	}
}

/// Ensure that the shader modules descriptor sets are
/// compatible (included in) the given descriptor sets.
#[macro_export]
macro_rules! shader_module_descriptor_set_compatible {
	( $set:ty : $module_set:ty { $($loc:literal => $ty:ident $([$count:literal])*),* } ) => {
		// Check that the module descriptor set definition matches the
		// original definition.
		$crate::static_assertions::assert_type_eq_all!(
			(
				$(
					[[$ty; $count]; $loc]
				),*
			),
			<$module_set as $crate::pipeline::shader::module::descriptor::set::Layout>::Map
		);

		$(
			// Check that the shader module descriptor set is compatible with
			// the pipeline layout descriptor set of same index.
			$crate::static_assertions::assert_type_eq_all!(
				[$ty; $count]:
				[
					<$sets as $crate::descriptor::set::layout::BindLocation<$loc>>::Type;
					<$sets as $crate::descriptor::set::layout::BindLocation<$loc>>::COUNT
				]
			);
		)*

		// Once all it checked, we can safely declare compatibility.
		unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<$set> for $module_set {}
	};
}

/// Shader module descriptor set layouts.
unsafe trait Layouts {
	/// Type that describes which descriptor set is bound to which index.
	/// 
	/// It must be a tuple of `[S; INDEX]` for each set binding.
	type Map;
}

#[macro_export]
macro_rules! shader_module_descriptor_sets {
	{
		$vis:vis struct $id:ident {
			$($index:literal : $ty:ty),*
		}
	} => {
		$vis struct $id;

		unsafe impl $crate::pipeline::shader::descriptor::set::Layouts for $id {
			type Map = (
				$(
					[$ty; $index]
				),*
			);
		}
	};
}

/// Layout compatibility property.
unsafe trait CompatibleWith<T> {}

unsafe trait Compatibility<T> {
	const COMPATIBLE: bool;
}

unsafe impl<A, B> Compatibility<A> for B {
	default const COMPATIBLE: bool = false;
}

unsafe impl<A, B: CompatibleWith<A>> Compatibility<A> for B {
	default const COMPATIBLE: bool = true;
}

/// Ensure that the shader modules descriptor sets are
/// compatible (included in) the given descriptor sets.
#[macro_export]
macro_rules! shader_module_descriptor_sets_compatible {
	( $sets:ty : $module_sets:ty { $($index:literal : $ty:ty),* } ) => {
		// Check that the module sets definition matches the
		// original definition.
		$crate::static_assertions::assert_type_eq_all!(
			(
				$(
					[$ty; $index]
				),*
			),
			<$module_sets as $crate::pipeline::shader::module::descriptor::set::Layouts>::Map
		);

		$(
			// Check that each shader module descriptor set is compatible with
			// the pipeline layout descriptor set of same index.
			$crate::static_assertions::assert_impl_all!(
				$ty:
				$crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<
					<$sets as $crate::descriptor::set::layout::BindSet<$index>>::Set
				>
			);
		)*

		// Once all it checked, we can safely declare compatibility.
		unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<$sets> for $module_sets {}
	};
}