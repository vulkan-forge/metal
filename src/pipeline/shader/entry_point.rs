use std::{
	ffi::{
		CStr,
		CString
	},
	sync::Arc
};
use super::Module;

pub struct EntryPoint {
	module: Arc<Module>,
	name: CString
}

impl EntryPoint {
	pub(crate) unsafe fn new(module: &Arc<Module>, name: CString) -> EntryPoint {
		EntryPoint {
			module: module.clone(),
			name
		}
	}

	pub fn module(&self) -> &Arc<Module> {
		&self.module
	}

	pub fn name(&self) -> &CStr {
		&self.name
	}
}