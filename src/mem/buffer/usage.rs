use ash::vk;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Usage {
	TransferSource = vk::BufferUsageFlags::TRANSFER_SRC.as_raw(),
	TransferDestination = vk::BufferUsageFlags::TRANSFER_DST.as_raw(),
	UniformTexelBuffer = vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER.as_raw(),
	StorageTexelBuffer = vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER.as_raw(),
	UniformBuffer = vk::BufferUsageFlags::UNIFORM_BUFFER.as_raw(),
	StorageBuffer = vk::BufferUsageFlags::STORAGE_BUFFER.as_raw(),
	IndexBuffer = vk::BufferUsageFlags::INDEX_BUFFER.as_raw(),
	VertexBuffer = vk::BufferUsageFlags::VERTEX_BUFFER.as_raw(),
	IndirectBuffer = vk::BufferUsageFlags::INDIRECT_BUFFER.as_raw()
}

impl Usage {
	pub(crate) fn into_vulkan(self) -> vk::BufferUsageFlags {
		vk::BufferUsageFlags::from_raw(self as u32)
	}
}

impl std::ops::BitOr for Usage {
	type Output = Usages;

	fn bitor(self, rhs: Self) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOr<Usages> for Usage {
	type Output = Usages;

	fn bitor(self, rhs: Usages) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

#[derive(Clone, Copy)]
pub struct Usages(vk::BufferUsageFlags);

impl Usages {
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn into_vulkan(self) -> vk::BufferUsageFlags {
		self.0
	}

	pub fn transfer_source(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::TRANSFER_SRC)
	}

	pub fn transfer_destination(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::TRANSFER_DST)
	}

	pub fn uniform_texel_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER)
	}

	pub fn storage_texel_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER)
	}

	pub fn uniform_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::UNIFORM_BUFFER)
	}

	pub fn storage_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::STORAGE_BUFFER)
	}

	pub fn index_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::INDEX_BUFFER)
	}

	pub fn vertex_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::VERTEX_BUFFER)
	}

	pub fn indirect_buffer(&self) -> bool {
		self.0.contains(vk::BufferUsageFlags::INDIRECT_BUFFER)
	}
}

impl From<Usage> for Usages {
	fn from(u: Usage) -> Usages {
		Usages(u.into_vulkan())
	}
}

impl std::ops::BitOr for Usages {
	type Output = Usages;

	fn bitor(self, rhs: Self) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOr<Usage> for Usages {
	type Output = Usages;

	fn bitor(self, rhs: Usage) -> Usages {
		Usages(self.into_vulkan() | rhs.into_vulkan())
	}
}

impl std::ops::BitOrAssign for Usages {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.into_vulkan()
	}
}

impl std::ops::BitOrAssign<Usage> for Usages {
	fn bitor_assign(&mut self, rhs: Usage) {
		self.0 |= rhs.into_vulkan()
	}
}