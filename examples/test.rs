use std::sync::Arc;
use log::error;
use magma::{
	Entry,
	Instance,
	instance::{
		PhysicalDevice,
		physical_device::QueueFamily
	},
	Device,
	device::{
		self,
		Queues,
		Queue
	},
	swapchain::{
		Surface,
		Capabilities,
		capabilities::ColorSpace
	},
	win::{
		self,
		WindowBuilderExt
	},
	Format
};
use winit::{
	event_loop::EventLoop,
	event::{
		Event,
		WindowEvent,
		MouseButton,
		ElementState
	},
	window::{
		Window as WinitWindow,
		WindowBuilder
	}
};

fn get_queue_family<'a>(physical_device: &'a PhysicalDevice, surface: &Surface<WinitWindow>) -> QueueFamily<'a> {
	// TODO we may want one queue for graphics, and another one for presentation.
	physical_device.queue_families().find(|&queue| {
		queue.supports_graphics() && surface.is_supported(queue).unwrap_or(false)
	}).unwrap()
}

fn get_device<'a>(physical_device: &'a PhysicalDevice, queue_family: QueueFamily<'a>) -> (Arc<Device>, device::Queues) {
	// TODO check that this extension is supported?
	let device_ext = device::Extensions {
		khr_swapchain: true,
		..device::Extensions::none()
	};

	Device::new(
		physical_device.clone(),
		physical_device.supported_features(), // enabled features (all of them?)
		&device_ext,
		[(queue_family, 1.0)].iter().cloned()
	).unwrap()
}

// Choose a surface format and color space.
fn choose_format(surface_capabilities: &Capabilities) -> Option<(Format, ColorSpace)> {
	for (format, color_space) in &surface_capabilities.supported_formats {
		if *format == Format::B8G8R8A8Srgb && *color_space == ColorSpace::SrgbNonLinear {
			return Some((*format, *color_space))
		}
	}

	None
}

pub fn main() {
	stderrlog::new().verbosity(3).init().unwrap();

	let entry = Arc::new(Entry::new().expect("Unable to load vulkan"));

	let required_extensions = win::required_extensions(&entry);

	for ext in required_extensions {
		println!("extension: {}", ext);
	}

	let instance = match Instance::new(entry, required_extensions) {
		Ok(i) => Arc::new(i),
		Err(e) => {
			error!("Could not build instance: {:?}", e);
			std::process::exit(1);
		}
	};

	let physical_device = Arc::new(instance.physical_devices().next().unwrap());
	println!("device: {}", physical_device.name());

	let event_loop = EventLoop::new();
	let surface = WindowBuilder::new().build_vk_surface(&event_loop, &instance).unwrap();

	// Create logical device (and queues).
	let queue_family = get_queue_family(&physical_device, &surface);
	let (device, mut queues) = get_device(&physical_device, queue_family);
	let queue = queues.next().unwrap();

	let surface_capabilities = surface.capabilities(device.physical_device()).unwrap();
	let (color_format, color_space) = choose_format(&surface_capabilities).expect("No appropriate format found");

	let (swapchain, images) = Swapchain::new(
		device.clone(),
		surface.clone(),
		surface_capabilities.min_image_count,
		format,
		dimensions, // TODO check if the dimensions are supported by the swapchain.
		1,
		ImageUsage::color_attachment(),
		&queue,
		surface_capabilities.current_transform,
		CompositeAlpha::Opaque, // ignore alpha component.
		PresentMode::Fifo, // guaranteed to exist.
		FullscreenExclusive::Default,
		true,
		color_space
	).unwrap();

	event_loop.run(move |event, _, _| {
		// TODO
	});
}
