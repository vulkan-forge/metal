use ash::vk;

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
	pub(crate) fn into_vulkan(self) -> vk::PrimitiveTopology {
		vk::PrimitiveTopology::from_raw(self as i32)
	}
}

pub struct InputAssembly(vk::PipelineInputAssemblyStateCreateInfo);

impl InputAssembly {
	pub fn new(topology: Topology, primitive_restart: bool) -> InputAssembly {
		InputAssembly(
			vk::PipelineInputAssemblyStateCreateInfo {
				topology: topology.into_vulkan(),
				primitive_restart_enable: if primitive_restart {
					vk::TRUE
				} else {
					vk::FALSE
				},
				..Default::default()
			}
		)
	}

	pub fn as_vulkan(&self) -> &vk::PipelineInputAssemblyStateCreateInfo {
		&self.0
	}
}