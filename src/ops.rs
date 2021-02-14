use ash::vk;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Compare {
	Never = vk::CompareOp::NEVER.as_raw(),
	Less = vk::CompareOp::LESS.as_raw(),
	Equal = vk::CompareOp::EQUAL.as_raw(),
	LessOrEqual = vk::CompareOp::LESS_OR_EQUAL.as_raw(),
	Greater = vk::CompareOp::GREATER.as_raw(),
	NotEqual = vk::CompareOp::NOT_EQUAL.as_raw(),
	GreaterOrEqual = vk::CompareOp::GREATER_OR_EQUAL.as_raw(),
	Always = vk::CompareOp::ALWAYS.as_raw()
}

impl Compare {
	pub(crate) fn into_vulkan(self) -> vk::CompareOp {
		vk::CompareOp::from_raw(self as i32)
	}
}

impl Default for Compare {
	#[inline]
	fn default() -> Self {
		unsafe {
			std::mem::transmute(vk::CompareOp::default().as_raw())
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Logic {
	Clear = vk::LogicOp::CLEAR.as_raw(),
	And = vk::LogicOp::AND.as_raw(),
	AndReverse = vk::LogicOp::AND_REVERSE.as_raw(),
	Copy = vk::LogicOp::COPY.as_raw(),
	AndInverted = vk::LogicOp::AND_INVERTED.as_raw(),
	NoOp = vk::LogicOp::NO_OP.as_raw(),
	Xor = vk::LogicOp::XOR.as_raw(),
	Or = vk::LogicOp::OR.as_raw(),
	Nor = vk::LogicOp::NOR.as_raw(),
	Equivalent = vk::LogicOp::EQUIVALENT.as_raw(),
	Invert = vk::LogicOp::INVERT.as_raw(),
	OrReverse = vk::LogicOp::OR_REVERSE.as_raw(),
	CopyInverted = vk::LogicOp::COPY_INVERTED.as_raw(),
	OrInverted = vk::LogicOp::OR_INVERTED.as_raw(),
	Nand = vk::LogicOp::NAND.as_raw(),
	Set = vk::LogicOp::SET.as_raw()
}

impl Logic {
	pub(crate) fn into_vulkan(self) -> vk::LogicOp {
		vk::LogicOp::from_raw(self as i32)
	}
}

impl Default for Logic {
	#[inline]
	fn default() -> Self {
		unsafe {
			std::mem::transmute(vk::LogicOp::default().as_raw())
		}
	}
}