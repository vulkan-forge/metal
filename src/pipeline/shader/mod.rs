pub mod module;
mod stage;
mod entry_point;

pub use module::Module;
pub use stage::{
	Stage,
	Stages
};
pub use entry_point::EntryPoint;

#[macro_export]
macro_rules! shader_stages {
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