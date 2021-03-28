use ash::vk;

pub mod set;
pub mod pool;
pub mod update;

pub use set::Set;
pub use pool::Pool;
pub use update::Update;

pub type RawImageInfo = vk::DescriptorImageInfo;
pub type RawBufferInfo = vk::DescriptorBufferInfo;

pub enum Write {
	Image(RawImageInfo),
	Images(Vec<RawImageInfo>),
	Buffer(RawBufferInfo),
	Buffers(Vec<RawBufferInfo>),
	// TexelBufferView // TODO
}

pub unsafe trait Descriptor: Copy {
	/// Descriptor type.
	const TYPE: Type;

	/// The number of array elements in the descriptor (1 if it is not an array).
	const COUNT: u32;
	
	/// Value used to update the descriptor.
	type Value;

	/// Prepares the necessary data to write to the descriptor.
	/// 
	/// The output gives the `Write` operation to perform
	/// which must match the descriptor type `TYPE`.
	fn write(value: Self::Value) -> Write;
}

/// Descriptor type
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