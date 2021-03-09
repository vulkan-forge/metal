use ash::vk;

/// Type describing the dynamic states of a graphics pipeline.
pub trait DynamicStates: 'static {
	type Viewport: DynamicState;
	type Scissor: DynamicState;
	type LineWidth: DynamicState;
	type DepthBias: DynamicState;
	type BlendConstants: DynamicState;
	type DepthBounds: DynamicState;
	type StencilCompareMask: DynamicState;
	type StencilWriteMask: DynamicState;
	type StencilReference: DynamicState;

	fn vulkan() -> Vec<vk::DynamicState> {
		let mut states = Vec::new();

		if Self::Viewport::IS_DYNAMIC {
			states.push(vk::DynamicState::VIEWPORT)
		}

		if Self::Scissor::IS_DYNAMIC {
			states.push(vk::DynamicState::SCISSOR)
		}

		if Self::LineWidth::IS_DYNAMIC {
			states.push(vk::DynamicState::LINE_WIDTH)
		}

		if Self::DepthBias::IS_DYNAMIC {
			states.push(vk::DynamicState::DEPTH_BIAS)
		}

		if Self::BlendConstants::IS_DYNAMIC {
			states.push(vk::DynamicState::BLEND_CONSTANTS)
		}

		if Self::DepthBounds::IS_DYNAMIC {
			states.push(vk::DynamicState::DEPTH_BOUNDS)
		}

		if Self::StencilCompareMask::IS_DYNAMIC {
			states.push(vk::DynamicState::STENCIL_COMPARE_MASK)
		}

		if Self::StencilWriteMask::IS_DYNAMIC {
			states.push(vk::DynamicState::STENCIL_WRITE_MASK)
		}

		if Self::StencilReference::IS_DYNAMIC {
			states.push(vk::DynamicState::STENCIL_REFERENCE)
		}

		states
	}
}

pub trait DynamicState {
	const IS_DYNAMIC: bool;
}

pub mod viewport {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod scissor {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod line_width {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod depth_bias {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod blend_constants {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod depth_bounds {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod stencil_compare_mask {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod stencil_write_mask {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod stencil_reference {
	pub struct Static;

	impl super::DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	impl super::DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

impl DynamicStates for () {
	type Viewport = viewport::Static;
	type Scissor = scissor::Static;
	type LineWidth = line_width::Static;
	type DepthBias = depth_bias::Static;
	type BlendConstants = blend_constants::Static;
	type DepthBounds = depth_bounds::Static;
	type StencilCompareMask = stencil_compare_mask::Static;
	type StencilWriteMask = stencil_write_mask::Static;
	type StencilReference = stencil_reference::Static;
}

pub struct DynamicViewportAndScissor;

impl DynamicStates for DynamicViewportAndScissor {
	type Viewport = viewport::Dynamic;
	type Scissor = scissor::Dynamic;
	type LineWidth = line_width::Static;
	type DepthBias = depth_bias::Static;
	type BlendConstants = blend_constants::Static;
	type DepthBounds = depth_bounds::Static;
	type StencilCompareMask = stencil_compare_mask::Static;
	type StencilWriteMask = stencil_write_mask::Static;
	type StencilReference = stencil_reference::Static;
}