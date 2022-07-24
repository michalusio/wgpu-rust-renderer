use super::{vector::Vector, Vector3, quaternion::Quaternion};

pub type Matrix4 = Vector<f32, 16>;

impl Matrix4 {

	pub fn identity() -> Matrix4 {
		Matrix4::of([
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
			0.0,
			0.0,
			0.0,
			0.0,
			1.0,
		])
	}

	// Good name?
	pub fn from_2d_array<'a>(src: &'a [[f32; 4]; 4]) -> Matrix4 {
		let mut m = Matrix4::default();
		for i in 0..4 {
			for j in 0..4 {
				m[i * 4 + j] = src[j][i];
			}
		}
		m
	}

	pub fn multiply(
		&self,
		m2: Matrix4
	) -> Matrix4 {
		let mut m = Matrix4::default();
		let a00 = self[0];
		let a01 = self[1];
		let a02 = self[2];
		let a03 = self[3];
		let a10 = self[4];
		let a11 = self[5];
		let a12 = self[6];
		let a13 = self[7];
		let a20 = self[8];
		let a21 = self[9];
		let a22 = self[10];
		let a23 = self[11];
		let a30 = self[12];
		let a31 = self[13];
		let a32 = self[14];
		let a33 = self[15];

		let b0 = m2[0];
		let b1 = m2[1];
		let b2 = m2[2];
		let b3 = m2[3];
		m[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[4];
		let b1 = m2[5];
		let b2 = m2[6];
		let b3 = m2[7];
		m[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[8];
		let b1 = m2[9];
		let b2 = m2[10];
		let b3 = m2[11];
		m[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[12];
		let b1 = m2[13];
		let b2 = m2[14];
		let b3 = m2[15];
		m[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		m
	}

	pub fn invert(&self) -> Matrix4 {
		let a00 = self[0];
		let a01 = self[1];
		let a02 = self[2];
		let a03 = self[3];
		let a10 = self[4];
		let a11 = self[5];
		let a12 = self[6];
		let a13 = self[7];
		let a20 = self[8];
		let a21 = self[9];
		let a22 = self[10];
		let a23 = self[11];
		let a30 = self[12];
		let a31 = self[13];
		let a32 = self[14];
		let a33 = self[15];

		let b00 = a00 * a11 - a01 * a10;
		let b01 = a00 * a12 - a02 * a10;
		let b02 = a00 * a13 - a03 * a10;
		let b03 = a01 * a12 - a02 * a11;
		let b04 = a01 * a13 - a03 * a11;
		let b05 = a02 * a13 - a03 * a12;
		let b06 = a20 * a31 - a21 * a30;
		let b07 = a20 * a32 - a22 * a30;
		let b08 = a20 * a33 - a23 * a30;
		let b09 = a21 * a32 - a22 * a31;
		let b10 = a21 * a33 - a23 * a31;
		let b11 = a22 * a33 - a23 * a32;

		let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

		if det == 0.0 {
			panic!("Determinant of the matrix is 0!");
		}

		let det = 1.0 / det;
		Matrix4::of([
			(a11 * b11 - a12 * b10 + a13 * b09) * det,
			(a02 * b10 - a01 * b11 - a03 * b09) * det,
			(a31 * b05 - a32 * b04 + a33 * b03) * det,
			(a22 * b04 - a21 * b05 - a23 * b03) * det,
			(a12 * b08 - a10 * b11 - a13 * b07) * det,
			(a00 * b11 - a02 * b08 + a03 * b07) * det,
			(a32 * b02 - a30 * b05 - a33 * b01) * det,
			(a20 * b05 - a22 * b02 + a23 * b01) * det,
			(a10 * b10 - a11 * b08 + a13 * b06) * det,
			(a01 * b08 - a00 * b10 - a03 * b06) * det,
			(a30 * b04 - a31 * b02 + a33 * b00) * det,
			(a21 * b02 - a20 * b04 - a23 * b00) * det,
			(a11 * b07 - a10 * b09 - a12 * b06) * det,
			(a00 * b09 - a01 * b07 + a02 * b06) * det,
			(a31 * b01 - a30 * b03 - a32 * b00) * det,
			(a20 * b03 - a21 * b01 + a22 * b00) * det
		])
	}

	pub fn compose(
		position: Vector3,
		quaternion: Quaternion,
		scale: Vector3,
	) -> Matrix4 {
		let x = quaternion[0];
		let y = quaternion[1];
		let z = quaternion[2];
		let w = quaternion[3];

		let x2 = x + x;
		let y2 = y + y;
		let z2 = z + z;

		let xx = x * x2;
		let xy = x * y2;
		let xz = x * z2;

		let yy = y * y2;
		let yz = y * z2;
		let zz = z * z2;

		let wx = w * x2;
		let wy = w * y2;
		let wz = w * z2;

		let sx = scale[0];
		let sy = scale[1];
		let sz = scale[2];

		Matrix4::of([
			(1.0 - (yy + zz)) * sx,
			(xy + wz) * sx,
			(xz - wy) * sx,
			0.0,
			(xy - wz) * sy,
			(1.0 - (xx + zz)) * sy,
			(yz + wx) * sy,
			0.0,
			(xz + wy) * sz,
			(yz - wx) * sz,
			 (1.0 - (xx + yy)) * sz,
			 0.0,
			 position[0],
			 position[1],
			 position[2],
			 1.0,
		])
	}

	pub fn decompose(
		&self
	) -> (Vector3, Quaternion, Vector3) {
		let sx = Vector3::of([self[0], self[1], self[2]]).magnitude();
		let sy = Vector3::of([self[4], self[5], self[6]]).magnitude();
		let sz = Vector3::of([self[8], self[9], self[10]]).magnitude();

		let sx = match self.determinant() < 0.0 {
			true => -sx,
			false => sx,
		};

		let position = Vector3::of([self[12], self[13], self[14]]);

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		let mut m2 = self.clone();

		m2[0] *= inv_sx;
		m2[1] *= inv_sx;
		m2[2] *= inv_sx;

		m2[4] *= inv_sy;
		m2[5] *= inv_sy;
		m2[6] *= inv_sy;

		m2[8] *= inv_sz;
		m2[9] *= inv_sz;
		m2[10] *= inv_sz;

		let quaternion = Quaternion::from_rotation_matrix(m2);

		let scale = Vector3::of([sx, sy, sz]);
		(position, quaternion, scale)
	}

	pub fn determinant(&self) -> f32 {
		let n11 = self[0];
		let n12 = self[4];
		let n13 = self[8];
		let n14 = self[12];
		let n21 = self[1];
		let n22 = self[5];
		let n23 = self[9];
		let n24 = self[13];
		let n31 = self[2];
		let n32 = self[6];
		let n33 = self[10];
		let n34 = self[14];
		let n41 = self[3];
		let n42 = self[7];
		let n43 = self[11];
		let n44 = self[15];

		n41 * (
			n14 * n23 * n32
			- n13 * n24 * n32
			- n14 * n22 * n33
			+ n12 * n24 * n33
			+ n13 * n22 * n34
			- n12 * n23 * n34
		) +
		n42 * (
			n11 * n23 * n34
			- n11 * n24 * n33
			+ n14 * n21 * n33
			- n13 * n21 * n34
			+ n13 * n24 * n31
			- n14 * n23 * n31
		) +
		n43 * (
			n11 * n24 * n32
			- n11 * n22 * n34
			- n14 * n21 * n32
			+ n12 * n21 * n34
			+ n14 * n22 * n31
			- n12 * n24 * n31
		) +
		n44 * (
			n13 * n22 * n31
			- n11 * n23 * n32
			+ n11 * n22 * n33
			+ n13 * n21 * n32
			- n12 * n21 * n33
			+ n12 * n23 * n31
		)
	}

	pub fn make_perspective(
		fovy: f32,
		aspect: f32,
		near: f32,
		far: f32
	) -> Matrix4 {
		let f = 1.0 / (fovy / 2.0).tan();
		let nf = 1.0 / (near - far);
		Matrix4::of([
			f / aspect,
			0.0,
			0.0,
			0.0,
			0.0,
			f,
			0.0,
			0.0,
			0.0,
			0.0,
			far * nf,
			-1.0,
			0.0,
			0.0,
			far * near * nf,
			0.0,
		])
	}
}
