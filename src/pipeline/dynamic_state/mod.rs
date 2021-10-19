use ash::vk;

pub use viewports::DynamicState as Viewports;
pub use scissors::DynamicState as Scissors;

/// Viewports/Scissors dynamic states.
pub trait ViewportsScissors: 'static {
	type Viewports: Viewports;
	type Scissors: Scissors + scissors::CompatibleWith<Self::Viewports>;

	fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<vk::DynamicState>) {
		if Self::Viewports::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::VIEWPORT)
		}

		if Self::Scissors::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::SCISSOR)
		}
	}
}

pub use line_width::DynamicState as LineWidth;
pub use depth_bias::DynamicState as DepthBias;

/// Rasterization dynamic states.
pub trait Rasterization: 'static {
	type LineWidth: LineWidth;
	type DepthBias: DepthBias;

	fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<vk::DynamicState>) {
		if Self::LineWidth::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::LINE_WIDTH)
		}

		if Self::DepthBias::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::DEPTH_BIAS)
		}
	}
}

/// Blend constants dynamic state.
pub use blend_constants::DynamicState as BlendConstants;

/// Depth bounds dynamic state.
pub use depth_bounds::DynamicState as DepthBounds;

pub use stencil_compare_mask::DynamicState as StencilCompareMask;
pub use stencil_write_mask::DynamicState as StencilWriteMask;
pub use stencil_reference::DynamicState as StencilReference;

/// Stencil test dynamic states.
pub trait StencilTest: 'static {
	type StencilCompareMask: StencilCompareMask;
	type StencilWriteMask: StencilWriteMask;
	type StencilReference: StencilReference;

	fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<vk::DynamicState>) {
		if Self::StencilCompareMask::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::STENCIL_COMPARE_MASK)
		}

		if Self::StencilWriteMask::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::STENCIL_WRITE_MASK)
		}

		if Self::StencilReference::IS_DYNAMIC {
			dynamic_states.push(vk::DynamicState::STENCIL_REFERENCE)
		}
	}
}

/// Trait that exposes the dynamic states that must be
/// updated.
pub unsafe trait Set<P: super::Graphics> {
	fn viewports(&self) -> Option<&[super::Viewport]>;

	fn scissors(&self) -> Option<&[super::Scissor]>;
}

pub mod viewports {
	use crate::pipeline::Viewport;

	pub trait InitialType {
		fn ptr(&self) -> *const Viewport;
	}

	impl InitialType for Viewport {
		fn ptr(&self) -> *const Viewport {
			self as *const _
		}
	}

	impl<const N: usize> InitialType for [Viewport; N] {
		fn ptr(&self) -> *const Viewport {
			self.as_ptr()
		}
	}

	impl InitialType for () {
		fn ptr(&self) -> *const Viewport {
			std::ptr::null()
		}
	}

	pub unsafe trait DynamicState {
		const COUNT: u32;

		const IS_DYNAMIC: bool;

		type InitialType: InitialType;
	}

	pub struct Static<const N: u32>;

	macro_rules! static_state {
		($($n:literal),*) => {
			$(
				unsafe impl DynamicState for Static<$n> {
					const COUNT: u32 = $n;
			
					const IS_DYNAMIC: bool = false;
			
					type InitialType = [Viewport; $n];
				}
			)*
		};
	}

	unsafe impl DynamicState for Static<1> {
		const COUNT: u32 = 1;

		const IS_DYNAMIC: bool = false;

		type InitialType = Viewport;
	}

	// NOTE: 16 is the maximum known limit for `maxViewport`.
	// See: http://vulkan.gpuinfo.org/displaydevicelimit.php?name=maxViewports&platform=all
	static_state!(2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

	pub struct Dynamic<const N: u32>;

	unsafe impl<const N: u32> DynamicState for Dynamic<N> {
		const COUNT: u32 = N;

		const IS_DYNAMIC: bool = true;

		type InitialType = ();
	}
}

pub mod scissors {
	use crate::pipeline::Scissor;

	pub trait InitialType {
		fn ptr(&self) -> *const Scissor;
	}

	impl InitialType for Scissor {
		fn ptr(&self) -> *const Scissor {
			self as *const _
		}
	}

	impl<const N: usize> InitialType for [Scissor; N] {
		fn ptr(&self) -> *const Scissor {
			self.as_ptr()
		}
	}

	impl InitialType for () {
		fn ptr(&self) -> *const Scissor {
			std::ptr::null()
		}
	}

	pub unsafe trait DynamicState {
		const COUNT: u32;

		const IS_DYNAMIC: bool;

		type InitialType: InitialType;
	}

	pub struct Static<const N: u32>;

