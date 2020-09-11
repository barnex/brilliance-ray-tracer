use super::*;
use std::fmt;
use std::ops::*;

/// 3x3 matrix intended for linear transformations.
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T> {
	el: [Vec3<T>; 3],
}

impl<T: Copy> Matrix<T> {
	#[inline]
	pub fn new(a: Vec3<T>, b: Vec3<T>, c: Vec3<T>) -> Self {
		Self { el: [a, b, c] }
	}
}

impl<T: Copy> From<[[T; 3]; 3]> for Matrix<T> {
	fn from(arr: [[T; 3]; 3]) -> Self {
		Self {
			el: [arr[0].into(), arr[1].into(), arr[2].into()],
		}
	}
}

impl<T> From<[Vec3<T>; 3]> for Matrix<T>
where
	T: Copy,
{
	fn from(arr: [Vec3<T>; 3]) -> Self {
		Self {
			el: [arr[0].into(), arr[1].into(), arr[2].into()],
		}
	}
}

impl<T> Default for Matrix<T>
where
	T: Default + Copy,
{
	fn default() -> Self {
		Self {
			el: [Vec3::default(), Vec3::default(), Vec3::default()],
		}
	}
}

impl<T> Matrix<T>
where
	T: Default + One + Copy,
{
	pub fn unit() -> Self {
		let o = T::default();
		let i = T::one();
		Matrix::from([[i, o, o], [o, i, o], [o, o, i]])
	}
}

impl<T> Index<usize> for Matrix<T> {
	type Output = Vec3<T>;

	/// Index returns column i as vector.
	#[inline]
	fn index(&self, i: usize) -> &Self::Output {
		&self.el[i]
	}
}

impl<T> IndexMut<usize> for Matrix<T> {
	/// Index returns column i as vector.
	#[inline]
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
		&mut self.el[i]
	}
}

impl<T> Neg for Matrix<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Matrix<T>;
	fn neg(self) -> Matrix<T> {
		Matrix {
			el: [-self[0], -self[1], -self[2]],
		}
	}
}

// TODO: unroll loop, do not require Default
impl<T> Mul<Matrix<T>> for Matrix<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Copy + Default,
{
	type Output = Matrix<T>;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
		let mut c = Matrix::default();
		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					c[i][j] = c[i][j] + rhs[i][k] * self[k][j]
				}
			}
		}
		c
	}
}

impl<T> Mul<Vec3<T>> for Matrix<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
	type Output = Vec3<T>;

	/// Matrix-Vector multiplication.
	fn mul(self, rhs: Vec3<T>) -> Self::Output {
		Vec3(
			self[0][0] * rhs[0] + self[1][0] * rhs[1] + self[2][0] * rhs[2],
			self[0][1] * rhs[0] + self[1][1] * rhs[1] + self[2][1] * rhs[2],
			self[0][2] * rhs[0] + self[1][2] * rhs[1] + self[2][2] * rhs[2],
		)
	}
}

impl<T> Mul<T> for Matrix<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
	type Output = Matrix<T>;

	/// Matrix-scalar multiplication.
	///
	///     use brilliance::*;
	///     let m = Matrix::from([[1,2,3],[4,5,6],[7,8,9]]);
	///     assert_eq!(
	///         m*2,
	///         Matrix::from([[2,4,6],[8,10,12],[14,16,18]]),
	///     );
	fn mul(self, rhs: T) -> Matrix<T> {
		Matrix {
			el: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
		}
	}
}

impl<T> Matrix<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Copy + Recip,
{
	/// Inverse matrix.
	pub fn recip(&self) -> Matrix<T> {
		let a = self[0][0];
		let b = self[1][0];
		let c = self[2][0];
		let d = self[0][1];
		let e = self[1][1];
		let f = self[2][1];
		let g = self[0][2];
		let h = self[1][2];
		let i = self[2][2];

		let a_: T = e * i - f * h;
		let b_: T = f * g - d * i;
		let c_: T = d * h - e * g;
		let inv: Matrix<T> = Self {
			el: [
				Vec3(e * i - f * h, f * g - d * i, d * h - e * g),
				Vec3(c * h - b * i, a * i - c * g, b * g - a * h),
				Vec3(b * f - c * e, c * d - a * f, a * e - b * d),
			],
		};
		let det: T = a * a_ + b * b_ + c * c_;
		inv * (det.recip())
	}
}

