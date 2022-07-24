use crate::{
	math::{
		Vector3, Quaternion, Euler, Matrix4,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
};

pub struct Node {
	children: Vec<ResourceId<Node>>,
	matrix: Matrix4,
	parent: Option<ResourceId<Node>>,
	position: Vector3,
	quaternion: Quaternion,
	rotation: Vector3,
	scale: Vector3,
	world_matrix: Matrix4,
}

impl Node {
	pub fn new() -> Self {
		Node {
			children: Vec::new(),
			matrix: Matrix4::identity(),
			parent: None,
			position: Vector3::default(),
			quaternion: Quaternion::of([0.0, 0.0, 0.0, 1.0]),
			rotation: Euler::default(),
			scale: Vector3::of([1.0, 1.0, 1.0]),
			world_matrix: Matrix4::identity(),
		}
	}

	pub fn from(position: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) -> Self {
		let mut node = Node::new();
		if let Some(position) = position {
			node.set_position(position);
		}
		if let Some(rotation) = rotation {
			node.set_rotation(Euler::from_quaternion(rotation));
		}
		if let Some(scale) = scale {
			node.set_scale(scale);
		}
		node
	}

	pub fn borrow_parent(&self) -> Option<&ResourceId<Node>> {
		self.parent.as_ref()
	}

	pub fn borrow_children(&self) -> &Vec<ResourceId<Node>> {
		&self.children
	}

	pub fn get_position(&self) -> Vector3 {
		self.position
	}

	pub fn set_position(&mut self, position: Vector3) {
		self.position = position;
	}

	pub fn get_rotation(&self) -> Euler {
		self.rotation
	}

	pub fn set_rotation(&mut self, rotation: Euler) {
		self.rotation = rotation;
	}

	pub fn get_scale(&self) -> Vector3 {
		self.scale
	}

	pub fn set_scale(&mut self, scale: Vector3) {
		self.scale = scale;
	}

	pub fn get_matrix(&self) -> Matrix4 {
		self.matrix
	}

	pub fn set_matrix(&mut self, matrix: Matrix4) -> &mut Self {
		self.matrix = matrix;
		(self.position, self.quaternion, self.scale) = self.matrix.decompose();
		self.rotation = Euler::from_quaternion(self.quaternion);
		self
	}

	pub fn get_world_matrix(&self) -> Matrix4 {
		self.world_matrix
	}

	pub fn set_world_matrix(&mut self, matrix: Matrix4) -> &mut Self {
		self.world_matrix = matrix;
		self
	}

	pub fn update_matrix(&mut self) -> &mut Self {
		self.quaternion = Quaternion::from_euler(self.rotation);
		self.matrix = Matrix4::compose(self.position, self.quaternion, self.scale);
		self
	}

	// @TODO: Optimize
	pub fn update_matrices(
		&mut self,
		pool: &mut ResourcePool<Node>,
	) {
		self.update_matrix();

		if let Some(parent) = self.borrow_parent() {
			let parent_matrix = pool.borrow(parent).unwrap().get_world_matrix();
			self.world_matrix = parent_matrix.multiply(self.matrix);
		} else {
			self.world_matrix = self.matrix;
		}

		let mut stack = Vec::new();

		for child in self.children.iter() {
			stack.push(*child);
		}

		while let Some(rid) = stack.pop() {
			let parent_matrix = {
				let node = pool.borrow_mut(&rid).unwrap();
				let parent = node.borrow_parent().cloned().unwrap();
				pool.borrow(&parent).unwrap().get_world_matrix()
			};

			let node = pool.borrow_mut(&rid).unwrap();
			node.set_world_matrix(parent_matrix.multiply(node.get_matrix()));

			for child in node.children.iter() {
				stack.push(*child);
			}
		}
	}
}

pub struct NodeExecutor {
}

impl NodeExecutor {
	pub fn update_matrices(
		pool: &mut ResourcePool<Node>,
		root: &ResourceId<Node>,
	) {
		let mut stack = Vec::new();
		stack.push(*root);

		while let Some(rid) = stack.pop() {
			let node = pool.borrow_mut(&rid).unwrap();
			node.update_matrix();

			let parent_matrix = {
				let node = pool.borrow_mut(&rid).unwrap();
				if let Some(parent) = node.borrow_parent().cloned() {
					pool.borrow(&parent).unwrap().get_world_matrix()
				} else {
					Matrix4::identity()
				}
			};

			let node = pool.borrow_mut(&rid).unwrap();
			node.set_world_matrix(parent_matrix.multiply(node.get_matrix()));

			for child in node.children.iter() {
				stack.push(*child);
			}
		}
	}

	pub fn collect_nodes(
		pool: &ResourcePool<Node>,
		root: &ResourceId<Node>,
		nodes: &mut Vec<ResourceId<Node>>,
	) {
		let mut stack = Vec::new();
		stack.push(*root);
		nodes.push(*root);

		while let Some(rid) = stack.pop() {
			let node = pool.borrow(&rid).unwrap();
			for child in node.children.iter() {
				stack.push(*child);
				nodes.push(*child);
			}
		}
	}
}
