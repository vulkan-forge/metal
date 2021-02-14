use ash::vk;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum DynamicState {
	Viewport = vk::DynamicState::VIEWPORT.as_raw(),
	Scissor = vk::DynamicState::SCISSOR.as_raw(),
	LineWidth = vk::DynamicState::LINE_WIDTH.as_raw(),
	DepthBias = vk::DynamicState::DEPTH_BIAS.as_raw(),
	BlendConstants = vk::DynamicState::BLEND_CONSTANTS.as_raw(),
	DepthBounds = vk::DynamicState::DEPTH_BOUNDS.as_raw(),
	StencilCompareMask = vk::DynamicState::STENCIL_COMPARE_MASK.as_raw(),
	StencilWriteMask = vk::DynamicState::STENCIL_WRITE_MASK.as_raw(),
	StencilReference = vk::DynamicState::STENCIL_REFERENCE.as_raw()
}

impl DynamicState {
	pub(crate) fn into_vulkan(self) -> vk::DynamicState {
		vk::DynamicState::from_raw(self as i32)
	}
}

pub unsafe trait DynamicStates {
	fn for_each<F>(f: F) where F: FnMut(DynamicState) -> ();
}

unsafe impl DynamicStates for () {
	fn for_each<F>(_f: F) where F: FnMut(DynamicState) -> () {
		// no dynamic states.
	}
}

pub struct Viewport<D: DynamicStates>(PhantomData<D>);
pub unsafe trait WithViewport {}
unsafe impl<D: DynamicStates> WithViewport for Viewport<D> {}

unsafe impl<D: DynamicStates> DynamicStates for Viewport<D> {
	fn for_each<F>(mut f: F) where F: FnMut(DynamicState) -> () {
		f(DynamicState::Viewport);
		D::for_each(f)
	}
}

pub struct Scissor<D: DynamicStates>(PhantomData<D>);
pub unsafe trait WithScissor {}
unsafe impl<D: DynamicStates> WithScissor for Scissor<D> {}

unsafe impl<D: DynamicStates> DynamicStates for Scissor<D> {
	fn for_each<F>(mut f: F) where F: FnMut(DynamicState) -> () {
		f(DynamicState::Scissor);
		D::for_each(f)
	}
}