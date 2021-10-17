use ash::vk;

// pub unsafe trait Topology {
// 	const VULKAN: vk::PrimitiveTopology;
// }

// pub mod topology {
// 	use ash::vk;

// 	macro_rules! topologies {
// 		($($name:ident = $variant:ident),*) => {
// 			$(
// 				pub struct $name;

// 				unsafe impl super::Topology for $name {
// 					const VULKAN: vk::PrimitiveTopology = vk::PrimitiveTopology::$variant;
// 				}
// 			)*
// 		};
// 	}

// 	topologies! {
// 		PointList = POINT_LIST,
// 		LineList = LINE_LIST,
// 		LineStrip = LINE_STRIP,
// 		TriangleList = TRIANGLE_LIST,
// 		TriangleStrip = TRIANGLE_STRIP,
// 		TriangleFan = TRIANGLE_FAN,
// 		LineListWithAdjacency = LINE_LIST_WITH_ADJACENCY,
// 		LineStripWithAdjacency = LINE_STRIP_WITH_ADJACENCY,
// 		TriangleListWithAdjacency = TRIANGLE_LIST_WITH_ADJACENCY,
// 		TriangleStripWithAdjacency = TRIANGLE_STRIP_WITH_ADJACENCY,
// 		PatchList = PATCH_LIST
// 	}
// }

// pub trait InputAssembly {
// 	type Topology: Topology;

// 	const PRIMITIVE_RESTART: bool;

// 	fn vulkan() -> vk::PipelineInputAssemblyStateCreateInfo {
// 		vk::PipelineInputAssemblyStateCreateInfo {
// 			topology: Self::Topology::VULKAN,
// 			primitive_restart_enable: if Self::PRIMITIVE_RESTART { vk::TRUE } else { vk::FALSE },
// 			..Default::default()
// 		}
// 	}
// }

// macro_rules! assemblies {
// 	($($name:ident),*) => {
// 		$(
// 			pub struct $name;

// 			impl InputAssembly for $name {
// 				type Topology = topology::$name;

// 				const PRIMITIVE_RESTART: bool = false;
// 			}
// 		)*
// 	};
// }

// assemblies! {
// 	PointList,
// 	LineList,
// 	LineStrip,
// 	TriangleList,
// 	TriangleStrip,
// 	TriangleFan,
// 	LineListWithAdjacency,
// 	LineStripWithAdjacency,
// 	TriangleListWithAdjacency,
// 	TriangleStripWithAdjacency,
// 	PatchList
// }

#[repr(transparent)]
pub struct InputAssembly(vk::PipelineInputAssemblyStateCreateInfo);

impl InputAssembly {
	pub fn new(topology: Topology, primitive_restart: bool) -> Self {
		Self(vk::PipelineInputAssemblyStateCreateInfo {
			topology: topology.into_vulkan(),
			primitive_restart_enable: if primitive_restart { vk::TRUE } else { vk::FALSE },
			..Default::default()
		})
	}

	pub fn as_vulkan(&self) -> &vk::PipelineInputAssemblyStateCreateInfo {
		&self.0
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Topology {
	PointList = vk::PrimitiveTopology::POINT_LIST.as_raw(),
	LineList = vk::PrimitiveTopology::LINE_LIST.as_raw(),
	LineStrip = vk::PrimitiveTopology::LINE_STRIP.as_raw(),
	TriangleList = vk::PrimitiveTopology::TRIANGLE_LIST.as_raw(),
	TriangleStrip = vk::PrimitiveTopology::TRIANGLE_STRIP.as_raw(),
	TriangleFan = vk::PrimitiveTopology::TRIANGLE_FAN.as_raw(),
	LineListWithAdjacency = vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY.as_raw(),
	LineStripWithAdjacency = vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY.as_raw(),
	TriangleListWithAdjacency = vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY.as_raw(),
	TriangleStripWithAdjacency = vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY.as_raw(),
	PatchList = vk::PrimitiveTopology::PATCH_LIST.as_raw()
}

impl Topology {
	pub(crate) const fn into_vulkan(self) -> vk::PrimitiveTopology {
		vk::PrimitiveTopology::from_raw(self as i32)
	}
}