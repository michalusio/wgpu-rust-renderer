use crate::math::Matrix4;

pub struct PerspectiveCamera {
	aspect: f32,
	far: f32,
	fovy: f32,
	near: f32,
	projection_matrix: Matrix4,
	projection_matrix_inverse: Matrix4,
}

impl PerspectiveCamera {
	pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
		let mut camera = PerspectiveCamera {
			aspect: aspect,
			far: far,
			fovy: fovy,
			near: near,
			projection_matrix: Matrix4::identity(),
			projection_matrix_inverse: Matrix4::identity(),
		};
		camera.update_projection_matrix();
		camera
	}

	pub fn set_aspect(&mut self, aspect: f32) -> &mut Self {
		self.aspect = aspect;
		self.update_projection_matrix();
		self
	}

	pub fn update_projection_matrix(&mut self) {
		self.projection_matrix = Matrix4::make_perspective(
			self.fovy,
			self.aspect,
			self.near,
			self.far,
		);
		self.projection_matrix_inverse = self.projection_matrix.invert();
	}

	pub fn borrow_projection_matrix(&self) -> &Matrix4 {
		&self.projection_matrix
	}

	pub fn borrow_projection_matrix_inverse(&self) -> &Matrix4 {
		&self.projection_matrix_inverse
	}
}