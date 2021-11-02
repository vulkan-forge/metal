pub unsafe trait CompatibleWith<T> {}

/// Bind a set on the given index.
/// 
/// This is used to ensure that no two sets are bounds
/// to the same index.
pub unsafe trait BindSet<const INDEX: u32> {
	/// Set layout.
	type Layout;
}

#[macro_export]
macro_rules! shader_module_layout {
	{
		$vis:vis struct $id:ident {
			$($index:literal => $set_ty:ty),*
		}
	} => {
		$vis struct $id;

		$(
			unsafe impl $crate::pipeline::shader::module::layout::BindSet<$index> for $id {
				type Layout = $set_ty;
			}
		)*

		// unsafe impl<T> $crate::pipeline::shader::module::layout::CompatibleWith<T> for $id
		// where
		// 	$(
		// 		T: $crate::pipeline::shader::module::layout::BindSet<$index>,
		// 		$set_ty: $crate::pipeline::shader::module::layout::CompatibleWith<<Self as $crate::pipeline::shader::module::layout::BindSet<$index>>::Layout>
		// 	),*
		// {}
	};
}