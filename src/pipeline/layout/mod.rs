use ash::{
	vk,
	version::DeviceV1_0
};
use std::{
	sync::Arc,
	marker::PhantomData
};
use crate::{
	OomError,
	Device,
	Resource
};

pub mod set;
pub mod push_constant;

pub use set::Set;
pub use push_constant::PushConstants;

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError),
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub unsafe trait Layout: Resource<Handle=vk::PipelineLayout> {
	type PushConstants: PushConstants;
	type Sets;
}

unsafe impl<L: std::ops::Deref> Layout for L where L::Target: Layout {
	type PushConstants = <L::Target as Layout>::PushConstants;
	type Sets = <L::Target as Layout>::Sets;
}

/// Layout without descriptor sets.
pub struct NoSets<P: PushConstants>(Raw<P>);

impl<P: PushConstants> NoSets<P> {
	pub fn new(device: &Arc<Device>) -> Result<Self, CreationError> {
		Ok(NoSets(Raw::new(device, &[])?))
	}
}

unsafe impl<P: PushConstants> Resource for NoSets<P> {
	type Handle = vk::PipelineLayout;

	fn handle(&self) -> vk::PipelineLayout {
		self.0.handle()
	}
}

unsafe impl<P: PushConstants> Layout for NoSets<P> {
	type PushConstants = P;
	type Sets = ();
}

/// Empty layout.
pub type Empty = NoSets<()>;

/// Layout compatibility marker.
/// 
/// This correspond to the notion of ["compatible for set N"](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#descriptorsets-compatibility)
/// in the vulkan specification.
pub unsafe trait CompatibleWith<L>: Layout {
	// ...
}

/// The `NoSets` layout is compatible with any layout with identical push constant ranges.
unsafe impl<P: PushConstants, L: Layout<PushConstants=P>> CompatibleWith<L> for NoSets<P> {}

/// The `NoSets` layout is compatible with any layout with identical push constant ranges.
unsafe impl<N: std::ops::Deref<Target=NoSets<P>>, P: PushConstants, L: Layout<PushConstants=P>> CompatibleWith<L> for N {}

pub struct Raw<C: PushConstants> {
	device: Arc<Device>,
	handle: vk::PipelineLayout,
	pc: PhantomData<C>
}

impl<C: PushConstants> Raw<C> {
	pub fn new(device: &Arc<Device>, set_layouts: &[Set]) -> Result<Raw<C>, CreationError> {
		let vk_set_layouts: Vec<_> = set_layouts.iter().map(|l| l.handle()).collect();
		
		let push_constant_ranges = C::RANGES;

		let infos = vk::PipelineLayoutCreateInfo {
			flags: vk::PipelineLayoutCreateFlags::empty(),
			set_layout_count: vk_set_layouts.len() as u32,
			p_set_layouts: vk_set_layouts.as_ptr(),
			push_constant_range_count: push_constant_ranges.len() as u32,
			p_push_constant_ranges: push_constant_ranges.as_ptr() as *const _,
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_pipeline_layout(&infos, None)?
		};

		Ok(Raw {
			device: device.clone(),
			handle,
			pc: PhantomData
		})
	}

	pub fn handle(&self) -> vk::PipelineLayout {
		self.handle
	}
}

impl<C: PushConstants> Drop for Raw<C> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline_layout(self.handle, None)
		}
	}
}