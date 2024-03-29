use ash::vk;

macro_rules! stages {
	($(
		$(#[$meta:meta])*
		$elem:ident => $variant:ident = $vulkan_const:ident
	),*) => {
		#[derive(PartialEq, Eq, Clone, Copy, Debug)]
		#[repr(u32)]
		pub enum Stage {
			$(
				$(#[$meta])*
				$variant = vk::ShaderStageFlags::$vulkan_const.as_raw()
			),*
		}

		$(
			shader_stage_set! {
				pub struct $variant {
					$variant
				}
			}
		)*

		impl Stage {
			pub(crate) fn into_vulkan(self) -> vk::ShaderStageFlags {
				vk::ShaderStageFlags::from_raw(self as u32)
			}
		}

		#[derive(Debug, Copy, Clone, PartialEq, Eq)]
		pub struct Stages {
			$(
				$(#[$meta])*
				pub $elem: bool
			),*
		}

		impl Stages {
			/// Creates a `Stages` struct will all stages set to `true`.
			// TODO: add example
			#[inline]
			pub const fn all() -> Self {
				Self {
					$(
						$elem: true
					),*
				}
			}

			/// Creates a `Stages` struct will all stages set to `false`.
			// TODO: add example
			#[inline]
			pub const fn none() -> Self {
				Self {
					$(
						$elem: false
					),*
				}
			}

			#[inline]
			pub const fn from_array<const N: usize>(array: [Stage; N]) -> Self {
				let mut stages = Self::none();
				let mut i = 0;

				while i < N {
					match array[i] {
						$(
							Stage::$variant => stages.$elem = true
						),+
					}

					i += 1;
				}

				stages
			}

			#[inline]
			pub fn len(&self) -> usize {
				let mut len = 0;
				$(
					if self.$elem {
						len += 1
					}
				)+

				len
			}

			#[inline]
			pub const fn contains(&self, stage: Stage) -> bool {
				match stage {
					$(
						Stage::$variant => self.$elem
					),+
				}
			}

			#[inline]
			pub const fn into_contains(self, stage: Stage) -> bool {
				match stage {
					$(
						Stage::$variant => self.$elem
					),+
				}
			}

			/// Checks whether we have more stages enabled than `other`.
			// TODO: add example
			#[inline]
			pub const fn is_superset_of(&self, other: &Stages) -> bool {
				stages!( @superset (self, other) [$($elem,)*] )
			}

			/// Checks whether any of the stages in `self` are also present in `other`.
			// TODO: add example
			#[inline]
			pub const fn intersects(&self, other: &Stages) -> bool {
				stages!( @intersects (self, other) [$($elem,)*] )
			}

			#[inline]
			pub fn insert(&mut self, stage: Stage) {
				match stage {
					$(
						Stage::$variant => self.$elem = true
					),+
				}
			}

			#[inline]
			pub(crate) const fn into_vulkan(self) -> vk::ShaderStageFlags {
				let mut result = 0;

				$(
					if self.$elem {
						result |= vk::ShaderStageFlags::$vulkan_const.as_raw();
					}
				)*

				vk::ShaderStageFlags::from_raw(result)
			}
		}

		impl std::ops::BitOr for Stages {
			type Output = Stages;

			#[inline]
			fn bitor(self, rhs: Stages) -> Stages {
				Stages {
					$(
						$elem: self.$elem || rhs.$elem,
					)+
				}
			}
		}

		impl std::ops::BitOrAssign for Stages {
			#[inline]
			fn bitor_assign(&mut self, rhs: Stages) {
				$(
					self.$elem = self.$elem || rhs.$elem;
				)+
			}
		}

		impl std::iter::Extend<Stage> for Stages {
			fn extend<I: IntoIterator<Item=Stage>>(&mut self, iter: I) {
				for stage in iter {
					self.insert(stage)
				}
			}
		}

		impl std::iter::FromIterator<Stage> for Stages {
			fn from_iter<I: IntoIterator<Item=Stage>>(iter: I) -> Self {
				let mut stages = Self::none();
				stages.extend(iter);
				stages
			}
		}
	};
	(@superset ($a:ident, $b:ident) [$elem:ident, $($other_elem:ident,)+]) => {
		($a.$elem || !$b.$elem) && stages!( @superset ($a, $b) [$($other_elem,)*] )
	};
	(@superset ($a:ident, $b:ident) [$elem:ident,]) => {
		($a.$elem || !$b.$elem)
	};
	(@intersects ($a:ident, $b:ident) [$elem:ident, $($other_elem:ident,)+]) => {
		($a.$elem && $b.$elem) || stages!( @intersects ($a, $b) [$($other_elem,)*] )
	};
	(@intersects ($a:ident, $b:ident) [$elem:ident,]) => {
		($a.$elem && $b.$elem)
	};
}

#[macro_export]
macro_rules! shader_stage_set {
	{
		$vis:vis struct $id:ident {
			$($stage:ident),*
		}
	} => {
		$vis struct $id;

		impl $crate::pipeline::shader::StageSet for $id {
			const STAGES: $crate::pipeline::shader::Stages = $crate::pipeline::shader::Stages::from_array([
				$(
					$crate::pipeline::shader::Stage::$stage
				),*
			]);
		}

		$(
			unsafe impl $crate::pipeline::shader::stages::Contains<{$crate::pipeline::shader::Stage::$stage}> for $id {}
		)*

		unsafe impl<S: StageSet> Subset<S> for $id
		where
			$(
				S: $crate::pipeline::shader::stages::Contains<{$crate::pipeline::shader::Stage::$stage}>
			),*
		{}
	};
}

stages! {
	vertex => Vertex = VERTEX,
	tessellation_control => TesselationControl = TESSELLATION_CONTROL,
	tessellation_evaluation => TesselationEvaluation = TESSELLATION_EVALUATION,
	geometry => Geometry = GEOMETRY,
	fragment => Fragment = FRAGMENT,
	compute => Compute = COMPUTE
}

shader_stage_set! {
	pub struct AllGraphics {
		Vertex, TesselationControl, TesselationEvaluation, Geometry, Fragment
	}
}

impl Stages {
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
}

pub trait StageSet {
	const STAGES: Stages;
}

/// Stage set that contains the given `STAGE`.
/// 
/// ## Safety
/// 
/// This trait must not be implemented unless
/// `Self::STAGES` actual contains `STAGE`.
pub unsafe trait Contains<const STAGE: Stage>: StageSet {}

pub unsafe trait Subset<S: StageSet>: StageSet {}