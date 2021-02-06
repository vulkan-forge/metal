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
		Surface
	},
	win::{
		self,
		WindowBuilderExt
	}
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

	/// Create logical device (and queues).
	let queue_family = get_queue_family(&physical_device, &surface);
	let (device, mut queues) = get_device(&physical_device, queue_family);
	let queue = queues.next().unwrap();

	// let surface_capabilities = surface.capabilities(device.physical_device()).unwrap();
	// let (color_format, color_space) = choose_format(&surface_capabilities).expect("No appropriate format found");

	// ...

	event_loop.run(move |event, _, _| {
		// TODO
	});
}
