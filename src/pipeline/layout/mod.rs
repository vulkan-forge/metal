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
	descriptor,
	resource
};

pub mod push_constant;

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

pub type Handle = vk::PipelineLayout;

pub unsafe trait UntypedLayout: resource::Reference<Handle=Handle> {
	type PushConstants: PushConstants;
	type DescriptorSets: descriptor::set::Layouts;
}

unsafe impl<L: std::ops::Deref> UntypedLayout for L where L::Target: UntypedLayout {
	type PushConstants = <L::Target as UntypedLayout>::PushConstants;
	type DescriptorSets = <L::Target as UntypedLayout>::DescriptorSets;
}

/// Layout without descriptor sets.
pub type NoSets<P> = Raw<P, ()>;

impl<P: PushConstants> NoSets<P> {
	pub fn from_device(device: &Arc<Device>) -> Result<Self, CreationError> {
		Raw::new(device, Arc::new(()))
	}
}

/// Empty layout.
pub type Empty = NoSets<()>;

pub struct Raw<C: PushConstants, S: descriptor::set::Layouts> {
	device: Arc<Device>,
	handle: vk::PipelineLayout,
	pc: PhantomData<C>,
	sets: Arc<S>
}

impl<C: PushConstants, S: descriptor::set::Layouts> Raw<C, S> {
	pub fn new(device: &Arc<Device>, set_layouts: Arc<S>) -> Result<Raw<C, S>, CreationError> {
		let handle = {
			let vk_set_layouts = set_layouts.handles();
			let vk_set_layouts = vk_set_layouts.as_ref();
			
			let push_constant_ranges = C::RANGES;

			let infos = vk::PipelineLayoutCreateInfo {
				flags: vk::PipelineLayoutCreateFlags::empty(),
				set_layout_count: vk_set_layouts.len() as u32,
				p_set_layouts: vk_set_layouts.as_ptr(),
				push_constant_range_count: push_constant_ranges.len() as u32,
				p_push_constant_ranges: push_constant_ranges.as_ptr() as *const _,
				..Default::default()
			};

			unsafe {
				device.handle().create_pipeline_layout(&infos, None)?
			}
		};

		Ok(Raw {
			device: device.clone(),
			handle,
			pc: PhantomData,
			sets: set_layouts
		})
	}

	pub fn handle(&self) -> vk::PipelineLayout {
		self.handle
	}

	pub fn set_layouts(&self) -> &Arc<S> {
		&self.sets
	}
}

unsafe impl<P: PushConstants, S: descriptor::set::Layouts> resource::Reference for Raw<P, S> {
	type Handle = Handle;

	fn handle(&self) -> Handle {
		self.handle()
	}
}

unsafe impl<P: PushConstants, S: descriptor::set::Layouts> UntypedLayout for Raw<P, S> {
	type PushConstants = P;
	type DescriptorSets = S;
}

impl<C: PushConstants, S: descriptor::set::Layouts> Drop for Raw<C, S> {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_pipeline_layout(self.handle, None)
		}
	}
}

/// Creates a new untyped pipeline layout type.
/// 
/// The created type will be a newtype wrapping a [`Raw`] pipeline layout and
/// implementing the [`UntypedLayout`] trait.
/// 
/// ## Example
/// 
/// ```
/// untyped_pipeline_layout! {
/// 	/// My pipeline layout.
/// 	pub struct MyLayout {
/// 		// ...
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! untyped_pipeline_layout {
	{
		$(#[$doc:meta])*
		$vis:vis struct $id:ident {
			type PushConstants = $push_constants:ty;
			type DescriptorSets = $descriptor_sets:ty;
		}
	} => {
		$(#[$doc])*
		$vis type $id = $crate::pipeline::layout::Raw<
			$push_constants,
			$descriptor_sets
		>;
	};
}

/// Well typed layout.
/// 
/// A layout a well typed with regards to the untyped layout U
/// if `Self::PushConstants = U::PushConstants` and
/// `Self::DescriptorSets: descriptor::set::layout::WellTyped<U::DescriptorSets>`.
pub unsafe trait WellTyped<U> {}

/// Typed pipeline layout.
pub unsafe trait Layout: WellTyped<<Self as Layout>::Untyped> {
	/// Untyped layout.
	type Untyped: UntypedLayout;

	type PushConstants;
	type DescriptorSets;
}

/// Creates a new typed pipeline layout.
#[macro_export]
macro_rules! pipeline_layout {
	{
		$(#[$doc:meta])*
		$vis:vis struct $id:ident : $untyped_layout:ty {
			type PushConstants = $push_constants:ty;
			type DescriptorSets = $descriptor_sets:ty;
		}
	} => {
		$(#[$doc])*
		$vis struct $id;

		unsafe impl $crate::pipeline::Layout for $id {
			type Untyped = $untyped_layout;

			type PushConstants = $push_constants;
			type DescriptorSets = $descriptor_sets;
		}

		unsafe impl<U: $crate::pipeline::UntypedLayout<PushConstants = $push_constants>> $crate::pipeline::layout::WellTyped<U> for $id
		where
			$descriptor_sets: $crate::descriptor::set::layout::WellTyped<U::DescriptorSets>
		{}
	};
}