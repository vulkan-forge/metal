pub struct CommandPool {
	device: Arc<Device>,
	handle: vk::CommandPool
}

impl CommandPool {
	/// Create a new command pool.
	///
	/// Command pools are not thread safe.
	/// It is recommended to create one command pool per frame per thread.
	pub fn new(device: &Arc<Device>) -> CommandPool {
		unsafe {
			self.handle = device.handle.create_command_pool(&infos, None).unwrap();
		}
	}
}

impl Drop for CommandPool {
	fn drop(&mut self) {
		unsafe {
			self.device.handle.destroy_command_pool(self.handle, None)
		}
	}
}

/// Command buffer.
pub struct CommandBuffer {
	// ...
}
