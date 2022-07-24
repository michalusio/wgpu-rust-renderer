use gltf::Gltf;

use crate::{
	geometry::{
		attribute::Attribute,
		geometry::Geometry,
		index::Index,
	},
	material::{
		material::{
			Material,
			Side,
		},
		node::{
			add::AddNode,
			brdf::{
				BRDFNode,
				BRDFNodeDescriptor,
			},
			const_float::ConstFloatNode,
			float::FloatNode,
			multiply::MultiplyNode,
			node::MaterialNode,
			normal::NormalNode,
			sub::SubNode,
			tangent_to_object_normal::TangentToObjectNormalNode,
			texture::TextureNode,
			vector3::Vector3Node,
			xyz::XYZNode,
			y::YNode,
			z::ZNode,
		},
	},
	math::{
		Color, Euler, Vector3, Quaternion, Matrix4,
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		mesh::Mesh,
		node::Node,
		scene::Scene,
	},
	texture::{
		sampler::{
			FilterMode,
			Sampler,
			SamplerDescriptor,
			WrapMode
		},
		texture::{
			Texture,
			TextureFormat,
		},
	},
	utils::{
		file_loader::FileLoader,
		texture_loader::TextureLoader,
	},
};

async fn parse_attribute(
	pools: &mut ResourcePools,
	path: &str,
	primitive: &gltf::Attribute<'_>,
) -> (&'static str, ResourceId<Attribute>) {
	let (semantic, accessor) = primitive;
	use gltf::mesh::Semantic;
	if let Some(view) = accessor.view() {
		let offset = view.offset();
		let length = view.length();
		let buffer = view.buffer();

		use gltf::buffer::Source;
		use std::io::{Read, Seek, SeekFrom};
		let data = match buffer.source() {
			Source::Bin => {
				panic!("Bin is not supported yet");
			},
			Source::Uri(uri) => {
				let mut buf = [0_u8; 4];
				let mut data = Vec::<f32>::new();
				let mut file = FileLoader::open(
					&(path.to_owned() + uri),
				).await;
				for i in 0..(length / 4) {
					file.seek(SeekFrom::Start((offset + i * 4) as u64)).unwrap();
					file.read_exact(&mut buf).unwrap();
					data.push(f32::from_le_bytes(buf));
				}
				data
			}
		};

		let (name, attribute) = match semantic {
			Semantic::Normals => {(
				"normal",
				Attribute::new(data, 3),
			)},
			Semantic::Positions => {(
				"position",
				Attribute::new(data, 3),
			)},
			Semantic::TexCoords(_) => {(
				"uv",
				Attribute::new(data, 2),
			)},
			_ => {
				panic!("Unsupport accessor semantic.");
			},
		};

		(name, pools.borrow_mut::<Attribute>().add(attribute))
	} else {
		panic!("Sparse accessor is not supported yet.");
	}
}

async fn parse_geometry(
	pools: &mut ResourcePools,
	path: &str,
	primitive_def: &gltf::Primitive<'_>,
) -> ResourceId<Geometry> {
	let mut geometry = Geometry::new();

	for attribute_def in primitive_def.attributes() {
		let (name, attribute) = parse_attribute(pools, path, &attribute_def).await;
		geometry.set_attribute(&name, attribute);
	}

	if let Some(accessor) = primitive_def.indices() {
		let index = parse_index(pools, path, &accessor).await;
		geometry.set_index(index);
	}

	pools.borrow_mut::<Geometry>().add(geometry)
}

