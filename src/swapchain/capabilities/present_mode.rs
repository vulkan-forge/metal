use ash::vk;

flag_set! {
	/// The way presenting a swapchain is accomplished.
	PresentMode {
		/// Immediately shows the image to the user. May result in visible tearing.
		immediate (Immediate): vk::PresentModeKHR::IMMEDIATE,

		/// The action of presenting an image puts it in wait. When the next vertical blanking period
		/// happens, the waiting image is effectively shown to the user. If an image is presented while
		/// another one is waiting, it is replaced.
		mailbox (Mailbox): vk::PresentModeKHR::MAILBOX,

		/// The action of presenting an image adds it to a queue of images. At each vertical blanking
		/// period, the queue is popped and an image is presented.
		///
		/// Guaranteed to be always supported.
		///
		/// This is the equivalent of OpenGL's `SwapInterval` with a value of 1.
		fifo (Fifo): vk::PresentModeKHR::FIFO,

		/// Same as `Fifo`, except that if the queue was empty during the previous vertical blanking
		/// period then it is equivalent to `Immediate`.
		///
		/// This is the equivalent of OpenGL's `SwapInterval` with a value of -1.
		relaxed (Relaxed): vk::PresentModeKHR::FIFO_RELAXED
	}

	/// List of `PresentMode`s that are supported.
	PresentModes: vec vk::PresentModeKHR [i32]
}

// TODO: These can't be enabled yet because they have to be used with shared present surfaces
// which vulkano doesnt support yet.
//SharedDemand: vk::PresentModeKHR::SHARED_DEMAND_REFRESH,
//SharedContinuous: vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH,