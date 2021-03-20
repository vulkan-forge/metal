use std::{
	fmt,
	ffi::CStr
};
use crate::Unbuildable;

validation_layers! {
	khronos_validation: KhronosValidation => b"VK_LAYER_KHRONOS_validation\0",
}

// pub struct InstanceValidationLayer<'a> {
// 	entry: &'a Entry,
// 	props: vk::LayerProperties
// }

// impl<'a> InstanceValidationLayer<'a> {
// 	pub(crate) fn new(entry: &'a Entry, props: vk::LayerProperties) -> InstanceValidationLayer<'a> {
// 		InstanceValidationLayer {
// 			entry,
// 			props
// 		}
// 	}

// 	#[inline]
// 	pub fn entry(&self) -> &'a Entry {
// 		self.entry
// 	}

// 	#[inline]
// 	pub fn name(&self) -> &str {
// 		unsafe {
// 			let c_name = CStr::from_ptr(self.props.layer_name.as_ptr());
// 			c_name.to_str().expect("validation layer name is not UTF-8 encoded")
// 		}
// 	}
// }
