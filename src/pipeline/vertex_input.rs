use ash::vk;
use crate::Format;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Rate {
	/// Specifies that vertex attribute addressing is a function of the vertex index.
	Vertex = vk::VertexInputRate::VERTEX.as_raw(),

	/// Specifies that vertex attribute addressing is a function of the instance index.
	Instance = vk::VertexInputRate::INSTANCE.as_raw()
}

impl Rate {
	#[inline]
	pub(crate) fn into_vulkan(self) -> vk::VertexInputRate {
		vk::VertexInputRate::from_raw(self as i32)
	}
}

pub struct Binding(vk::VertexInputBindingDescription); // This MUST be homomorphic to `vk::VertexInputBindingDescription`.

impl Binding {
	pub fn new(binding: u32, stride: u32, input_rate: Rate) -> Binding {
		Binding(vk::VertexInputBindingDescription {
			binding,
			stride,
			input_rate: input_rate.into_vulkan()
		})
	}
}

pub struct Attribute(vk::VertexInputAttributeDescription); // This MUST be homomorphic to `vk::VertexInputAttributeDescription`.

impl Attribute {
	pub fn new(location: u32, binding: u32, format: Format, offset: u32) -> Attribute {
		Attribute(vk::VertexInputAttributeDescription {
			location,
			binding,
			format: format.into_vulkan(),
			offset
		})
	}
}

pub struct VertexInput {
	bindings: Vec<Binding>,
	attributes: Vec<Attribute>,
	handle: vk::PipelineVertexInputStateCreateInfo
}

impl VertexInput {
	pub fn new() -> Self {
		VertexInput {
			bindings: Vec::new(),
			attributes: Vec::new(),
			handle: vk::PipelineVertexInputStateCreateInfo::default()
		}
	}

	pub fn add_binding(&mut self, binding: Binding) -> u32 {
		let index = self.bindings.len() as u32;
		self.bindings.push(binding);

		self.handle.vertex_binding_description_count = self.bindings.len() as u32;
		self.handle.p_vertex_binding_descriptions = self.bindings.as_ptr() as *const _;

		index
	}

	pub fn add_attribute(&mut self, attr: Attribute) {
		self.attributes.push(attr);

		self.handle.vertex_attribute_description_count = self.attributes.len() as u32;
		self.handle.p_vertex_attribute_descriptions = self.attributes.as_ptr() as *const _;
	}

	/// Returns the vulkan representation.
	pub(crate) fn as_vulkan(&self) -> &vk::PipelineVertexInputStateCreateInfo {
		&self.handle
	}
}