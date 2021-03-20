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

pub unsafe trait Set<S: DynamicStates> {
	fn viewports(&self) -> Option<&[super::Viewport]>;

	fn scissors(&self) -> Option<&[super::Scissor]>;
}

pub mod viewport {
	pub struct Static<const N: usize>;

	impl<const N: usize> super::DynamicState for Static<N> {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic<const N: usize>;

	impl<const N: usize> super::DynamicState for Dynamic<N> {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod scissor {
	pub struct Static<const N: usize>;

	impl<const N: usize> super::DynamicState for Static<N> {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic<const N: usize>;

	impl<const N: usize> super::DynamicState for Dynamic<N> {
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
	type Viewport = viewport::Static<1>;
	type Scissor = scissor::Static<1>;
	type LineWidth = line_width::Static;
	type DepthBias = depth_bias::Static;
	type BlendConstants = blend_constants::Static;
	type DepthBounds = depth_bounds::Static;
	type StencilCompareMask = stencil_compare_mask::Static;
	type StencilWriteMask = stencil_write_mask::Static;
	type StencilReference = stencil_reference::Static;
}

unsafe impl Set<()> for () {
	fn viewports(&self) -> Option<&[super::Viewport]> {
		None
	}

	fn scissors(&self) -> Option<&[super::Scissor]> {
		None
	} 
}

pub struct DynamicViewportAndScissor<const N: usize>;

impl<const N: usize> DynamicStates for DynamicViewportAndScissor<N> {
	type Viewport = viewport::Dynamic<N>;
	type Scissor = scissor::Dynamic<N>;
	type LineWidth = line_width::Static;
	type DepthBias = depth_bias::Static;
	type BlendConstants = blend_constants::Static;
	type DepthBounds = depth_bounds::Static;
	type StencilCompareMask = stencil_compare_mask::Static;
	type StencilWriteMask = stencil_write_mask::Static;
	type StencilReference = stencil_reference::Static;
}

unsafe impl<const N: usize> Set<DynamicViewportAndScissor<N>> for ([super::Viewport; N], [super::Scissor; N]) {
	fn viewports(&self) -> Option<&[super::Viewport]> {
		Some(&self.0)
	}

	fn scissors(&self) -> Option<&[super::Scissor]> {
		Some(&self.1)
	}
}

unsafe impl Set<DynamicViewportAndScissor<1>> for (super::Viewport, super::Scissor) {
	fn viewports(&self) -> Option<&[super::Viewport]> {
		Some(std::slice::from_ref(&self.0))
	}

	fn scissors(&self) -> Option<&[super::Scissor]> {
		Some(std::slice::from_ref(&self.1))
	}
}