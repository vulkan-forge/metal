use ash::vk;

macro_rules! dynamic_states {
	($($name:ident : $variant:ident ($vulkan:ident)),*) => {
		#[derive(Clone, Copy, Default, Debug)]
		pub struct DynamicStates {
			$(
				$name: bool
			),*
		}

		impl DynamicStates {
			pub fn new() -> DynamicStates {
				Self::default()
			}

			pub fn empty() -> DynamicStates {
				Self::default()
			}

			pub fn contains(&self, state: DynamicState) -> bool {
				match state {
					$(
						DynamicState::$variant => self.$name
					),*
				}
			}

			pub fn add(&mut self, state: DynamicState) {
				match state {
					$(
						DynamicState::$variant => self.$name = true
					),*
				}
			}

			pub(crate) fn into_vulkan(self) -> Vec<vk::DynamicState> {
				let mut vec = Vec::new();

				$(
					if self.$name {
						vec.push(vk::DynamicState::$vulkan)
					}
				)*

				vec
			}
		}

		#[derive(Clone, Copy, Debug)]
		#[repr(i32)]
		pub enum DynamicState {
			$(
				$variant = vk::DynamicState::$vulkan.as_raw()
			),*
		}
	};
}

dynamic_states! {
	viewport: Viewport (VIEWPORT),
	scissor: Scissor (SCISSOR),
	line_width: LineWidth (LINE_WIDTH),
	depth_bias: DepthBias (DEPTH_BIAS),
	blend_constants: BlendConstants (BLEND_CONSTANTS),
	depth_bounds: DepthBounds (DEPTH_BOUNDS),
	stencil_compare_mask: StencilCompareMask (STENCIL_COMPARE_MASK),
	stencil_write_mask: StencilWriteMask (STENCIL_WRITE_MASK),
	stencil_reference: StencilReference (STENCIL_REFERENCE)
}

impl From<DynamicState> for DynamicStates {
	fn from(i: DynamicState) -> DynamicStates {
		let mut o = DynamicStates::new();
		o.add(i);
		o
	}
}

macro_rules! from_tuple {
	(($($id:ident),*) : $ty:ty) => {
		impl From<$ty> for DynamicStates {
			fn from(($($id),*): $ty) -> DynamicStates {
				let mut o = DynamicStates::new();
				$(
					o.add($id);
				)*
				o
			}
		}
	};
}

from_tuple!((a, b): (DynamicState, DynamicState));
from_tuple!((a, b, c): (DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d): (DynamicState, DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d, e): (DynamicState, DynamicState, DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d, e, f): (DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d, e, f, g): (DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d, e, f, g, h): (DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState));
from_tuple!((a, b, c, d, e, f, g, h, i): (DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState, DynamicState));