impl<T> fmt::Display for Matrix<T>
where
	T: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
	}
}

impl<T> fmt::Debug for Matrix<T>
where
	T: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
	}
}

//#[cfg(test)]
//mod tests {
//	use super::*;
//
//	#[test]
//	fn mul_matrix_matrix() {
//		let theta = 45.0 * DEG;
//		let c = theta.cos();
//		let s = theta.sin();
//		let a = Matrix::from([[c, s, 0.], [-s, c, 0.], [0., 0., 1.]]);
//		//assert_eq!(
//		//	a * a,
//		//	Matrix::from([[0., 1., 0.], [-1., 0., 0.], [0., 0., 1.]])
//		//);
//	}
//
//	#[test]
//	fn mul_matrix_vector() {
//		let theta = 30.0 * DEG;
//		let c = theta.cos();
//		let s = theta.sin();
//
//		let m = Matrix::from([[c, s, 0.], [-s, c, 0.], [0., 0., 1.]]);
//
//		//assert_eq!(m * Vector(1., 0., 0.), Vector(0.866025, 0.500000, 0.000000));
//		//assert_eq!(m * Vector(0., 1., 0.), Vector(-0.50000, 0.866025, 0.000000));
//		//assert_eq!(m * Vector(0., 0., 1.), Vector(0.000000, 0.000000, 1.000000));
//	}
//}

//  func ExampleMatrix_Mul() {
//  	theta := 45 * math.Pi / 180
//  	c := math.Cos(theta)
//  	s := math.Sin(theta)
//  	a := Matrix{{c, s, 0}, {-s, c, 0}, {0, 0, 1}}
//  	fmt.Printf("% 4.1f", a.Mul(&a))
//
//  	//Output:
//  	// [[ 0.0  1.0  0.0] [-1.0  0.0  0.0] [ 0.0  0.0  1.0]]
//  }
//
//  func ExampleMatrix_Mul_2() {
//  	R := Matrix{{0, 1, 0}, {-1, 0, 0}, {0, 0, 1}}
//  	F := Matrix{{-1, 0, 0}, {0, 1, 0}, {0, 0, 1}}
//  	fmt.Printf("% 4.1f\n", R.Mul(&F))
//  	fmt.Printf("% 4.1f\n", F.Mul(&R))
//
//  	//Output:
//  	// [[ 0.0 -1.0  0.0] [-1.0  0.0  0.0] [ 0.0  0.0  1.0]]
//  	// [[ 0.0  1.0  0.0] [ 1.0  0.0  0.0] [ 0.0  0.0  1.0]]
//  }
//
//  func ExampleMatrix_MulVec() {
//  	theta := 30 * math.Pi / 180
//  	c := math.Cos(theta)
//  	s := math.Sin(theta)
//
//  	m := Matrix{{c, s, 0}, {-s, c, 0}, {0, 0, 1}}
//  	fmt.Printf("% 3f\n", m.MulVec(Vec{1, 0, 0}))
//  	fmt.Printf("% 3f\n", m.MulVec(Vec{0, 1, 0}))
//  	fmt.Printf("% 3f\n", m.MulVec(Vec{0, 0, 1}))
//
//  	//Output:
//  	// [ 0.866025  0.500000  0.000000]
//  	// [-0.500000  0.866025  0.000000]
//  	// [ 0.000000  0.000000  1.000000]
//  }
//
//  func ExampleMatrix_Inverse() {
//  	m := Matrix{{1, 2, 3}, {3, -1, 2}, {2, 3, -1}}
//  	inv := m.Inverse()
//  	check := inv.Mul(&m)
//
//  	for i := range check {
//  		for j, v := range check[i] {
//  			if math.Abs(v) < 1e-9 {
//  				check[i][j] = 0
//  			}
//  		}
//  	}
//  	fmt.Printf("% 4.3f", check)
//
//  	//Output:
//  	// [[ 1.000  0.000  0.000] [ 0.000  1.000  0.000] [ 0.000  0.000  1.000]]
//  }
//