async fn parse_material(
	pools: &mut ResourcePools,
	path: &str,
	material_def: &gltf::Material<'_>,
) -> ResourceId<Material> {
	let pbr_metallic_roughness = material_def.pbr_metallic_roughness();

	// Base color

	let base_color_factor = pbr_metallic_roughness.base_color_factor();

	let base_color = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
		Vector3Node::new(
			Color::of([base_color_factor[0], base_color_factor[1], base_color_factor[2]]),
		),
	));

	let base_color = if let Some(info) = pbr_metallic_roughness.base_color_texture() {
		let (texture, sampler) = parse_texture_info(pools, path, &info, TextureFormat::Uint8Srgb).await;

		let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TextureNode::new(texture, sampler)),
		);

		let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(XYZNode::new(texture_node)),
		);

		pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(base_color, texture_rgb))
		)
	} else {
		base_color
	};

	// Metallic/Roughness

	let metallic_factor = pbr_metallic_roughness.metallic_factor();
	let roughness_factor = pbr_metallic_roughness.roughness_factor();

	let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
		Box::new(FloatNode::new(metallic_factor)),
	);

	let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
		Box::new(FloatNode::new(roughness_factor)),
	);

	let (metallic, roughness) = if let Some(info) = pbr_metallic_roughness.metallic_roughness_texture() {
		let (texture, sampler) = parse_texture_info(pools, path, &info, TextureFormat::default()).await;

		let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TextureNode::new(texture, sampler)),
		);

		let texture_g = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(YNode::new(texture_node)),
		);

		let texture_b = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(ZNode::new(texture_node)),
		);

		let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(metallic, texture_b)),
		);

		let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(roughness, texture_g)),
		);

		(metallic, roughness)
	} else {
		(metallic, roughness)
	};

	// Normal

	let normal = if let Some(info) = material_def.normal_texture() {
		let (texture, sampler) = parse_normal_texture_info(pools, path, &info).await;

		let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TextureNode::new(texture, sampler)),
		);

		let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(XYZNode::new(texture_node)),
		);

		let const_2 = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(ConstFloatNode::new(2.0)),
		);

		let const_1 = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(ConstFloatNode::new(1.0)),
		);

		let multiply = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(texture_rgb, const_2)),
		);

		let sub = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(SubNode::new(multiply, const_1)),
		);

		pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TangentToObjectNormalNode::new(sub)),
		)
	} else {
		pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
			NormalNode::new()
		))
	};

	// BRDF

	let brdf = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
		BRDFNode::new(BRDFNodeDescriptor {
			base_color: base_color,
			metallic: metallic,
			normal: normal,
			roughness: roughness,
		}),
	));

	// Emissive

	let emissive_factor = material_def.emissive_factor();
	let emissive = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
		Vector3Node::new(
			Color::of([emissive_factor[0], emissive_factor[1], emissive_factor[2]]),
		),
	));

	let emissive = if let Some(info) = material_def.emissive_texture() {
		let (texture, sampler) = parse_texture_info(pools, path, &info, TextureFormat::Uint8Srgb).await;

		let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TextureNode::new(texture, sampler)),
		);

		let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(XYZNode::new(texture_node)),
		);

		pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(emissive, texture_rgb))
		)
	} else {
		emissive
	};

	let add = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
		AddNode::new(
			brdf,
			emissive,
		),
	));

	pools.borrow_mut::<Material>().add(Material::new(add, Side::default()))
}

async fn parse_node(
	pools: &mut ResourcePools,
	scene: &ResourceId<Scene>,
	path: &str,
	node_def: &gltf::Node<'_>,
) -> ResourceId<Node> {
	let mut node = Node::new();

	match node_def.transform() {
		gltf::scene::Transform::Matrix {
			matrix,
		} => {
			node.set_matrix(Matrix4::from_2d_array(&matrix));
		},
		gltf::scene::Transform::Decomposed {
			translation,
			rotation,
			scale,
		} => {
			node.set_position(Vector3::of(translation));
			node.set_rotation(Euler::from_quaternion(Quaternion::of(rotation)));
			node.set_scale(Vector3::of(scale));
			node.update_matrix();
		},
	};

	let node = pools.borrow_mut::<Node>().add(Node::new());

	if let Some(mesh_def) = node_def.mesh() {
		for primitive_def in mesh_def.primitives() {
			let (geometry, material) = parse_primitive(pools, path, &primitive_def).await;
			let mesh = pools.borrow_mut::<Mesh>().add(Mesh::new(geometry, material));
			pools.borrow_mut::<Scene>().borrow_mut(scene).unwrap().assign(&node, &mesh);
		}
	}

	node
}

async fn parse_normal_texture_info(
	pools: &mut ResourcePools,
	path: &str,
	info: &gltf::material::NormalTexture<'_>,
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	parse_texture(pools, path, &info.texture(), TextureFormat::default()).await
}

async fn parse_index(
	pools: &mut ResourcePools,
	path: &str,
	index: &gltf::Accessor<'_>,
) -> ResourceId<Index> {
	if let Some(view) = index.view() {
		let offset = view.offset();
		let length = view.length();
		let buffer = view.buffer();

		use gltf::buffer::Source;
		use std::io::{Read, Seek, SeekFrom};
		let data = match buffer.source() {
			Source::Bin => {
				panic!("Bin is not supported yet");
			},
			Source::Uri(uri) => {
				let mut buf = [0_u8; 2];
				let mut data = Vec::<u16>::new();
				let mut file = FileLoader::open(
					&(path.to_owned() + uri),
				).await;
				for i in 0..(length / 2) {
					file.seek(SeekFrom::Start((offset + i * 2) as u64)).unwrap();
					file.read_exact(&mut buf).unwrap();
					data.push(u16::from_le_bytes(buf));
				}
				data
			}
		};

		pools.borrow_mut::<Index>().add(Index::new(data))
	} else {
		panic!("Sparse accessor is not supported yet.");
	}
}

