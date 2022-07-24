mod vector;
mod euler;
mod matrix3;
mod matrix3gpu;
mod matrix4;
mod quaternion;

pub use self::vector::Vector;
pub use self::euler::Euler;
pub use self::matrix3::Matrix3;
pub use self::matrix3gpu::Matrix3GPU;
pub use self::matrix4::Matrix4;
pub use self::quaternion::Quaternion;

pub type Color = Vector<f32, 3>;
pub type Vector3 = Vector<f32, 3>;
