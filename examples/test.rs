use std::{
	sync::Arc,
	path::Path,
	io::Read,
	convert::TryInto
};
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
	Swapchain,
	swapchain::{
		Surface,
		capabilities::{
			CompositeAlpha,
			PresentMode
		},
		Capabilities,
		capabilities::ColorSpace
	},
	image,
	pipeline::{
		self,
		shader,
		Viewport,
		Scissor,
		ColorBlend,
		color_blend::{
			self,
			BlendFactor
		},
		Layout
	},
	win::{
		self,
		WindowBuilderExt
	},
	framebuffer,
	Format,
	ops
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
	let surface = Arc::new(WindowBuilder::new().build_vk_surface(&event_loop, &instance).unwrap());
	let dimensions: (u32, u32) = surface.backend().inner_size().into();

	// Create logical device (and queues).
	let queue_family = get_queue_family(&physical_device, &surface);
	let (device, mut queues) = get_device(&physical_device, queue_family);
	let queue = queues.next().unwrap();

	let surface_capabilities = surface.capabilities(device.physical_device()).unwrap();
	let (color_format, color_space) = choose_format(&surface_capabilities).expect("No appropriate format found");

	let swapchain = Swapchain::new(
		&device,
		&surface,
		surface_capabilities.min_image_count,
		color_format,
		color_space,
		Some(dimensions), // TODO check if the dimensions are supported by the swapchain.
		1,
		image::Usage::color_attachment(),
		Some(&queue),
		surface_capabilities.current_transform,
		CompositeAlpha::Opaque, // ignore alpha component.
		PresentMode::Fifo, // guaranteed to exist.
		true,
		None
	).unwrap();

	// Load the shader modules.
	let vertex_shader = unsafe { load_shader_module(&device, "examples/shaders/triangle.vert.spv") };
	let fragment_shader = unsafe { load_shader_module(&device, "examples/shaders/triangle.frag.spv") };
	let stages = unsafe {
		pipeline::stage::Vertex::new(
			vertex_shader.entry_point("main"),
			pipeline::stage::Fragment::new(
				fragment_shader.entry_point("main")
			)
		)
	};

	let render_pass = create_render_pass(&device, swapchain.format());

	let layout = Arc::new(Layout::new(&device, &[], &[]).expect("unable to create pipeline layout"));

	let pipeline = pipeline::Graphics::<_, (), 1>::new(
		&device,
		stages,
		None, // no vertex input/assembly
		None, // no tesselation
		[Viewport::new(0.0, 0.0, dimensions.0 as f32, dimensions.1 as f32, 0.0, 1.0)],
		[Scissor::new(0, 0, dimensions.0, dimensions.1)],
		pipeline::Rasterization::new(
			false,
			false,
			pipeline::rasterization::PolygonMode::Fill,
			pipeline::rasterization::CullMode::Back,
			pipeline::rasterization::FrontFace::Clockwise,
			None,
			1.0
		),
		pipeline::Multisample::default(), // no multisampling
		None,
		None,
		ColorBlend::new(None, [0.0, 0.0, 0.0, 0.0]).with_attachment(color_blend::Attachment::new(
			Some(color_blend::AttachmentBlend::new(
				BlendFactor::SourceAlpha,
				BlendFactor::OneMinusSourceAlpha,
				color_blend::Operation::Add,
				BlendFactor::One,
				BlendFactor::Zero,
				color_blend::Operation::Add
			)),
			color_blend::ColorComponents::rgba()
		)),
		&layout,
		render_pass.subpass(0).unwrap()
	).expect("unable to create pipeline");

	event_loop.run(move |event, _, _| {
		// TODO
	});
}

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

/// Load a shader module.
///
/// # Safety
/// 
/// The SPIR-V code is not validated or may require features that are not enabled.
unsafe fn load_shader_module<P: AsRef<Path>>(device: &Arc<Device>, path: P) -> Arc<shader::Module> {
	use std::fs::File;
	let mut file = File::open(path).expect("Unable to open shader file");
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer).expect("Unable to read shader file");
	Arc::new(shader::Module::new(device, &buffer).expect("Unable to load shader module"))
}

fn create_render_pass(device: &Arc<Device>, format: Format) -> Arc<framebuffer::RenderPass> {
	let mut attachments = framebuffer::render_pass::Attachments::new();

	let color_attachment = attachments.add(framebuffer::render_pass::Attachment {
		format,
		samples: 1u8.try_into().unwrap(),
		load: framebuffer::render_pass::LoadOp::Clear,
		store: framebuffer::render_pass::StoreOp::Store,
		stencil_load: framebuffer::render_pass::LoadOp::DontCare,
		stencil_store: framebuffer::render_pass::StoreOp::DontCare,
		initial_layout: image::Layout::Undefined,
		final_layout: image::Layout::PresentSrc
	});

	let subpass = framebuffer::render_pass::SubpassRef {
		color_attachments: &[color_attachment.with_layout(image::Layout::ColorAttachmentOptimal)],
		depth_stencil: None,
		input_attachments: &[],
		resolve_attachments: &[],
		preserve_attachments: &[]
	};

	let mut render_pass = framebuffer::RenderPassBuilder::new(&attachments);
	render_pass.add(subpass);

	Arc::new(render_pass.build(device).expect("unable to build render pass"))
}