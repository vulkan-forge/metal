use std::marker::PhantomData;
use ash::vk;

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
	pub const fn into_vulkan(self) -> vk::DescriptorType {
		vk::DescriptorType::from_raw(self as i32)
	}
}

/// Type that represent a descriptor type and count.
pub trait SizedType {
	const TYPE: Type;
	const COUNT: u32;
}

pub trait Array {
	const COUNT: u32;
}

impl<T> Array for T {
	default const COUNT: u32 = 1u32;
}

impl<T, const N: usize> Array for [T; N] {
	const COUNT: u32 = {N as u32};
}

pub struct UniformBuffer<T>(PhantomData<T>);

impl<T: Array> SizedType for UniformBuffer<T> {
	const TYPE: Type = Type::UniformBuffer;
	const COUNT: u32 = T::COUNT;
}