use ash::vk;

pub mod ty;
pub mod set;
pub mod pool;
// pub mod update;

pub use ty::{Type, DataType, WellTyped};

pub use set::{
	Set,
	Sets
};
pub use pool::Pool;
// pub use update::{
// 	Update,
// 	UpdateSet
// };

pub type RawImageInfo = vk::DescriptorImageInfo;
pub type RawBufferInfo = vk::DescriptorBufferInfo;

// /// Descriptor.
// pub unsafe trait Descriptor: Copy {
// 	/// Descriptor type.
// 	const TYPE: Type;

// 	/// The number of array elements in the descriptor (1 if it is not an array).
// 	const COUNT: u32;
// }

pub enum WriteInfo {
	Image(RawImageInfo),
	Images(Vec<RawImageInfo>),
	Buffer(RawBufferInfo),
	Buffers(Vec<RawBufferInfo>),
	// TexelBufferView // TODO
}

// /// Descriptor write.
// pub unsafe trait Write<D: Descriptor, T>: Set where Self::Layout: set::layout::HasDescriptor<D> {
// 	/// Prepares the necessary data to write to the descriptor.
// 	/// 
// 	/// The output gives the `Write` operation to perform
// 	/// which must match the descriptor type `TYPE`.
// 	fn prepare(value: &T) -> WriteInfo;

// 	/// Assign the given value to the descriptor.
// 	/// 
// 	/// This function is automatically and safely called from the `write` function.
// 	/// 
// 	/// ## Safety
// 	/// 
// 	/// This does not actually write to the descriptor,
// 	/// but only store the assigned value so it is not dropped before the set.
// 	/// However this will release the currently/previoulsy assigned value,
// 	/// which may lead to undefined behavior.
// 	unsafe fn set(&mut self, value: T);
// }