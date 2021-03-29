use ash::vk;

pub mod set;
pub mod pool;
pub mod update;

pub use set::{
	Set,
	Sets,
	SendSets
};
pub use pool::Pool;
pub use update::Update;

pub type RawImageInfo = vk::DescriptorImageInfo;
pub type RawBufferInfo = vk::DescriptorBufferInfo;

/// Descriptor.
pub unsafe trait Descriptor: Copy {
	/// Descriptor type.
	const TYPE: Type;

	/// The number of array elements in the descriptor (1 if it is not an array).
	const COUNT: u32;
}

pub enum Write {
	Image(RawImageInfo),
	Images(Vec<RawImageInfo>),
	Buffer(RawBufferInfo),
	Buffers(Vec<RawBufferInfo>),
	// TexelBufferView // TODO
}

/// Descriptor writer.
pub unsafe trait Writer<S: Set, D: Descriptor>: Sized where S::Layout: set::layout::HasDescriptor<D> {
	/// Prepares the necessary data to write to the descriptor.
	/// 
	/// The output gives the `Write` operation to perform
	/// which must match the descriptor type `TYPE`.
	fn prepare(&self) -> Write;

	unsafe fn apply(self, set: &mut S);

	/// Directly write the descriptor of the given set.
	fn write_to(self, set: &mut S, offset: u32) {
		unsafe {
			let mut update = Update::new();
			update.write(set, offset, &self); // this is safe because `update` is dropped just after this call.
			self.apply(set)
		}
	}
}

/// Descriptor type.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Type {
	Sampler = vk::DescriptorType::SAMPLER.as_raw(),
	CombinedImageSampler  = vk::DescriptorType::COMBINED_IMAGE_SAMPLER.as_raw(),
	SampledImage = vk::DescriptorType::SAMPLED_IMAGE.as_raw(),
	StorageImage = vk::DescriptorType::STORAGE_IMAGE.as_raw(),
	UniformTexelBuffer = vk::DescriptorType::UNIFORM_TEXEL_BUFFER.as_raw(),
	StorageTexelBuffer = vk::DescriptorType::STORAGE_TEXEL_BUFFER.as_raw(),
	UniformBuffer = vk::DescriptorType::UNIFORM_BUFFER.as_raw(),
	StorageBuffer = vk::DescriptorType::STORAGE_BUFFER.as_raw(),
	UniformBufferDynamic = vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC.as_raw(),
	StorageBufferDynamic = vk::DescriptorType::STORAGE_BUFFER_DYNAMIC.as_raw(),
	InputAttachment = vk::DescriptorType::INPUT_ATTACHMENT.as_raw()
}

impl Type {
	pub fn into_vulkan(self) -> vk::DescriptorType {
		vk::DescriptorType::from_raw(self as i32)
	}
}