use super::{vector::Vector, matrix3::Matrix3};

pub type Matrix3GPU = Vector<f32, 12>;

impl Matrix3GPU {
	pub fn identity() -> Matrix3GPU {
		Matrix3GPU::from([
			1.0,
			0.0,
			0.0,
			0.0,
			0.0,
			1.0,
			0.0,
			0.0,
			0.0,
			0.0,
			1.0,
			0.0
		])
	}

	pub fn from_matrix3(src: Matrix3) -> Matrix3GPU {
		Matrix3GPU::from([
			src[0],
			src[1],
			src[2],
			0.0,
			src[3],
			src[4],
			src[5],
			0.0,
			src[6],
			src[7],
			src[8],
			0.0
		])
	}
}
