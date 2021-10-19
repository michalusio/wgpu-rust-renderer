use wasm_bindgen::{
	JsCast,
	prelude::*,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
	Request,
	RequestInit,
	RequestMode,
	Response,
};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use wgpu_rust_renderer::{
	math::{
		color::Color,
		vector3::Vector3,
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		node::Node,
		scene::Scene,
	},
	utils::{
		geometry_helper::GeometryHelper,
		material_helper::MaterialHelper,
		texture_loader::TextureLoader,
	},
	web::wgpu_web_renderer::WGPUWebRenderer,
};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

// Window and DOM element helpers

fn get_window_inner_size() -> (f64, f64) {
	let window = web_sys::window().unwrap();
	(
		window.inner_width().unwrap().as_f64().unwrap(),
		window.inner_height().unwrap().as_f64().unwrap()
	)
}

fn get_window_device_pixel_ratio() -> f64 {
	let window = web_sys::window().unwrap();
	window.device_pixel_ratio()
}

async fn create_scene(
	pools: &mut ResourcePools
) -> (ResourceId<Scene>, ResourceId<PerspectiveCamera>, Vec<ResourceId<Node>>) {
	let mut objects = Vec::new();
	let mut scene = Scene::new();

	let geometry = GeometryHelper::create_box(
		pools,
		1.0,
		1.0,
		1.0,
	);

	let texture = TextureLoader::load_png(
		pools,
		std::io::Cursor::new(
			// Path from index.html
			load_file("./assets/texture.png".to_string())
				.await
				.unwrap(),
		)
	);

	let material = MaterialHelper::create_basic_material_with_texture(
		pools,
		Color::set(&mut Color::create(), 1.0, 1.0, 1.0),
		texture,
	);

	let mesh = pools.borrow_mut::<Mesh>().add(Mesh::new(geometry, material));
	let mut node = Node::new();
	node.borrow_rotation_mut()[0] = 35.0_f32.to_radians();
	let node = pools.borrow_mut::<Node>().add(node);
	scene.add_node(&node);
	scene.assign(&node, &mesh);
	objects.push(node);

	let window_size = get_window_inner_size();
	let camera = pools.borrow_mut::<PerspectiveCamera>().add(
		PerspectiveCamera::new(
			60.0_f32.to_radians(),
			(window_size.0 / window_size.1) as f32,
			0.1,
			1000.0,
		),
	);

	let mut node = Node::new();
	Vector3::set(
		node.borrow_position_mut(),
		0.0, 0.0, 3.0,
	);

	let node = pools.borrow_mut::<Node>().add(node);
	scene.add_node(&node);
	scene.assign(&node, &camera);

	(pools.borrow_mut::<Scene>().add(scene), camera, objects)
}

fn resize(
	renderer: &mut WGPUWebRenderer,
	pools: &mut ResourcePools,
	camera: &ResourceId<PerspectiveCamera>,
	width: u32,
	height: u32,
) {
	pools
		.borrow_mut::<PerspectiveCamera>()
		.borrow_mut(camera)
		.unwrap()
		.set_aspect(width as f32 / height as f32);
	renderer.set_size(width as f64, height as f64);
}

fn update(
	pools: &mut ResourcePools,
	scene: &ResourceId<Scene>,
	objects: &Vec<ResourceId<Node>>,
) {
	{
		let node = pools.borrow_mut::<Node>().borrow_mut(&objects[0]).unwrap();
		Vector3::add(
			node.borrow_rotation_mut(),
			&[0.0, 0.01, 0.0],
		);
	}

	pools.borrow::<Scene>()
		.borrow(scene)
		.unwrap()
		.update_matrices(pools);
}

fn render(
	renderer: &mut WGPUWebRenderer,
	pools: &ResourcePools,
	scene: &ResourceId<Scene>,
	camera: &ResourceId<PerspectiveCamera>,
) {
	renderer.render(pools, scene, camera);
}

fn create_window(event_loop: &EventLoop<()>) -> std::rc::Rc<winit::window::Window> {
	let window = winit::window::Window::new(&event_loop).unwrap();
	let window = std::rc::Rc::new(window);

	// winit::window::Window doesn't seem to detect browser's onresize event so we emulate it.
    {
		let window = window.clone();
		let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
			let size = get_window_inner_size();
			window.set_inner_size(winit::dpi::PhysicalSize::new(
				size.0, size.1,
			));
		}) as Box<dyn FnMut(_)>);
		web_sys::window()
			.unwrap()
			.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
			.unwrap();
		closure.forget();
    }

	window
}

#[wasm_bindgen(start)]
pub async fn start() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	console_log::init().expect("could not initialize logger");

	let event_loop = EventLoop::new();
	let window = create_window(&event_loop);

	use winit::platform::web::WindowExtWebSys;

	web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.body())
		.and_then(|body| {
			body.append_child(&web_sys::Element::from(window.canvas()))
				.ok()
		})
		.expect("couldn't append canvas to document body");

	let inner_size = get_window_inner_size();
	let pixel_ratio = get_window_device_pixel_ratio();

	let mut renderer = WGPUWebRenderer::new(&window, window.canvas()).await;
	renderer.set_size(inner_size.0 as f64, inner_size.1 as f64);
	renderer.set_pixel_ratio(pixel_ratio as f64);

	let mut pools = ResourcePools::new();
	let (scene, camera, objects) = create_scene(&mut pools).await;

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				resize(&mut renderer, &mut pools, &camera, size.width, size.height);
				update(&mut pools, &scene, &objects);
				render(&mut renderer, &mut pools, &scene, &camera);
			},
			Event::RedrawEventsCleared => {
                window.request_redraw();
            },
			Event::RedrawRequested(_) => {
				update(&mut pools, &scene, &objects);
				render(&mut renderer, &mut pools, &scene, &camera);
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

// @TODO: Proper error handling
pub async fn load_file(url: String) -> Result<Vec<u8>, String> {
	let mut opts = RequestInit::new();
	opts.method("GET");
	opts.mode(RequestMode::Cors); // @TODO: Should be able to opt-out

	let request = match Request::new_with_str_and_init(&url, &opts) {
		Ok(request) => request,
		Err(_e) => return Err("Failed to create request".to_string()),
	};

	let window = web_sys::window().unwrap();
	let response = match JsFuture::from(window.fetch_with_request(&request)).await {
		Ok(response) => response,
		Err(_e) => return Err("Failed to fetch".to_string()),
	};

	let response: Response = match response.dyn_into() {
		Ok(response) => response,
		Err(_e) => return Err("Failed to dyn_into Response".to_string()),
	};

	let buffer = match response.array_buffer() {
		Ok(buffer) => buffer,
		Err(_e) => return Err("Failed to get as array buffer".to_string()),
	};

	let buffer = match JsFuture::from(buffer).await {
		Ok(buffer) => buffer,
		Err(_e) => return Err("Failed to ...?".to_string()),
	};

	Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
