use ash::{
	vk,
	version::DeviceV1_0
};
use std::sync::Arc;
use crate::{
	OomError,
	device::Device
};
use super::EntryPoint;

#[derive(Debug)]
pub enum CreationError {
	OomError(OomError),
}

impl From<vk::Result> for CreationError {
	fn from(e: vk::Result) -> Self {
		match e {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OomError(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OomError(OomError::Device),
			_ => unreachable!()
		}
	}
}

pub struct Module {
	device: Arc<Device>,
	handle: vk::ShaderModule
}

impl Module {
	pub unsafe fn new<B: AsRef<[u8]>>(device: &Arc<Device>, vspir: B) -> Result<Module, CreationError> {
		let bytes = vspir.as_ref();

		let infos = vk::ShaderModuleCreateInfo {
			code_size: bytes.len(),
			p_code: bytes.as_ptr() as *const u32,
			..Default::default()
		};

		let handle = device.handle().create_shader_module(&infos, None)?;
	
		Ok(Module {
			device: device.clone(),
			handle
		})
	}

	pub unsafe fn entry_point(self: &Arc<Self>, name: &str) -> EntryPoint {
		EntryPoint::new(self, std::ffi::CString::new(name).expect("invalid shader module entry point name"))
	}

	pub(crate) fn handle(&self) -> vk::ShaderModule {
		self.handle
	}
}

impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			self.device.handle().destroy_shader_module(self.handle, None)
		}
	}
}