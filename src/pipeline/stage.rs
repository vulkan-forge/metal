use ash::vk;
use super::shader;

pub type Flags = vk::PipelineStageFlags;

/// Shader stages.
pub unsafe trait Stages: PartialStages {}

/// Possibly incomplete list sequence of shader stages.
pub unsafe trait PartialStages {
	fn for_each<F>(&self, f: F) where F: FnMut(Stage) -> ();
}

pub struct Stage<'a> {
	pub entry_point: &'a shader::EntryPoint,
	pub ty: shader::Stage
}

impl<'a> Stage<'a> {
	fn new(entry_point: &'a shader::EntryPoint, ty: shader::Stage) -> Stage<'a> {
		Stage {
			entry_point,
			ty
		}
	}
}

/// Vertex shader stage.
pub struct Vertex<T: AfterVertex> {
	entry_point: shader::EntryPoint,
	next: T
}

impl<T: AfterVertex> Vertex<T> {
	pub unsafe fn new(entry_point: shader::EntryPoint, next: T) -> Self {
		Self {
			entry_point,
			next
		}
	}
}

unsafe impl<T: AfterVertex> PartialStages for Vertex<T> {
	fn for_each<F>(&self, mut f: F) where F: FnMut(Stage) -> () {
		f(Stage::new(&self.entry_point, shader::Stage::Vertex));
		self.next.for_each(f)
	}
}

unsafe impl<T: AfterVertex> Stages for Vertex<T> {}

/// Type of stages that can follow a vertex shader stage.
/// 
/// A vertex shader stage can be followed by
/// tesselation shader stages,
/// a geometry shader stage or
/// a fragment shader stage.
pub unsafe  trait AfterVertex: PartialStages {}
unsafe impl AfterVertex for Fragment {}
unsafe impl<T: AfterGeometry> AfterVertex for Geometry<T> {}
unsafe impl<T: AfterTesselation> AfterVertex for Tesselation<T> {}

/// Tesselation shader stages.
pub struct Tesselation<T: AfterTesselation> {
	control: shader::EntryPoint,
	evaluation: shader::EntryPoint,
	next: T
}

impl<T: AfterTesselation> Tesselation<T> {
	pub unsafe fn new(control: shader::EntryPoint, evaluation: shader::EntryPoint, next: T) -> Self {
		Self {
			control,
			evaluation,
			next
		}
	}
}

unsafe impl<T: AfterTesselation> PartialStages for Tesselation<T> {
	fn for_each<F>(&self, mut f: F) where F: FnMut(Stage) -> () {
		f(Stage::new(&self.control, shader::Stage::TesselationControl));
		f(Stage::new(&self.evaluation, shader::Stage::TesselationEvaluation));
		self.next.for_each(f)
	}
}

/// Type of stages that can follow tesselation shader stages.
/// 
/// Tesselation shader stages can be followed by
/// a geometry shader stage or
/// a fragment shader stage.
pub unsafe trait AfterTesselation: PartialStages {}
unsafe impl<T: AfterGeometry> AfterTesselation for Geometry<T> {}
unsafe impl AfterTesselation for Fragment {}

/// Geometry shader stage.
pub struct Geometry<T: AfterGeometry> {
	entry_point: shader::EntryPoint,
	next: T
}

impl<T: AfterGeometry> Geometry<T> {
	pub unsafe fn new(entry_point: shader::EntryPoint, next: T) -> Self {
		Self {
			entry_point,
			next
		}
	}
}

unsafe impl<T: AfterGeometry> PartialStages for Geometry<T> {
	fn for_each<F>(&self, mut f: F) where F: FnMut(Stage) -> () {
		f(Stage::new(&self.entry_point, shader::Stage::Geometry));
		self.next.for_each(f)
	}
}

/// Type of stages that can follow a geometry shader stage.
/// 
/// A geometry shader stage can be followed by
/// a fragment shader stage.
pub unsafe trait AfterGeometry: PartialStages {}
unsafe impl AfterGeometry for Fragment {}

/// Fragment shader stage.
pub struct Fragment {
	entry_point: shader::EntryPoint
}

impl Fragment {
	pub unsafe fn new(entry_point: shader::EntryPoint) -> Self {
		Self {
			entry_point
		}
	}
}

unsafe impl PartialStages for Fragment {
	fn for_each<F>(&self, mut f: F) where F: FnMut(Stage) -> () {
		f(Stage::new(&self.entry_point, shader::Stage::Fragment))
	}
}