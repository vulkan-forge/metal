pub trait Readable {
	type Read;

	fn read(&self) -> Self::Read;
}

pub struct Ref<R> {
	reader: R
}

/// GPU ref cell.
pub struct RefCell<T> {
	value: T
}

impl<T> RefCell<T> {
	pub fn borrow(&self) -> Ref<T::Read> {
		// ...
	}
}