use std::{
	sync::Arc,
	rc::Rc,
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
		self,
		Surface,
		capabilities::{
			CompositeAlpha,
			PresentMode
		},
		Capabilities,
		capabilities::ColorSpace
	},
	image,
	Image,
	pipeline::{
		self,
		shader,
		VertexInput,
		input_assembly,
		InputAssembly,
		Viewport,
		Scissor,
		ColorBlend,
		color_blend::{
			self,
			BlendFactor
		},
		layout,
		DynamicStates
	},
	win::{
		self,
		WindowBuilderExt
	},
	framebuffer,
	Framebuffer,
	format::ClearValue,
	Format,
	command::{
		self,
		Pool,
		Buffer as CommandBuffer
	},
	sync::{
		Task,
		semaphore,
		fence,
		future::{
			SignalSemaphore,
			SignalSemaphores,
			SignalFence
		}
	},
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

	for physical_device in instance.physical_devices() {
		println!("device: {}", physical_device.name());
	}

	let physical_device = Arc::new(instance.physical_devices().last().unwrap());
	println!("choosen device: {}", physical_device.name());

	let event_loop = EventLoop::new();
	let surface = Arc::new(WindowBuilder::new().build_vk_surface(&event_loop, &instance).unwrap());
	let dimensions: (u32, u32) = surface.backend().inner_size().into();

	// Create logical device (and queues).
	let queue_family = get_queue_family(&physical_device, &surface);
	let (device, mut queues) = get_device(&physical_device, queue_family);
	let queue = queues.next().unwrap();

	let surface_capabilities = surface.capabilities(device.physical_device()).unwrap();
	let (color_format, color_space) = choose_format(&surface_capabilities).expect("No appropriate format found");

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

	let render_pass = create_render_pass(&device, color_format);

	let layout = layout::Empty::new(&device).expect("unable to create pipeline layout");
	
	let pipeline: Arc<pipeline::Graphics<layout::Empty, (), ()>> = Arc::new(pipeline::Graphics::new(
		&device,
		&stages,
		(), // no vertex input
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
		layout,
		render_pass.subpass(0).unwrap()
	).expect("unable to create pipeline"));

	let mut renderer = None;
	let mut render_queue = Some(queue);

	event_loop.run(move |event, _, _| {
		// println!("event: {:?}", event);
		match event {
			winit::event::Event::RedrawRequested(_) => {
				if renderer.is_none() {
					renderer = Some(Renderer::new(
						&device,
						&surface,
						color_format,
						color_space,
						dimensions,
						render_queue.take().unwrap(),
						&render_pass,
						&pipeline
					));
				}

				renderer.as_mut().unwrap().render()
			},
			_ => ()
		}
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

pub struct Renderer<W> {
	swapchain: Swapchain<W>,
	command_buffers: Vec<command::buffer::LocallyRecorded<'static, command::pool::raw::RcBuffer>>,
	queue: Queue,
	image_available_semaphore: semaphore::Raw,
	render_finished_semaphore: semaphore::Raw,
	render_finished_fence: fence::Raw
}

impl<W: 'static> Renderer<W> {
	pub fn new(
		device: &Arc<Device>,
		surface: &Arc<Surface<W>>,
		color_format: Format,
		color_space: ColorSpace,
		dimensions: (u32, u32),
		queue: Queue,
		render_pass: &Arc<framebuffer::RenderPass>,
		pipeline: &Arc<pipeline::Graphics<layout::Empty, (), ()>>
	) -> Self {
		let surface_capabilities = surface.capabilities(device.physical_device()).unwrap();

		let (swapchain, swapchain_images) = Swapchain::new(
			device,
			surface,
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
		).expect("unable to create swapchain");
	
		let framebuffers: Vec<_> = swapchain_images.into_iter().map(|i| {
			let view = i.into_view(
				image::view::Type::D2,
				color_format,
				image::view::ComponentMapping::default(), // identity
				image::view::SubresourceRange {
					aspects: image::view::Aspects::color(),
					base_mip_level: 0,
					level_count: 1,
					base_array_layer: 0,
					layer_count: 1
				}
			).expect("unable to create swapchain image view");
	
			Arc::new(Framebuffer::new(
				&device,
				&render_pass,
				vec![Arc::new(view)],
				dimensions,
				1
			).expect("unable to create framebuffer"))
		}).collect();
	
		let pool = Rc::new(command::pool::Raw::new(&device, queue.family()).expect("unable to create command pool"));
		let command_buffers = pool.allocate_rc(framebuffers.len() as u32).expect("unable to allocate command buffers");
		let recorded_command_buffers: Vec<_> = command_buffers.into_iter().enumerate().map(|(i, buffer)| {
			buffer.record_local(|b| {
				let mut render_pass = b.begin_render_pass(
					&render_pass,
					&framebuffers[i],
					(0, 0, dimensions.0, dimensions.1),
					&[ClearValue::f32color(0.0, 0.0, 0.0, 1.0)]
				);

				render_pass.bind_pipeline(&pipeline, ()).draw((), (), 3, 1, 0, 0);
			}).expect("unable to record command buffer")
		}).collect();
	
		let image_available_semaphore = semaphore::Raw::new(&device).expect("unable to create semaphore");
		let render_finished_semaphore = semaphore::Raw::new(&device).expect("unable to create semaphore");
		let render_finished_fence = fence::Raw::new(&device).expect("unable to create fence");

		Renderer {
			swapchain,
			command_buffers: recorded_command_buffers,
			queue,
			image_available_semaphore,
			render_finished_semaphore,
			render_finished_fence
		}
	}

	pub fn render(&mut self) {
		let ((next_index, _), image_acquired) = self.swapchain
			.acquire_next_image(None)
			.then_signal_semaphore(&self.image_available_semaphore)
			.expect("unable to acquire next image");

		let ((), render_finished) = image_acquired
			.and_then_pipeline_stages_of(self.queue.submit(&self.command_buffers[next_index as usize]), pipeline::stage::Flags::TOP_OF_PIPE)
			.then_signal_semaphore_and_fence(&self.render_finished_semaphore, &self.render_finished_fence)
			.expect("unable to render");

		let (_, render_finished) = render_finished
			.and_then(self.queue.present(&self.swapchain, next_index))
			.in_parallel()
			.expect("unable to present");

		render_finished.wait(None).expect("unable to wait for the render");
	}
}