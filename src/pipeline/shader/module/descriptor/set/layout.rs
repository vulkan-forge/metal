pub unsafe trait BindLocation<const LOC: u32, const COUNT: u32> {}

pub unsafe trait BindTypedLocation<const N: u32> {
	type Type: crate::descriptor::ty::Array;
}

// /// Creates a type that describes an untyped shader module
// /// descriptor set layout.
// #[macro_export]
// macro_rules! untyped_shader_module_descriptor_set {
// 	{
// 		$vis:vis struct $id:ident {
// 			$($loc:literal => [$descriptor_ty:ident; $count:literal]),*
// 		}
// 	} => {
// 		$vis struct $id;

// 		$(
// 			unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::BindLocation<$loc, $count> for $id {}
// 		)*

// 		unsafe impl<T> $crate::pipeline::shader::module::descriptor::set::layout::WellTypedBy<T> for $id where
// 		T: $crate::pipeline::shader::module::descriptor::set::layout::Matches<Self>,
// 		$(
// 			T: $crate::pipeline::shader::module::descriptor::set::layout::BindTypedLocation<$loc>,
// 			<T as $crate::pipeline::shader::module::descriptor::set::layout::BindTypedLocation<$loc>>::Type: $crate::descriptor::ty::ArrayLen<$count>
// 		),*
// 		{}
// 	}
// }

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
///   pub struct SetLayout {
///     0 => magma::descriptor::ty::UniformBuffer<Matrix4x4>
///   }
/// }
/// ```
#[macro_export]
macro_rules! shader_module_descriptor_set {
	{
		$vis:vis struct $id:ident {
			$($loc:literal => $ty:ty),*
		}
	} => {
		$vis struct $id;

		$(
			unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::BindTypedLocation<$loc> for $id {
				type Type = $ty;
			}
		)*

		// unsafe impl<T> $crate::pipeline::shader::module::descriptor::set::layout::Matches<T> for $id where
		// $(
		// 	T: $crate::descriptor::set::layout::BindLocation<$loc, {<$ty as $crate::descriptor::ty::Array>::COUNT}>,
		// ),*
		// {}
	}
}

// /// Ensure that the shader modules descriptor sets are
// /// compatible (included in) the given descriptor sets.
// #[macro_export]
// macro_rules! shader_module_descriptor_set_compatible {
// 	( $set:ty : $module_set:ty { $($loc:literal => $ty:ident $([$count:literal])*),* } ) => {
// 		// Check that the module descriptor set definition matches the
// 		// original definition.
// 		$crate::static_assertions::assert_type_eq_all!(
// 			(
// 				$(
// 					[[$ty; $count]; $loc]
// 				),*
// 			),
// 			<$module_set as $crate::pipeline::shader::module::descriptor::set::Layout>::Map
// 		);

// 		$(
// 			// Check that the shader module descriptor set is compatible with
// 			// the pipeline layout descriptor set of same index.
// 			$crate::static_assertions::assert_type_eq_all!(
// 				[$ty; $count]:
// 				[
// 					<$sets as $crate::descriptor::set::layout::BindLocation<$loc>>::Type;
// 					<$sets as $crate::descriptor::set::layout::BindLocation<$loc>>::COUNT
// 				]
// 			);
// 		)*

// 		// Once all it checked, we can safely declare compatibility.
// 		unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<$set> for $module_set {}
// 	};
// }

// /// Layout compatibility property.
// unsafe trait CompatibleWith<T> {}

// unsafe trait Compatibility<T> {
// 	const COMPATIBLE: bool;
// }

// unsafe impl<A, B> Compatibility<A> for B {
// 	default const COMPATIBLE: bool = false;
// }

// unsafe impl<A, B: CompatibleWith<A>> Compatibility<A> for B {
// 	default const COMPATIBLE: bool = true;
// }

// /// Ensure that the shader modules descriptor sets are
// /// compatible (included in) the given descriptor sets.
// #[macro_export]
// macro_rules! shader_module_descriptor_sets_compatible {
// 	( $sets:ty : $module_sets:ty { $($index:literal : $ty:ty),* } ) => {
// 		// Check that the module sets definition matches the
// 		// original definition.
// 		$crate::static_assertions::assert_type_eq_all!(
// 			(
// 				$(
// 					[$ty; $index]
// 				),*
// 			),
// 			<$module_sets as $crate::pipeline::shader::module::descriptor::set::Layouts>::Map
// 		);

// 		$(
// 			// Check that each shader module descriptor set is compatible with
// 			// the pipeline layout descriptor set of same index.
// 			$crate::static_assertions::assert_impl_all!(
// 				$ty:
// 				$crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<
// 					<$sets as $crate::descriptor::set::layout::BindSet<$index>>::Set
// 				>
// 			);
// 		)*

// 		// Once all it checked, we can safely declare compatibility.
// 		unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<$sets> for $module_sets {}
// 	};
// }