use winit::window::Window;

use crate::renderer::{
	wgpu_attributes::WGPUAttributes,
	wgpu_render_pipeline::WGPURenderPipelines,
};
use crate::scene::scene::Scene;

pub struct WGPURenderer {
	adapter: wgpu::Adapter,
	attributes: WGPUAttributes,
	device: wgpu::Device,
	height: f64,
	pixel_ratio: f64,
	queue: wgpu::Queue,
	render_pipelines: WGPURenderPipelines,
	surface: wgpu::Surface,
	surface_configuration: wgpu::SurfaceConfiguration,
	width: f64
}

impl WGPURenderer {
	pub async fn new(window: &Window) -> Self {
		let width = 640.0;
		let height = 480.0;
		let pixel_ratio = 1.0;

		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
				power_preference: wgpu::PowerPreference::default(),
			})
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					label: None,
					limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
				},
				None,
			)
			.await
			.expect("Failed to create device");

		let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

		let surface_configuration = wgpu::SurfaceConfiguration {
			format: swapchain_format,
			height: (height * pixel_ratio) as u32,
			present_mode: wgpu::PresentMode::Mailbox,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			width: (width * pixel_ratio) as u32,
		};

		surface.configure(&device, &surface_configuration);

		WGPURenderer {
			adapter: adapter,
			attributes: WGPUAttributes::new(),
			device: device,
			height: height,
			pixel_ratio: pixel_ratio,
			queue: queue,
			render_pipelines: WGPURenderPipelines::new(),
			surface: surface,
			surface_configuration: surface_configuration,
			width: width
		}
	}

	pub fn set_size(&mut self, width: f64, height: f64) -> &mut Self {
		self.width = width;
		self.height = height;

		self.surface_configuration.width = (self.width * self.pixel_ratio) as u32;
		self.surface_configuration.height = (self.height * self.pixel_ratio) as u32;
		self.surface.configure(&self.device, &self.surface_configuration);

		self
	}

	pub fn get_size(&self) -> (f64, f64) {
		(self.width, self.height)
	}

	pub fn set_pixel_ratio(&mut self, pixel_ratio: f64) -> &mut Self {
		self.pixel_ratio = pixel_ratio;
		self.set_size(self.width, self.height)
	}

	pub fn get_pixel_ratio(&self) -> f64 {
		self.pixel_ratio
	}

	pub fn render(&mut self, scene: &Scene) {
		let pipeline = self.render_pipelines.get(&self.device, &self.adapter, &self.surface);

		let frame = self.surface
			.get_current_frame()
			.expect("Failed to acquire next swap chain texture")
			.output;

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(
			&wgpu::CommandEncoderDescriptor {label: None});

		{
			let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
						store: true,
					},
				}],
				depth_stencil_attachment: None,
			});

			pass.set_pipeline(&pipeline);

			for i in 0..scene.get_objects_num() {
				let object = scene.borrow_object(i).unwrap();
				if let Some(mesh) = scene.borrow_mesh(object.get_id()) {
					let geometry = mesh.borrow_geometry();
					// @TODO: Implement correctly
					if let Some(attribute) = geometry.borrow_attribute("position") {
						if self.attributes.get(attribute).is_none() {
							self.attributes.update(&self.device, attribute);
						}
					}
					if let Some(attribute) = geometry.borrow_attribute("normal") {
						if self.attributes.get(attribute).is_none() {
							self.attributes.update(&self.device, attribute);
						}
					}
				}
			}

			for i in 0..scene.get_objects_num() {
				let object = scene.borrow_object(i).unwrap();
				if let Some(mesh) = scene.borrow_mesh(object.get_id()) {
					let geometry = mesh.borrow_geometry();
					// @TODO: Should be programmable
					if let Some(positions) = geometry.borrow_attribute("position") {
						if let Some(buffer) = self.attributes.get(positions) {
							pass.set_vertex_buffer(0, buffer.slice(..));
						}
					}
					if let Some(normals) = geometry.borrow_attribute("normal") {
						if let Some(buffer) = self.attributes.get(normals) {
							pass.set_vertex_buffer(1, buffer.slice(..));
						}
					}
					let geometry = mesh.borrow_geometry();
					let positions = geometry.borrow_attribute("position").unwrap();
					pass.draw(0..positions.get_count(), 0..1);
				}
			}

		}

		self.queue.submit(Some(encoder.finish()));
	}
}