use std::{ops::{Mul, Div, Add, Sub, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign}, iter::Sum};

use num_traits::Float;

#[derive(Copy, Clone, Debug)]
pub struct Vector<T: Copy + Clone, const N: usize>(pub [T; N]);

impl<T: Copy + Clone, const N: usize> From<[T; N]> for Vector<T, N> {
	fn from(data: [T; N]) -> Self {
		Vector(data)
	}
}

impl<T: Copy + Clone, const N: usize> Into<[T; N]> for Vector<T, N> {
	fn into(self) -> [T; N] {
		self.0
	}
}

impl<T: Copy + Clone + Default, const N: usize> Default for Vector<T, N> {
	fn default() -> Self {
		Vector([T::default(); N])
	}
}

impl<T: Copy + Clone + Default, const N: usize> Index<usize> for Vector<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl<T: Copy + Clone + Default, const N: usize> IndexMut<usize> for Vector<T, N> {

	fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
		&mut self.0[index]
	}
}

impl<T: Copy + Clone + Add, const N: usize> Add for Vector<T, N> where <T as Add>::Output: Copy + Default {
	type Output = Vector<<T as Add>::Output, N>;

	fn add(self, other: Vector<T, N>) -> Vector<<T as Add>::Output, N> {
		let mut result = [<T as Add>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] + other.0[i];
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + AddAssign, const N: usize> AddAssign for Vector<T, N> {

	fn add_assign(&mut self, other: Vector<T, N>) {
		for i in 0..N {
			self.0[i] += other.0[i];
		}
	}
}

impl<T: Copy + Clone + Sub, const N: usize> Sub for Vector<T, N> where <T as Sub>::Output: Copy + Default {
	type Output = Vector<<T as Sub>::Output, N>;

	fn sub(self, other: Vector<T, N>) -> Vector<<T as Sub>::Output, N> {
		let mut result = [<T as Sub>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] - other.0[i];
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + SubAssign, const N: usize> SubAssign for Vector<T, N> {

	fn sub_assign(&mut self, other: Vector<T, N>) {
		for i in 0..N {
			self.0[i] -= other.0[i];
		}
	}
}

impl<T: Copy + Clone + Mul, const N: usize> Mul for Vector<T, N> where <T as Mul>::Output: Copy + Default {
	type Output = Vector<<T as Mul>::Output, N>;

	fn mul(self, other: Vector<T, N>) -> Vector<<T as Mul>::Output, N> {
		let mut result = [<T as Mul>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] * other.0[i];
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + MulAssign, const N: usize> MulAssign for Vector<T, N> {

	fn mul_assign(&mut self, other: Vector<T, N>) {
		for i in 0..N {
			self.0[i] *= other.0[i];
		}
	}
}

impl<T: Copy + Clone + Mul, const N: usize> Mul<T> for Vector<T, N> where <T as Mul>::Output: Copy + Default {
	type Output = Vector<<T as Mul>::Output, N>;

	fn mul(self, other: T) -> Vector<<T as Mul>::Output, N> {
		let mut result = [<T as Mul>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] * other;
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + MulAssign, const N: usize> MulAssign<T> for Vector<T, N> {

	fn mul_assign(&mut self, other: T) {
		for i in 0..N {
			self.0[i] *= other;
		}
	}
}

impl<T: Copy + Clone + Div, const N: usize> Div for Vector<T, N> where <T as Div>::Output: Copy + Default {
	type Output = Vector<<T as Div>::Output, N>;

	fn div(self, other: Vector<T, N>) -> Vector<<T as Div>::Output, N> {
		let mut result = [<T as Div>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] / other.0[i];
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + DivAssign, const N: usize> DivAssign for Vector<T, N> {

	fn div_assign(&mut self, other: Vector<T, N>) {
		for i in 0..N {
			self.0[i] /= other.0[i];
		}
	}
}

impl<T: Copy + Clone + Div, const N: usize> Div<T> for Vector<T, N> where <T as Div>::Output: Copy + Default {
	type Output = Vector<<T as Div>::Output, N>;

	fn div(self, other: T) -> Vector<<T as Div>::Output, N> {
		let mut result = [<T as Div>::Output::default(); N];
		for i in 0..N {
			result[i] = self.0[i] / other;
		}
		Vector(result)
	}
}

impl<T: Copy + Clone + DivAssign, const N: usize> DivAssign<T> for Vector<T, N> {

	fn div_assign(&mut self, other: T) {
		for i in 0..N {
			self.0[i] /= other;
		}
	}
}

impl<T: Copy + Clone + Mul, const N: usize> Vector<T, N> where <T as Mul>::Output: Copy + Default + Sum {
	pub fn dot(self, other: Vector<T, N>) -> <T as Mul>::Output {
		self.0.iter().zip(other.0).map(|(a, b)| *a * b).sum()
	}
	
}

impl<T: Copy + Clone + Float, const N: usize> Vector<T, N> where <T as Mul>::Output: Copy + Default + Sum + Float {
	pub fn magnitude(self) -> <T as Mul>::Output {
		self.dot(self).sqrt()
	}
	pub fn normalized(self) -> Self {
		self / self.magnitude()
	}
}