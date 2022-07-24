use super::{vector::Vector, euler::Euler, matrix4::Matrix4};

pub type Quaternion = Vector<f32, 4>;

impl Quaternion {
	pub fn create() -> Quaternion {
		Quaternion::of([0.0, 0.0, 0.0, 1.0])
	}

	pub fn from_euler(e: Euler) -> Quaternion {
		// Assume XYZ order
		let x = e[0];
		let y = e[1];
		let z = e[2];

		let c1 = (x / 2.0).cos();
		let c2 = (y / 2.0).cos();
		let c3 = (z / 2.0).cos();

		let s1 = (x / 2.0).sin();
		let s2 = (y / 2.0).sin();
		let s3 = (z / 2.0).sin();

		Quaternion::of([
			s1 * c2 * c3 + c1 * s2 * s3,
			c1 * s2 * c3 - s1 * c2 * s3,
			c1 * c2 * s3 + s1 * s2 * c3,
			c1 * c2 * c3 - s1 * s2 * s3
		])
	}

	pub fn from_rotation_matrix(
		m: Matrix4,
	) -> Quaternion {
		let m11 = m[0];
		let m12 = m[4];
		let m13 = m[8];
		let m21 = m[1];
		let m22 = m[5];
		let m23 = m[9];
		let m31 = m[2];
		let m32 = m[6];
		let m33 = m[10];

		let trace = m11 + m22 + m33;

		if trace > 0.0 {
			let s = 0.5 / (trace + 1.0).sqrt();
			Quaternion::of([
				(m32 - m23) * s,
				(m13 - m31) * s,
				(m21 - m12) * s,
				0.25 / s
			])
		} else if m11 > m22 && m11 > m33 {
			let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
			Quaternion::of([
				0.25 * s,
				(m12 + m21) / s,
				(m13 + m31) / s,
				(m32 - m23) / s
			])
		} else if m22 > m33 {
			let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
			Quaternion::of([
				(m12 + m21) / s,
				0.25 * s,
				(m23 + m32) / s,
				(m13 - m31) / s
			])
		} else {
			let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
			Quaternion::of([
				(m13 + m31) / s,
				(m23 + m32) / s,
				0.25 * s,
				(m21 - m12) / s
			])
		}
	}
}
