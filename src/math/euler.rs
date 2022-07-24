use super::{vector::Vector, quaternion::Quaternion};

pub type Euler = Vector<f32, 3>;

impl Euler {
	pub fn from_quaternion(
		q: Quaternion,
	) -> Euler {
		// Assume XYZ order
		let q0 = q[0];
		let q1 = q[1];
		let q2 = q[2];
		let q3 = q[3];

		let q0q0 = q0 * q0;
		let q0q1 = q0 * q1;
		let q0q2 = q0 * q2;
		let q0q3 = q0 * q3;
		let q1q1 = q1 * q1;
		let q1q2 = q1 * q2;
		let q1q3 = q1 * q3;
		let q2q2 = q2 * q2;
		let q2q3 = q2 * q3;
		let q3q3 = q3 * q3;

		let roll = (2.0 * (q2q3 + q0q1)).atan2(q0q0 - q1q1 - q2q2 + q3q3);
		let pitch = (2.0 * (q0q2 - q1q3)).asin();
		let yaw = (2.0 * (q1q2 + q0q3)).atan2(q0q0 + q1q1 - q2q2 - q3q3);

		Euler::from([roll, pitch, yaw])
	}
}
