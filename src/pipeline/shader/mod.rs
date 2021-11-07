pub mod module;
pub mod stages;
mod entry_point;

pub use module::Module;
pub use stages::{
	Stage,
	Stages,
	StageSet
};
pub use entry_point::EntryPoint;

#[macro_export]
macro_rules! pipeline_shader_stages {
	(
		$vis:vis struct $id:ident : $layout:ty {
			$(
				$stage:ty
			),*
		}
	) => {
		pub struct $id;

		// ...
	};
}