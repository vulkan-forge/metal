use ash::{
	vk,
	version::DeviceV1_0
};
use crate::{
	OomError,
	Format,
	resource
};
use super::{
	Image
};

#[derive(Debug)]
pub enum CreationError {
	OutOfMemory(OomError)
}

impl From<vk::Result> for CreationError {
	fn from(r: vk::Result) -> CreationError {
		match r {
			vk::Result::ERROR_OUT_OF_HOST_MEMORY => CreationError::OutOfMemory(OomError::Host),
			vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => CreationError::OutOfMemory(OomError::Device),
			_ => unreachable!()
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Type {
	D1 = vk::ImageViewType::TYPE_1D.as_raw(),
	D2 = vk::ImageViewType::TYPE_2D.as_raw(),
	D3 = vk::ImageViewType::TYPE_3D.as_raw(),
	Cube = vk::ImageViewType::CUBE.as_raw(),
	D1Array = vk::ImageViewType::TYPE_1D_ARRAY.as_raw(),
	D2Array = vk::ImageViewType::TYPE_2D_ARRAY.as_raw(),
	CubeArray = vk::ImageViewType::CUBE_ARRAY.as_raw()
}

impl Type {
	pub(crate) fn into_vulkan(self) -> vk::ImageViewType {
		vk::ImageViewType::from_raw(self as i32)
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ComponentSwizzle {
	Identity = vk::ComponentSwizzle::IDENTITY.as_raw(),
	Zero = vk::ComponentSwizzle::ZERO.as_raw(),
	One = vk::ComponentSwizzle::ONE.as_raw(),
	Red = vk::ComponentSwizzle::R.as_raw(),
	Green = vk::ComponentSwizzle::G.as_raw(),
	Blue = vk::ComponentSwizzle::B.as_raw(),
	Alpha = vk::ComponentSwizzle::A.as_raw()
}

impl ComponentSwizzle {
	pub(crate) fn into_vulkan(self) -> vk::ComponentSwizzle {
		vk::ComponentSwizzle::from_raw(self as i32)
	}
}

impl Default for ComponentSwizzle {
	fn default() -> Self {
		Self::Identity
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentMapping {
	red: ComponentSwizzle,
	green: ComponentSwizzle,
	blue: ComponentSwizzle,
	alpha: ComponentSwizzle
}

impl ComponentMapping {
	pub fn new(
		red: ComponentSwizzle,
		green: ComponentSwizzle,
		blue: ComponentSwizzle,
		alpha: ComponentSwizzle
	) -> Self {
		Self {
			red, green, blue, alpha
		}
	}

	pub(crate) fn into_vulkan(self) -> vk::ComponentMapping {
		vk::ComponentMapping {
			r: self.red.into_vulkan(),
			g: self.green.into_vulkan(),
			b: self.blue.into_vulkan(),
			a: self.alpha.into_vulkan()
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Aspects {
	color: bool,
	depth: bool,
	stencil: bool,
	metadata: bool
}

impl Aspects {
	pub fn new(
		color: bool,
		depth: bool,
		stencil: bool,
		metadata: bool
	) -> Aspects {
		Self {
			color, depth, stencil, metadata
		}
	}

	pub fn color() -> Self {
		Self::new(true, false, false, false)
	}

	pub fn depth() -> Self {
		Self::new(false, true, false, false)
	}

	pub fn depth_stencil() -> Self {
		Self::new(false, true, true, false)
	}

	pub(crate) fn into_vulkan(self) -> vk::ImageAspectFlags {
		let mut flags = vk::ImageAspectFlags::empty();

		if self.color {
			flags |= vk::ImageAspectFlags::COLOR
		}

		if self.depth {
			flags |= vk::ImageAspectFlags::DEPTH
		}

		if self.stencil {
			flags |= vk::ImageAspectFlags::STENCIL
		}

		if self.metadata {
			flags |= vk::ImageAspectFlags::METADATA
		}

		flags
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SubresourceRange {
	pub aspects: Aspects,
	pub base_mip_level: u32,
	pub level_count: u32,
	pub base_array_layer: u32,
	pub layer_count: u32
}

impl SubresourceRange {
	pub(crate) fn into_vulkan(self) -> vk::ImageSubresourceRange {
		vk::ImageSubresourceRange {
			aspect_mask: self.aspects.into_vulkan(),
			base_mip_level: self.base_mip_level,
			level_count: self.level_count,
			base_array_layer: self.base_array_layer,
			layer_count: self.layer_count
		}
	}
}

pub unsafe trait View: resource::Reference<Handle=vk::ImageView> {
	// ...
}

unsafe impl<V: std::ops::Deref> View for V where V::Target: View {
	// ...
}

pub struct Raw<I: Image> {
	image: I,
	handle: vk::ImageView
}

impl<I: Image> Raw<I> {
	pub fn new(
		image: I,
		ty: Type,
		format: Format,
		components: ComponentMapping,
		subresource_range: SubresourceRange
	) -> Result<Self, CreationError> {
		let infos = vk::ImageViewCreateInfo {
			image: image.handle(),
			view_type: ty.into_vulkan(),
			format: format.into_vulkan(),
			components: components.into_vulkan(),
			subresource_range: subresource_range.into_vulkan(),
			..Default::default()
		};

		let handle = unsafe {
			image.device().handle().create_image_view(&infos, None)?
		};

		Ok(Self {
			image,
			handle
		})
	}
}

unsafe impl<I: Image> resource::AbstractReference for Raw<I> {
	fn uid(&self) -> u64 {
		use ash::vk::Handle;
		self.handle.as_raw()
	}
}

unsafe impl<I: Image> resource::Reference for Raw<I> {
	type Handle = vk::ImageView;

	fn handle(&self) -> vk::ImageView {
		self.handle
	}
}

unsafe impl<I: Image> View for Raw<I> {
	//
}

impl<I: Image> Drop for Raw<I> {
	fn drop(&mut self) {
		unsafe {
			self.image.device().handle().destroy_image_view(self.handle, None)
		}
	}
}

pub struct LocalViews<'a> {
	handles: Vec<vk::ImageView>,
	resources: Vec<crate::resource::Ref<'a>>
}

impl<'a> LocalViews<'a> {
	pub fn new() -> Self {
		Self {
			handles: Vec::new(),
			resources: Vec::new()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.handles.is_empty()
	}

	pub fn len(&self) -> usize {
		self.handles.len()
	}

	pub fn push<V: 'a + View>(&mut self, view: V) {
		self.handles.push(view.handle());
		self.resources.push(view.into());
	}

	pub(crate) fn as_vulkan(&self) -> &[vk::ImageView] {
		&self.handles
	}
}

impl<'a> AsRef<[vk::ImageView]> for LocalViews<'a> {
	fn as_ref(&self) -> &[vk::ImageView] {
		self.as_vulkan()
	}
}

impl<'a> IntoIterator for LocalViews<'a> {
	type Item = crate::resource::Ref<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::Ref<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}

pub struct Views<'a> {
	handles: Vec<vk::ImageView>,
	resources: Vec<crate::resource::SendRef<'a>>
}

impl<'a> Views<'a> {
	pub fn new() -> Self {
		Self {
			handles: Vec::new(),
			resources: Vec::new()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.handles.is_empty()
	}

	pub fn len(&self) -> usize {
		self.handles.len()
	}

	pub fn push<V: 'a + Send + View>(&mut self, view: V) {
		self.handles.push(view.handle());
		self.resources.push(view.into());
	}

	pub(crate) fn as_vulkan(&self) -> &[vk::ImageView] {
		&self.handles
	}
}

impl<'a> IntoIterator for Views<'a> {
	type Item = crate::resource::SendRef<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::SendRef<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}

impl<'a> AsRef<[vk::ImageView]> for Views<'a> {
	fn as_ref(&self) -> &[vk::ImageView] {
		self.as_vulkan()
	}
}

pub struct SyncViews<'a> {
	handles: Vec<vk::ImageView>,
	resources: Vec<crate::resource::SyncRef<'a>>
}

impl<'a> SyncViews<'a> {
	pub fn new() -> Self {
		Self {
			handles: Vec::new(),
			resources: Vec::new()
		}
	}

	pub fn is_empty(&self) -> bool {
		self.handles.is_empty()
	}

	pub fn len(&self) -> usize {
		self.handles.len()
	}

	pub fn push<V: 'a + Send + Sync + View>(&mut self, view: V) {
		self.handles.push(view.handle());
		self.resources.push(view.into());
	}

	pub(crate) fn as_vulkan(&self) -> &[vk::ImageView] {
		&self.handles
	}
}

impl<'a> IntoIterator for SyncViews<'a> {
	type Item = crate::resource::SyncRef<'a>;
	type IntoIter = std::vec::IntoIter<crate::resource::SyncRef<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.resources.into_iter()
	}
}

impl<'a> AsRef<[vk::ImageView]> for SyncViews<'a> {
	fn as_ref(&self) -> &[vk::ImageView] {
		self.as_vulkan()
	}
}