use ash::vk;
use crate::{
	DeviceOwned,
	Format
};

mod usage;
mod layout;
pub mod view;

pub use usage::Usage;
pub use layout::Layout;
pub use view::View;

pub unsafe trait Image: DeviceOwned {
	fn handle(&self) -> vk::Image;

	fn into_view(
		self,
		ty: view::Type,
		format: Format,
		components: view::ComponentMapping,
		subresource_range: view::SubresourceRange
	) -> Result<View<Self>, view::CreationError> where Self: Sized {
		View::new(
			self,
			ty,
			format,
			components,
			subresource_range
		)
	}

	fn viewed(
		&self,
		ty: view::Type,
		format: Format,
		components: view::ComponentMapping,
		subresource_range: view::SubresourceRange
	) -> Result<View<&Self>, view::CreationError> {
		View::new(
			self,
			ty,
			format,
			components,
			subresource_range
		)
	}

	// requires #![feature(arbitrary_self_types)]
	// fn view<I>(
	// 	self: &I,
	// 	ty: view::Type,
	// 	format: Format,
	// 	components: view::ComponentMapping,
	// 	subresource_range: view::SubresourceRange
	// ) -> Result<View<I>, view::CreationError> where I: Deref<Target=Self> + Clone {
	// 	View::new(
	// 		self.clone(),
	// 		ty,
	// 		format,
	// 		components,
	// 		subresource_range
	// 	)
	// }
}

unsafe impl<'a, T: ?Sized + Image> Image for &'a T {
	fn handle(&self) -> vk::Image {
		(*self).handle()
	}
}