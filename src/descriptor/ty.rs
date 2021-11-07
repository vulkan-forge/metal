use std::marker::PhantomData;
use ash::vk;
use crate::pipeline::shader;

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
pub trait DataType {
	const DESCRIPTOR_TYPE: Type;
	const COUNT: u32;
}

/// Type that represent a descriptor type, count and accessing stages.
pub trait AccessedDataType {
	const DESCRIPTOR_TYPE: Type;
	const COUNT: u32;
	const STAGES: shader::Stages;
}

/// Marker for descriptor type.
pub unsafe trait WellTyped<const TYPE: Type, const COUNT: u32, const STAGES: crate::pipeline::shader::Stages> {}

unsafe impl<T: AccessedDataType> WellTyped<{T::DESCRIPTOR_TYPE}, {T::COUNT}, {T::STAGES}> for T {}

pub trait Array {
	const COUNT: u32;
}

impl<T> Array for T {
	default const COUNT: u32 = 1u32;
}

impl<T, const N: usize> Array for [T; N] {
	const COUNT: u32 = {N as u32};
}

/// Array type of length `L`.
/// 
/// This type is unsafe because it must be
/// implemented at most once,
/// `L` must be equal to `Array::LEN`.
pub unsafe trait ArrayLen<const L: u32>: Array {}

unsafe impl<T: Array> ArrayLen<{T::COUNT}> for T {}

pub struct UniformBuffer<T>(PhantomData<T>);

impl<T: Array> DataType for UniformBuffer<T> {
	const DESCRIPTOR_TYPE: Type = Type::UniformBuffer;
	const COUNT: u32 = T::COUNT;
}

pub struct Accessed<T, const STAGES: shader::Stages>(PhantomData<T>);

impl<T: DataType, const STAGES: shader::Stages> AccessedDataType for Accessed<T, STAGES> {
	const DESCRIPTOR_TYPE: Type = T::DESCRIPTOR_TYPE;
	const COUNT: u32 = T::COUNT;
	const STAGES: shader::Stages = STAGES;
}