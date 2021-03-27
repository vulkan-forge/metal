use std::{
	sync::Arc,
	fmt
};
use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	Device,
	Format,
	DeviceOwned,
	sync::SharingQueues,
	OomError,
	mem::{
		Slot,
		MemoryRequirements
	}
};
use super::{
	Usage,
	Tiling,
	Type,
	SampleCount,
	Layout,
	Bound
};

#[derive(Debug)]
pub enum BindError {
	OutOfMemory(OomError),
	InvalidOpaqueCaptureAddress
}

impl fmt::Display for BindError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::OutOfMemory(e) => e.fmt(f),
			Self::InvalidOpaqueCaptureAddress => write!(f, "invalid opaque capture address")
		}
	}
}

impl std::error::Error for BindError {
	fn source(&self) -> Option<&(dyn 'static + std::error::Error)> {
		match self {
			Self::OutOfMemory(e) => Some(e),
			_ => None
		}
	}
}

impl From<vk::Result> for BindError {
	fn from(r: vk::Result) -> BindError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => BindError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => BindError::OutOfMemory(OomError::Device),
			vk::Result::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR => BindError::InvalidOpaqueCaptureAddress,
			_ => unreachable!()
		}
	}
}

pub struct Unbound {
	device: Arc<Device>,
	handle: vk::Image,
	size: (u32, u32, u32),
	usage: Usage,
	tiling: Tiling
}

impl Unbound {
	pub fn new<S: Into<SharingQueues>>(
		device: &Arc<Device>,
		ty: Type,
		format: Format,
		size: (u32, u32, u32),
		mip_levels: u32,
		array_layers: u32,
		samples: SampleCount,
		tiling: Tiling,
		usage: Usage,
		sharing_queues: S,
		initial_layout: Layout
	) -> Result<Self, OomError> {
		let sharing_queues = sharing_queues.into();
		let (sharing_mode, queue_family_index_count, p_queue_family_indices) = sharing_queues.as_vulkan();

		let flags = vk::ImageCreateFlags::empty();

		let infos = vk::ImageCreateInfo {
			flags,
			image_type: ty.into_vulkan(),
			format: format.into_vulkan(),
			extent: vk::Extent3D {
				width: size.0,
				height: size.1,
				depth: size.2
			},
			mip_levels,
			array_layers,
			samples: samples.into_vulkan(),
			tiling: tiling.into_vulkan(),
			usage: usage.to_vulkan(),
			sharing_mode,
			queue_family_index_count,
			p_queue_family_indices,
			initial_layout: initial_layout.into_vulkan(),
			..Default::default()
		};

		let handle = unsafe {
			device.handle().create_image(&infos, None)?
		};

		Ok(Self {
			device: device.clone(),
			handle,
			size,
			usage,
			tiling
		})
	}

	#[inline]
	pub(crate) fn handle(&self) -> vk::Image {
		self.handle
	}

	#[inline]
	pub fn size(&self) -> (u32, u32, u32) {
		self.size
	}

	#[inline]
	pub fn usage(&self) -> Usage {
		self.usage
	}

	#[inline]
	pub fn memory_requirements(&self) -> MemoryRequirements {
		unsafe {
			let mr = self.device.handle.get_image_memory_requirements(self.handle);
			MemoryRequirements::new(mr, self.tiling.is_linear())
		}
	}

	#[inline]
	pub unsafe fn bind<S: Slot>(self, slot: S) -> Result<Bound<S>, (Self, BindError)> {
		let memory = slot.memory();
		
		// We check for correctness in debug mode.
		debug_assert!({
			let mem_reqs = self.memory_requirements();
			mem_reqs.size() <= (memory.size() - slot.offset()) as u64
				&& (slot.offset() as u64 % mem_reqs.alignment()) == 0
				&& mem_reqs.contains_memory_type_index(memory.memory_type().index())
		});

		match self.device.handle.bind_image_memory(self.handle, memory.handle(), slot.offset()) {
			Ok(()) => (),
			Err(e) => return Err((self, e.into()))
		}

		Ok(Bound::new(self, slot))
	}
}

impl DeviceOwned for Unbound {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}
}
