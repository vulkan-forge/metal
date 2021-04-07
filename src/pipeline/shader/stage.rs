use ash::vk;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Stage {
	Vertex = vk::ShaderStageFlags::VERTEX.as_raw(),
	TesselationControl = vk::ShaderStageFlags::TESSELLATION_CONTROL.as_raw(),
	TesselationEvaluation = vk::ShaderStageFlags::TESSELLATION_EVALUATION.as_raw(),
	Geometry = vk::ShaderStageFlags::GEOMETRY.as_raw(),
	Fragment = vk::ShaderStageFlags::FRAGMENT.as_raw(),
	Compute = vk::ShaderStageFlags::COMPUTE.as_raw()
}

impl Stage {
	pub(crate) fn into_vulkan(self) -> vk::ShaderStageFlags {
		vk::ShaderStageFlags::from_raw(self as u32)
	}
}

/// Describes which shader stages have access to a descriptor.
// TODO: add example with BitOr
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Stages {
	/// `True` means that the descriptor will be used by the vertex shader.
	pub vertex: bool,
	/// `True` means that the descriptor will be used by the tessellation control shader.
	pub tessellation_control: bool,
	/// `True` means that the descriptor will be used by the tessellation evaluation shader.
	pub tessellation_evaluation: bool,
	/// `True` means that the descriptor will be used by the geometry shader.
	pub geometry: bool,
	/// `True` means that the descriptor will be used by the fragment shader.
	pub fragment: bool,
	/// `True` means that the descriptor will be used by the compute shader.
	pub compute: bool,
}

impl Stages {
	/// Creates a `Stages` struct will all stages set to `true`.
	// TODO: add example
	#[inline]
	pub const fn all() -> Stages {
		Stages {
			vertex: true,
			tessellation_control: true,
			tessellation_evaluation: true,
			geometry: true,
			fragment: true,
			compute: true,
		}
	}

	/// Creates a `Stages` struct will all stages set to `false`.
	// TODO: add example
	#[inline]
	pub const fn none() -> Stages {
		Stages {
			vertex: false,
			tessellation_control: false,
			tessellation_evaluation: false,
			geometry: false,
			fragment: false,
			compute: false,
		}
	}

	/// Creates a `Stages` struct with all graphics stages set to `true`.
	// TODO: add example
	#[inline]
	pub const fn all_graphics() -> Stages {
		Stages {
			vertex: true,
			tessellation_control: true,
			tessellation_evaluation: true,
			geometry: true,
			fragment: true,
			compute: false,
		}
	}

	/// Creates a `Stages` struct with the vertex shader stage set to `true`.
	#[inline]
	pub const fn vertex_shader() -> Stages {
		Stages {
			vertex: true,
			tessellation_control: false,
			tessellation_evaluation: false,
			geometry: false,
			fragment: false,
			compute: false,
		}
	}

	/// Creates a `Stages` struct with the fragment shader stage set to `true`.
	#[inline]
	pub const fn fragment_shader() -> Stages {
		Stages {
			vertex: false,
			tessellation_control: false,
			tessellation_evaluation: false,
			geometry: false,
			fragment: true,
			compute: false,
		}
	}

	/// Creates a `Stages` struct with the compute stage set to `true`.
	// TODO: add example
	#[inline]
	pub const fn compute() -> Stages {
		Stages {
			vertex: false,
			tessellation_control: false,
			tessellation_evaluation: false,
			geometry: false,
			fragment: false,
			compute: true,
		}
	}

	/// Checks whether we have more stages enabled than `other`.
	// TODO: add example
	#[inline]
	pub const fn is_superset_of(&self, other: &Stages) -> bool {
		(self.vertex || !other.vertex)
			&& (self.tessellation_control || !other.tessellation_control)
			&& (self.tessellation_evaluation || !other.tessellation_evaluation)
			&& (self.geometry || !other.geometry)
			&& (self.fragment || !other.fragment)
			&& (self.compute || !other.compute)
	}

	/// Checks whether any of the stages in `self` are also present in `other`.
	// TODO: add example
	#[inline]
	pub const fn intersects(&self, other: &Stages) -> bool {
		(self.vertex && other.vertex)
			|| (self.tessellation_control && other.tessellation_control)
			|| (self.tessellation_evaluation && other.tessellation_evaluation)
			|| (self.geometry && other.geometry)
			|| (self.fragment && other.fragment)
			|| (self.compute && other.compute)
	}

	#[inline]
	pub(crate) const fn into_vulkan(self) -> vk::ShaderStageFlags {
		let mut result = 0;

		if self.vertex {
			result |= vk::ShaderStageFlags::VERTEX.as_raw();
		}
		if self.tessellation_control {
			result |= vk::ShaderStageFlags::TESSELLATION_CONTROL.as_raw();
		}
		if self.tessellation_evaluation {
			result |= vk::ShaderStageFlags::TESSELLATION_EVALUATION.as_raw();
		}
		if self.geometry {
			result |= vk::ShaderStageFlags::GEOMETRY.as_raw();
		}
		if self.fragment {
			result |= vk::ShaderStageFlags::FRAGMENT.as_raw();
		}
		if self.compute {
			result |= vk::ShaderStageFlags::COMPUTE.as_raw();
		}

		vk::ShaderStageFlags::from_raw(result)
	}
}

impl std::ops::BitOr for Stages {
	type Output = Stages;

	#[inline]
	fn bitor(self, other: Stages) -> Stages {
		Stages {
			vertex: self.vertex || other.vertex,
			tessellation_control: self.tessellation_control || other.tessellation_control,
			tessellation_evaluation: self.tessellation_evaluation || other.tessellation_evaluation,
			geometry: self.geometry || other.geometry,
			fragment: self.fragment || other.fragment,
			compute: self.compute || other.compute,
		}
	}
}