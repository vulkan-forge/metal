pub use std::{
	sync::Arc,
	hash::{
		Hash,
		Hasher
	}
};
use ash::vk::Handle;

mod ref_local;
mod ref_send;
mod ref_sync;
mod proxy;

pub use ref_local::Ref;
pub use ref_send::SendRef;
pub use ref_sync::SyncRef;
pub use proxy::Proxy;

/// GPU resource.
pub unsafe trait AbstractResource {
	/// Unique identifier of the resource.
	/// 
	/// ## Safety
	/// 
	/// This must be unique across a given device.
	fn uid(&self) -> u64;
}

pub unsafe trait Resource {
	type Handle: Copy + Handle;

	fn handle(&self) -> Self::Handle;
}

unsafe impl<B: std::ops::Deref> Resource for B where B::Target: Resource {
	type Handle = <B::Target as Resource>::Handle;

	fn handle(&self) -> Self::Handle {
		self.deref().handle()
	}
}

unsafe impl<R: Resource> AbstractResource for R {
	fn uid(&self) -> u64 {
		self.handle().as_raw()
	}
}