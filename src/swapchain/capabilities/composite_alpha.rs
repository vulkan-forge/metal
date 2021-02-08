use ash::vk;

flag_set! {
	/// How the alpha values of the pixels of the window are treated.
	CompositeAlpha {
		/// The alpha channel of the image is ignored.
		/// All the pixels are considered as if they have a
		/// value of 1.0.
		opaque (Opaque): vk::CompositeAlphaFlagsKHR::OPAQUE,

		/// The alpha channel of the image is respected.
		/// The color channels are expected to have
		/// already been multiplied by the alpha value.
		pre_multiplied (PreMultiplied): vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
		
		/// The alpha channel of the image is respected.
		/// The color channels will be multiplied by the
		/// alpha value by the compositor before being added to what is behind.
		post_multiplied (PostMultiplied): vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,

		/// Let the operating system or driver implementation choose.
		inherit (Inherit): vk::CompositeAlphaFlagsKHR::INHERIT
	}

	/// List of supported composite alpha modes.
	///
	/// See the docs of `CompositeAlpha`.
	CompositeAlphas: flags vk::CompositeAlphaFlagsKHR [u32]
}