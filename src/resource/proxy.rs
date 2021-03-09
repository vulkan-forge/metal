use std::sync::Arc;
use super::Resource;

/// A safe `Send` wrapper over `Arc<R>` resources.
/// 
/// As mentioned in the documentation of `Arc`,
/// `Arc<R>` makes it thread safe to have multiple ownership of the same data,
/// but it doesn't add thread safety to its data.
/// As a consequence, `Arc<R>` can only be `Send` when `R` is `Send + Sync`,
/// which can become a problem in our setting.
/// 
/// This `Proxy` type overcomes this issue by guaranteeing that the data
/// behind the `Arc` will never be read through this reference,
/// making it `Send` even when `R` is not `Sync`.
/// Moreover, by copying the handle to the underlying resource,
/// `Proxy<R>` acts as a proxy for the resource by implementing `Resource`,
/// providing the handle of the undrlying resource when asked.
/// 
/// Note that `R` still needs to be `Send` since `Drop::drop`
/// may still be called from another thread.
pub struct Proxy<R: ?Sized + Resource> {
	arc: Arc<R>,
	handle: R::Handle
}

unsafe impl<R: Send + ?Sized + Resource> Send for Proxy<R> {}

impl<R: ?Sized + Resource> Proxy<R> {
	/// Creates a new proxy for the given resource.
	pub fn new(arc: &Arc<R>) -> Proxy<R> {
		Proxy {
			arc: arc.clone(),
			handle: arc.handle()
		}
	}

	/// Returns the underlying `Arc<R>`.
	pub fn unwrap(self) -> Arc<R> where R: Sync + Send {
		self.arc
	}
}

unsafe impl<R: ?Sized + Resource> Resource for Proxy<R> {
	type Handle = R::Handle;

	fn handle(&self) -> R::Handle {
		self.handle
	}
}