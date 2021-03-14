use ash::vk;

pub unsafe trait Topology {
	const VULKAN: vk::PrimitiveTopology;
}

pub mod topology {
	use ash::vk;

	macro_rules! topologies {
		($($name:ident = $variant:ident),*) => {
			$(
				pub struct $name;

				unsafe impl super::Topology for $name {
					const VULKAN: vk::PrimitiveTopology = vk::PrimitiveTopology::$variant;
				}
			)*
		};
	}

	topologies! {
		PointList = POINT_LIST,
		LineList = LINE_LIST,
		LineStrip = LINE_STRIP,
		TriangleList = TRIANGLE_LIST,
		TriangleStrip = TRIANGLE_STRIP,
		TriangleFan = TRIANGLE_FAN,
		LineListWithAdjacency = LINE_LIST_WITH_ADJACENCY,
		LineStripWithAdjacency = LINE_STRIP_WITH_ADJACENCY,
		TriangleListWithAdjacency = TRIANGLE_LIST_WITH_ADJACENCY,
		TriangleStripWithAdjacency = TRIANGLE_STRIP_WITH_ADJACENCY,
		PatchList = PATCH_LIST
	}
}

pub trait InputAssembly {
	type Topology: Topology;

	const PRIMITIVE_RESTART: bool;

	fn vulkan() -> vk::PipelineInputAssemblyStateCreateInfo {
		vk::PipelineInputAssemblyStateCreateInfo {
			topology: Self::Topology::VULKAN,
			primitive_restart_enable: if Self::PRIMITIVE_RESTART { vk::TRUE } else { vk::FALSE },
			..Default::default()
		}
	}
}

macro_rules! assemblies {
	($($name:ident),*) => {
		$(
			pub struct $name;

			impl InputAssembly for $name {
				type Topology = topology::$name;

				const PRIMITIVE_RESTART: bool = false;
			}
		)*
	};
}

assemblies! {
	PointList,
	LineList,
	LineStrip,
	TriangleList,
	TriangleStrip,
	TriangleFan,
	LineListWithAdjacency,
	LineStripWithAdjacency,
	TriangleListWithAdjacency,
	TriangleStripWithAdjacency,
	PatchList
}