	macro_rules! static_state {
		($($n:literal),*) => {
			$(
				unsafe impl DynamicState for Static<$n> {
					const COUNT: u32 = $n;
			
					const IS_DYNAMIC: bool = false;
			
					type InitialType = [Scissor; $n];
				}
			)*
		};
	}

	unsafe impl DynamicState for Static<1> {
		const COUNT: u32 = 1;

		const IS_DYNAMIC: bool = false;

		type InitialType = Scissor;
	}

	// NOTE: 16 is the maximum known limit for `maxViewport`.
	// See: http://vulkan.gpuinfo.org/displaydevicelimit.php?name=maxViewports&platform=all
	static_state!(2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

	pub struct Dynamic<const N: u32>;

	unsafe impl<const N: u32> DynamicState for Dynamic<N> {
		const COUNT: u32 = N;

		const IS_DYNAMIC: bool = true;

		type InitialType = ();
	}

	pub unsafe trait CompatibleWith<V> {}

	unsafe impl<const N: u32> CompatibleWith<super::viewports::Static<N>> for Static<N> {}
	unsafe impl<const N: u32> CompatibleWith<super::viewports::Static<N>> for Dynamic<N> {}
	unsafe impl<const N: u32> CompatibleWith<super::viewports::Dynamic<N>> for Static<N> {}
	unsafe impl<const N: u32> CompatibleWith<super::viewports::Dynamic<N>> for Dynamic<N> {}
}

pub mod line_width {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod depth_bias {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod blend_constants {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;

		fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<super::vk::DynamicState>);
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;

		fn build_vulkan_dynamic_states(_: &mut Vec<super::vk::DynamicState>) {
			// not a dynamic state.
		}
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;

		fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<super::vk::DynamicState>) {
			dynamic_states.push(super::vk::DynamicState::BLEND_CONSTANTS)
		}
	}
}

pub mod depth_bounds {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;

		fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<super::vk::DynamicState>);
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;

		fn build_vulkan_dynamic_states(_: &mut Vec<super::vk::DynamicState>) {
			// not dynamic state.
		}
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;

		fn build_vulkan_dynamic_states(dynamic_states: &mut Vec<super::vk::DynamicState>) {
			dynamic_states.push(super::vk::DynamicState::DEPTH_BOUNDS)
		}
	}
}

pub mod stencil_compare_mask {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod stencil_write_mask {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub mod stencil_reference {
	pub unsafe trait DynamicState {
		const IS_DYNAMIC: bool;
	}

	pub struct Static;

	unsafe impl DynamicState for Static {
		const IS_DYNAMIC: bool = false;
	}

	pub struct Dynamic;

	unsafe impl DynamicState for Dynamic {
		const IS_DYNAMIC: bool = true;
	}
}

pub struct StaticRasterization;

impl Rasterization for StaticRasterization {
	type LineWidth = line_width::Static;
	type DepthBias = depth_bias::Static;
}

pub struct StaticStencilTest;

impl StencilTest for StaticStencilTest {
	type StencilCompareMask = stencil_compare_mask::Static;
	type StencilWriteMask = stencil_write_mask::Static;
	type StencilReference = stencil_reference::Static;
}

pub struct StaticViewportsScissors<const N: u32>;

impl<const N: u32> ViewportsScissors for StaticViewportsScissors<N>
where
	viewports::Static<N>: viewports::DynamicState,
	scissors::Static<N>: scissors::DynamicState {
	type Viewports = viewports::Static<N>;
	type Scissors = scissors::Static<N>;
}

pub struct DynamicViewportsScissors<const N: u32>;

impl<const N: u32> ViewportsScissors for DynamicViewportsScissors<N> {
	type Viewports = viewports::Dynamic<N>;
	type Scissors = scissors::Dynamic<N>;
}

// unsafe impl<const N: usize, P: super::Graphics<ViewportsScissors=DynamicViewportsScissors<N>>> Set<P> for ([super::Viewport; N], [super::Scissor; N]) {
// 	fn viewports(&self) -> Option<&[super::Viewport]> {
// 		Some(&self.0)
// 	}

// 	fn scissors(&self) -> Option<&[super::Scissor]> {
// 		Some(&self.1)
// 	}
// }

// unsafe impl<P: super::Graphics<ViewportsScissors=DynamicViewportsScissors<1>>> Set<P> for (super::Viewport, super::Scissor) {
// 	fn viewports(&self) -> Option<&[super::Viewport]> {
// 		Some(std::slice::from_ref(&self.0))
// 	}

// 	fn scissors(&self) -> Option<&[super::Scissor]> {
// 		Some(std::slice::from_ref(&self.1))
// 	}
// }