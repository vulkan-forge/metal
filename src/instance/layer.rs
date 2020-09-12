use std::ffi::{
	CStr
};
use ash::vk;
use crate::Entry;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ValidationLayer {
	KhronosValidation
}

impl ValidationLayer {
	pub fn c_name(&self) -> &'static CStr {
		use ValidationLayer::*;

		unsafe {
			let name = match self {
				KhronosValidation => b"VK_LAYER_KHRONOS_validation\0"
			};

			CStr::from_bytes_with_nul_unchecked(name)
		}
	}
}

pub struct InstanceValidationLayer<'a> {
	entry: &'a Entry,
	props: vk::LayerProperties
}

impl<'a> InstanceValidationLayer<'a> {
	pub(crate) fn new(entry: &'a Entry, props: vk::LayerProperties) -> InstanceValidationLayer<'a> {
		InstanceValidationLayer {
			entry,
			props
		}
	}

	#[inline]
	pub fn entry(&self) -> &'a Entry {
		self.entry
	}

	#[inline]
	pub fn name(&self) -> &str {
		unsafe {
			let c_name = CStr::from_ptr(self.props.layer_name.as_ptr());
			c_name.to_str().expect("validation layer name is not UTF-8 encoded")
		}
	}
}
