pub unsafe trait BindLocation<const LOC: u32> {
	type Binding;
}

/// Compatible with descriptor set.
pub unsafe trait CompatibleWith<S, const STAGE: crate::pipeline::shader::Stage> {}

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
/// magma::shader_module_descriptor_set! {
///   pub struct SetLayout {
///     0 => magma::descriptor::ty::UniformBuffer<Matrix4x4>
///   }
/// }
/// ```
#[macro_export]
macro_rules! shader_module_descriptor_set_layout {
	{
		$vis:vis struct $id:ident {
			$($loc:literal => $ty:ty),*
		}
	} => {
		$vis struct $id;

		$(
			unsafe impl $crate::pipeline::shader::module::descriptor::set::layout::BindLocation<$loc> for $id {
				type Binding = $ty;
			}
		)*

		unsafe impl<T, const STAGE: $crate::pipeline::shader::Stage> $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<T, STAGE> for $id
		where
			$(
				T: $crate::descriptor::set::layout::BindLocation<$loc, Binding=$ty>,
				<T as $crate::descriptor::set::layout::BindLocation<$loc>>::Stages: $crate::pipeline::shader::stages::Contains<STAGE>,
			)*
		{}
	}
}

#[macro_export]
macro_rules! shader_module_descriptor_set_layouts {
	{
		$vis:vis struct $id:ident {
			$($index:literal => $set_ty:ty),*
		}
	} => {
		$vis struct $id;

		$(
			unsafe impl $crate::descriptor::set::layout::BindSet<$index> for $id {
				type Set = $set_ty;
			}
		)*

		unsafe impl<T, const STAGE: $crate::pipeline::shader::Stage> $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<T, STAGE> for $id
		where
			$(
				T: $crate::descriptor::set::layout::BindSet<$index>,
				$set_ty: $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<<T as $crate::descriptor::set::layout::BindSet<$index>>::Set, STAGE>
			)*
		{}
	}
}