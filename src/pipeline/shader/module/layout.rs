/// Shader module layout compatible with the pipeline layout `T`
/// for the given stage `STAGE`.
pub unsafe trait CompatibleWith<T, const STAGE: crate::pipeline::shader::Stage> {}

#[macro_export]
macro_rules! shader_module_layout {
	{
		$vis:vis struct $id:ident {
			type PushConstants = $push_constants:ty;
			type DescriptorSets = $descriptor_sets:ty; 
		}
	} => {
		$vis struct $id;

		unsafe impl<T: $crate::pipeline::Layout, const STAGE: $crate::pipeline::shader::Stage> $crate::pipeline::shader::module::layout::CompatibleWith<T, STAGE> for $id
		where
			$descriptor_sets: $crate::pipeline::shader::module::descriptor::set::layout::CompatibleWith<T::DescriptorSets, STAGE>
		{}
	};
}