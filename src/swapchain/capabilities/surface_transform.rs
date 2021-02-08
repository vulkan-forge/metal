use ash::vk;

flag_set! {
	SurfaceTransform {
		identity (Identity): vk::SurfaceTransformFlagsKHR::IDENTITY,
		rotate90 (Rotate90): vk::SurfaceTransformFlagsKHR::ROTATE_90,
		rotate180 (Rotate180): vk::SurfaceTransformFlagsKHR::ROTATE_180,
		rotate270 (Rotate270): vk::SurfaceTransformFlagsKHR::ROTATE_270,
		horizontal_mirror (HorizontalMirror): vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR,
		horizontal_mirror_rotate90 (HorizontalMirrirRotate90): vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90,
		horizontal_mirror_rotate180 (HorizontalMirrirRotate180): vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180,
		horizontal_mirror_rotate270 (HorizontalMirrirRotate270): vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270,
		inherit (Inherit): vk::SurfaceTransformFlagsKHR::INHERIT
	}

	SurfaceTransforms: flags vk::SurfaceTransformFlagsKHR [u32]
}