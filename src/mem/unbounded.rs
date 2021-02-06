pub enum Strategy {
	Linear
}

/// Allocator without logical memory limit.
///
/// The allocator is still limited by the actual physical memory size.
pub struct Unbounded {
	// ...
}
