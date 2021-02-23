use ash::vk;

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct ClearValue(vk::ClearValue);

impl ClearValue {
	pub fn f32color(red: f32, green: f32, blue: f32, alpha: f32) -> ClearValue {
		ClearValue(vk::ClearValue {
			color: vk::ClearColorValue {
				float32: [red, green, blue, alpha]
			}
		})
	}
}