async fn parse_primitive(
	pools: &mut ResourcePools,
	path: &str,
	primitive_def: &gltf::Primitive<'_>,
) -> (ResourceId<Geometry>, ResourceId<Material>) {
	(
		parse_geometry(pools, path, primitive_def).await,
		parse_material(pools, path, &primitive_def.material()).await
	)
}

fn parse_sampler(
	pools: &mut ResourcePools,
	sampler: &gltf::texture::Sampler,
) -> ResourceId<Sampler> {
	// @TODO: Proper default values
	pools.borrow_mut::<Sampler>().add(Sampler::new(
		SamplerDescriptor {
			mag_filter: match sampler.mag_filter() {
				Some(filter) => match filter {
					gltf::texture::MagFilter::Nearest => FilterMode::Nearest,
					gltf::texture::MagFilter::Linear => FilterMode::Linear,
				},
				None => FilterMode::Linear,
			},
			min_filter: match sampler.min_filter() {
				Some(filter) => match filter {
					gltf::texture::MinFilter::Linear |
					gltf::texture::MinFilter::LinearMipmapLinear |
					gltf::texture::MinFilter::LinearMipmapNearest => FilterMode::Linear,
					gltf::texture::MinFilter::Nearest |
					gltf::texture::MinFilter::NearestMipmapLinear |
					gltf::texture::MinFilter::NearestMipmapNearest => FilterMode::Nearest,
				},
				None => FilterMode::Linear,
			},
			mipmap_filter: match sampler.min_filter() {
				Some(filter) => match filter {
					gltf::texture::MinFilter::Linear |
					gltf::texture::MinFilter::Nearest => FilterMode::Linear,
					gltf::texture::MinFilter::LinearMipmapLinear |
					gltf::texture::MinFilter::NearestMipmapLinear => FilterMode::Linear,
					gltf::texture::MinFilter::LinearMipmapNearest |
					gltf::texture::MinFilter::NearestMipmapNearest => FilterMode::Nearest,
				},
				None => FilterMode::Linear,
			},
			wrap_u: match sampler.wrap_s() {
				gltf::texture::WrappingMode::ClampToEdge => WrapMode::ClampToEdge,
				gltf::texture::WrappingMode::MirroredRepeat => WrapMode::MirrorRepeat,
				gltf::texture::WrappingMode::Repeat => WrapMode::Repeat,
			},
			wrap_v: match sampler.wrap_t() {
				gltf::texture::WrappingMode::ClampToEdge => WrapMode::ClampToEdge,
				gltf::texture::WrappingMode::MirroredRepeat => WrapMode::MirrorRepeat,
				gltf::texture::WrappingMode::Repeat => WrapMode::Repeat,
			},
			wrap_w: WrapMode::Repeat,
		},
	))
}

async fn parse_texture(
	pools: &mut ResourcePools,
	path: &str,
	texture_def: &gltf::Texture<'_>,
	format: TextureFormat
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	let source = texture_def.source();

	use gltf::image::Source;
	let texture = match source.source() {
		Source::Uri {uri, mime_type: _mime_type} => {
			TextureLoader::load_with_filepath(
				pools,
				&(path.to_owned() + uri),
				format,
			).await
		},
		Source::View {..} => {
			panic!("Unsuppored");
		},
	};

	(texture, parse_sampler(pools, &texture_def.sampler()))
}

async fn parse_texture_info(
	pools: &mut ResourcePools,
	path: &str,
	info: &gltf::texture::Info<'_>,
	format: TextureFormat,
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	parse_texture(pools, path, &info.texture(), format).await
}

pub struct GltfLoader{
}

impl GltfLoader {
	pub async fn load_gltf(
		pools: &mut ResourcePools,
		scene: &ResourceId<Scene>,
		path: &str,
		filename: &str,
	) -> Vec<ResourceId<Node>> {
		let gltf = Gltf::from_reader(
			FileLoader::open(&(path.to_owned() + filename)).await,
		).unwrap();

		let mut nodes = Vec::new();

		let scene_def = gltf.default_scene().unwrap();
		for node_def in scene_def.nodes() {
			nodes.push(parse_node(pools, scene, path, &node_def).await);
		}

		nodes
	}
}