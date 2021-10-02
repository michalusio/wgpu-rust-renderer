use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};
use wgpu_rust_renderer::{
	renderer::wgpu_renderer::WGPURenderer,
	scene::{
		attribute::AttributeManager,
		geometry::Geometry,
		mesh::Mesh,
		scene::Scene,
	}
};

fn create_scene() -> Scene {
	let mut scene = Scene::new();
	let mut attribute_manager = AttributeManager::new();

	let mut geometry = Geometry::new();

	geometry.set_attribute("position", attribute_manager.create(
		[
			0.0, 0.5, 0.0,
			0.5, -0.5, 0.0,
			-0.5, -0.5, 0.0,
		].to_vec(),
		3,
	));

	geometry.set_attribute("normal", attribute_manager.create(
		[
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
		].to_vec(),
		3,
	));

	let mesh = Mesh::new(geometry);
	let id = scene.create_object();
	scene.add_mesh(id, mesh);

	scene
}

#[tokio::main]
async fn main() {
	let event_loop = EventLoop::new();
	let window = Window::new(&event_loop).unwrap();

	let window_size = window.inner_size();
	let pixel_ratio = window.scale_factor();

	let mut renderer = WGPURenderer::new(&window).await;
	renderer.set_size(window_size.width as f64, window_size.height as f64);
	renderer.set_pixel_ratio(pixel_ratio);

	let scene = create_scene();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				renderer.set_size(size.width as f64, size.height as f64);
			},
			Event::RedrawRequested(_) => {
				renderer.render(&scene);
			},
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				*control_flow = ControlFlow::Exit;
			},
			_ => {}
		}
	});
}