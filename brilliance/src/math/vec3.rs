use std::cmp::{PartialEq, PartialOrd};
use std::convert::From;
use std::fmt;
use std::ops::*;

/// A generic 3-component vector, in the mathematical sense.
/// I.e., the displacement between two points in 3D space.
///
/// Most commonly used is `Vec3<f64>`, aliased as type `Vector`.
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3<T> {
	pub el: [T; 3],
}

/// Shorthand constructor with tuple-style syntax.
///
///     use brilliance::math::*;
///     let v = Vec3(1, 2, 3);
///
#[inline]
#[allow(non_snake_case)]
pub fn Vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
	Vec3::new(x, y, z)
}

impl<T> Vec3<T> {
	#[inline]
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { el: [x, y, z] }
	}

	pub fn iter(&self) -> std::slice::Iter<T> {
		self.el.iter()
	}
}

impl<T> Vec3<T>
where
	T: PartialOrd,
{
	pub fn argmax(&self) -> usize {
		let mut arg = 0;
		for i in 1..2 {
			if self[i] > self[arg] {
				arg = i
			}
		}
		arg
	}
}
impl<T> Default for Vec3<T>
where
	T: Default,
{
	/// All components set to default.
	///
	///     use brilliance::math::*;
	///     let v: Vec3<i32> = Vec3::default();
	///     assert_eq!(v, Vec3(0, 0, 0))
	///
	#[inline]
	fn default() -> Self {
		Self::new(T::default(), T::default(), T::default())
	}
}

impl<T> From<[T; 3]> for Vec3<T> {
	fn from(arr: [T; 3]) -> Self {
		Self { el: arr }
	}
}

//impl<T> Vec3<T>where T:Add+Mul{
//
//	#[inline]
//	pub fn madd()
//}

impl<T> Add for Vec3<T>
where
	T: Add<T, Output = T> + Copy,
{
	type Output = Self;

	/// Element-wise sum.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 3);
	///     let b = Vec3(4, 5, 6);
	///     assert_eq!(a+b, Vec3(5, 7, 9));
	///
	#[inline]
	fn add(self, rhs: Vec3<T>) -> Self::Output {
		Self {
			el: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
		}
	}
}

impl<T> AddAssign for Vec3<T>
where
	T: AddAssign + Copy,
{
	/// Element-wise sum.
	///
	///     use brilliance::math::*;
	///     let mut a = Vec3(1, 2, 3);
	///     a += Vec3(4, 5, 6);
	///     assert_eq!(a, Vec3(5, 7, 9));
	///
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self[0] += rhs[0];
		self[1] += rhs[1];
		self[2] += rhs[2];
	}
}

impl<T> Index<usize> for Vec3<T> {
	type Output = T;

	/// Index component 0, 1, or 2 (X, Y, or Z)
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 3);
	///     assert_eq!(a[0], 1);
	///     assert_eq!(a[1], 2);
	///     assert_eq!(a[2], 3);
	///
	#[inline]
	fn index(&self, idx: usize) -> &Self::Output {
		&self.el[idx]
	}
}

impl<T> IndexMut<usize> for Vec3<T> {
	/// Index component 0, 1, or 2 (X, Y, or Z)
	///
	///     use brilliance::math::*;
	///     let mut a = Vec3(0, 0, 0);
	///     a[0] = 1;
	///     a[1] = 2;
	///     a[2] = 3;
	///     assert_eq!(a, Vec3(1, 2, 3));
	///
	#[inline]
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut self.el[idx]
	}
}

impl<T> Div<T> for Vec3<T>
where
	T: Div<T, Output = T> + Copy,
{
	type Output = Self;

	/// Scalar division.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1.0, 2.0, 3.0);
	///     assert_eq!(a/2.0, Vec3(0.5, 1.0, 1.5));
	///
	#[inline]
	fn div(self, rhs: T) -> Self::Output {
		Self {
			el: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
		}
	}
}

impl<T> Mul<T> for Vec3<T>
where
	T: Mul<T, Output = T> + Copy,
{
	type Output = Self;

	/// Scalar multiplication.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 3);
	///     assert_eq!(a*2, Vec3(2, 4, 6));
	///
	#[inline]
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			el: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
		}
	}
}

impl<T> MulAssign<T> for Vec3<T>
where
	T: MulAssign + Copy,
{
	/// Scalar multiplication.
	///
	///     use brilliance::math::*;
	///     let mut a = Vec3(1, 2, 3);
	///     a *= 2;
	///     assert_eq!(a, Vec3(2, 4, 6));
	///
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self[0] *= rhs;
		self[1] *= rhs;
		self[2] *= rhs;
	}
}

impl Mul<Vec3<f64>> for f64 {
	type Output = Vec3<f64>;

	/// Left multiplication.
	/// Identical to right multiplication, but allowing for a more natural syntax.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1.0, 2.0, 3.0);
	///     assert_eq!(2.0*a, Vec3(2.0, 4.0, 6.0));
	///
	#[inline]
	fn mul(self, rhs: Vec3<f64>) -> Self::Output {
		rhs.mul(self)
	}
}

impl Mul<Vec3<f32>> for f32 {
	type Output = Vec3<f32>;

	/// Left multiplication.
	/// Identical to right multiplication, but allowing for a more natural syntax.
	///
	///     use brilliance::math::*;
	///     let a = Vec3::<f32>(1.0, 2.0, 3.0);
	///     assert_eq!(2.0*a, Vec3::<f32>(2.0, 4.0, 6.0));
	///
	#[inline]
	fn mul(self, rhs: Vec3<f32>) -> Self::Output {
		rhs.mul(self)
	}
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			el: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
		}
	}
}

impl<T> Neg for Vec3<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Self;

	/// Negative.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 3);
	///     assert_eq!(-a, Vec3(-1, -2, -3));
	///
	#[inline]
	fn neg(self) -> Self::Output {
		Self {
			el: [-self[0], -self[1], -self[2]],
		}
	}
}

impl<T> Sub for Vec3<T>
where
	T: Sub<T, Output = T> + Copy,
{
	type Output = Self;

	/// Element-wise subtraction.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(100, 200, 300);
	///     let b = Vec3(1, 2, 3);
	///     assert_eq!(a-b, Vec3(99, 198, 297));
	///
	#[inline]
	fn sub(self, rhs: Vec3<T>) -> Self::Output {
		Self {
			el: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
		}
	}
}

impl<T> SubAssign for Vec3<T>
where
	T: SubAssign + Copy,
{
	/// Element-wise subtraction.
	///
	///     use brilliance::math::*;
	///     let mut a = Vec3(100, 200, 300);
	///     a -= Vec3(1, 2, 3);
	///     assert_eq!(a, Vec3(99, 198, 297));
	///
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self[0] -= rhs[0];
		self[1] -= rhs[1];
		self[2] -= rhs[2];
	}
}

//impl<T> Neg for Vec3<T>
//where
//	T: Neg,
//{
//	type Output = Vec3<T>;
//	#[inline]
//	fn neg(self) -> Self {
//		Self::new(-self[0], -self[1], -self[2])
//	}
//}

impl From<Vectorf> for Vector {
	/// Losslessly convert from a `Vec3<f32>` to a `Vec3<f64>`.
	///
	///     use brilliance::math::*;
	///     let a = Vectorf(1.0, 2.0, 3.0);
	///     let b: Vector = a.into();
	///     assert_eq!(b, Vector(1.0, 2.0, 3.0));
	///
	fn from(v: Vectorf) -> Vector {
		Vector::new(v[0] as f64, v[1] as f64, v[2] as f64)
	}
}

impl<T> fmt::Display for Vec3<T>
where
	T: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
	}
}

impl<T> fmt::Debug for Vec3<T>
where
	T: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
	}
}

impl<T> Vec3<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
	/// Dot (inner) product.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 0);
	///     let b = Vec3(0, 2, 1);
	///     assert_eq!(a.dot(b), 4);
	///
	#[inline]
	pub fn dot(self, rhs: Vec3<T>) -> T {
		self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
	}

	/// Cross product (right-handed).
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 0, 0);
	///     let b = Vec3(0, 1, 0);
	///     assert_eq!(a.cross(b), Vec3(0, 0, 1));
	///
	#[inline]
	pub fn cross(self, rhs: Self) -> Self {
		Self {
			el: [
				self[1] * rhs[2] - self[2] * rhs[1],
				self[2] * rhs[0] - self[0] * rhs[2],
				self[0] * rhs[1] - self[1] * rhs[0],
			],
		}
	}

	/// Element-wise product.
	///
	///     use brilliance::math::*;
	///     let a = Vec3(1, 2, 3);
	///     let b = Vec3(4, 5, 6);
	///     assert_eq!(a.mul3(b), Vec3(4, 10, 18));
	///
	#[inline]
	pub fn mul3(self, rhs: Self) -> Self {
		Self {
			el: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
		}
	}
}

/// Vector is shorthand for `Vec3<f64>`,
/// because it is so commonly used.
pub type Vector = Vec3<f64>;

/// Shorthand constructor with tuple-style syntax.
///
///      use brilliance::math::*;
///      let v = Vector(1.0, 2.0, 3.0);
///
#[inline]
#[allow(non_snake_case)]
pub fn Vector(x: f64, y: f64, z: f64) -> Vector {
	Vector::new(x, y, z)
}

impl Vector {
	/// Length (norm).
	///
	///     use brilliance::math::*;
	///     let v = Vector(0.0, 3.0, 4.0);
	///     assert_eq!(v.len(), 5.0);
	///
	#[inline]
	pub fn len(self) -> f64 {
		self.len2().sqrt()
	}

	/// Length squared (norm squared).
	///
	///     use brilliance::math::*;
	///     let v = Vector(0.0, 3.0, 4.0);
	///     assert_eq!(v.len2(), 25.0);
	///
	#[inline]
	pub fn len2(self) -> f64 {
		self.dot(self)
	}

	/// Returns a vector with the same direction but unit length.
	///
	///     use brilliance::math::*;
	///     let v = Vector(99.9, 0.0, 0.0);
	///     assert_eq!(v.normalized(), Vector(1.0, 0.0, 0.0));
	///
	#[inline]
	#[must_use]
	pub fn normalized(self) -> Self {
		self * (1. / self.len())
	}

	/// Re-scale the vector to unit length.
	///
	///     use brilliance::math::*;
	///     let mut v = Vector(99.9, 0.0, 0.0);
	///     v.normalize();
	///     assert_eq!(v, Vector(1.0, 0.0, 0.0));
	///
	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	/// Test if the vector has approximately unit length.
	/// Intended for use with debug_assert! where a unit vector is expected.
	///
	///     use brilliance::math::*;
	///     assert!(!Vector(9.9, 0.0, 0.0).is_normalized());
	///     assert!( Vector(1.0, 0.0, 0.0).is_normalized());
	///
	pub fn is_normalized(&self) -> bool {
		(self.len() - 1.0).abs() < 1e-6
	}

	/// Test that all components are neither NaN nor infinite.
	/// Intended for use with debug_assert!
	///
	///     use brilliance::math::*;
	///     assert!(!Vector(0.0/0.0, 0.0, 0.0).is_finite());
	///     assert!(!Vector(1.0/0.0, 0.0, 0.0).is_finite());
	///     assert!( Vector(1.0, 2.0, 3.0).is_finite());
	///
	pub fn is_finite(&self) -> bool {
		self[0].is_finite() && self[1].is_finite() && self[2].is_finite()
	}

	/// The zero vector.
	pub const ZERO: Vector = Self { el: [0.0, 0.0, 0.0] };
	/// Unit vector along X.
	pub const EX: Vector = Self { el: [1.0, 0.0, 0.0] };
	/// Unit vector along Y.
	pub const EY: Vector = Self { el: [0.0, 1.0, 0.0] };
	/// Unit vector along Z.
	pub const EZ: Vector = Self { el: [0.0, 0.0, 1.0] };

	/// Minimum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vector(1.0, 2.0, 3.0);
	///     assert_eq!(v.min(), 1.0);
	///
	#[inline]
	pub fn min(self) -> f64 {
		f64::min(f64::min(self[0], self[1]), self[2])
	}

	/// Maximum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vector(1.0, 2.0, 3.0);
	///     assert_eq!(v.max(), 3.0);
	///
	#[inline]
	pub fn max(self) -> f64 {
		f64::max(f64::max(self[0], self[1]), self[2])
	}
}

/// Vectorf is shorthand for `Vec3<f32>`,
/// because it is so commonly used.
pub type Vectorf = Vec3<f32>;

/// Shorthand constructor with tuple-style syntax.
///
///      use brilliance::math::*;
///      let v = Vectorf(1.0, 2.0, 3.0);
///
#[inline]
#[allow(non_snake_case)]
pub fn Vectorf(x: f32, y: f32, z: f32) -> Vectorf {
	Vectorf::new(x, y, z)
}

impl From<Vector> for Vectorf {
	#[inline]
	fn from(v: Vector) -> Self {
		Self::new(v[0] as f32, v[1] as f32, v[2] as f32)
	}
}

impl Vectorf {
	/// Length (norm).
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(0.0, 3.0, 4.0);
	///     assert_eq!(v.len(), 5.0);
	///
	#[inline]
	pub fn len(self) -> f32 {
		self.dot(self).sqrt()
	}

	/// Component-wise reciprocal.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 4.0);
	///     assert_eq!(v.inv(), Vectorf(1.0, 0.5, 0.25));
	///
	#[inline]
	pub fn inv(self) -> Self {
		Self::new(1.0 / self[0], 1.0 / self[1], 1.0 / self[2])
	}

	/// Component-wise minimum.
	///
	///     use brilliance::math::*;
	///     let a = Vectorf(1.0, 2.0, 3.0);
	///     let b = Vectorf(2.0, 1.0, 0.0);
	///     assert_eq!(Vectorf::min(a, b), Vectorf(1.0, 1.0, 0.0));
	///
	#[inline]
	pub fn min(a: Self, b: Self) -> Self {
		Self::new(f32::min(a[0], b[0]), f32::min(a[1], b[1]), f32::min(a[2], b[2]))
	}

	/// Component-wise maximum.
	///
	///     use brilliance::math::*;
	///     let a = Vectorf(1.0, 2.0, 3.0);
	///     let b = Vectorf(2.0, 1.0, 0.0);
	///     assert_eq!(Vectorf::max(a, b), Vectorf(2.0, 2.0, 3.0));
	///
	#[inline]
	pub fn max(a: Self, b: Self) -> Self {
		Self::new(f32::max(a[0], b[0]), f32::max(a[1], b[1]), f32::max(a[2], b[2]))
	}

	/// Returns a vector with the same direction but unit length.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(99.9, 0.0, 0.0);
	///     assert_eq!(v.normalized(), Vectorf(1.0, 0.0, 0.0));
	///
	#[inline]
	#[must_use]
	pub fn normalized(self) -> Self {
		self * (1. / self.len())
	}

	/// Re-scale the vector to unit length.
	///
	///     use brilliance::math::*;
	///     let mut v = Vectorf(99.9, 0.0, 0.0);
	///     v.normalize();
	///     assert_eq!(v, Vectorf(1.0, 0.0, 0.0));
	///
	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	/// Test if the vector has approximately unit length.
	/// Intended for use with debug_assert! where a unit vector is expected.
	///
	///     use brilliance::math::*;
	///     assert!(!Vectorf(9.9, 0.0, 0.0).is_normalized());
	///     assert!( Vectorf(1.0, 0.0, 0.0).is_normalized());
	///
	pub fn is_normalized(&self) -> bool {
		(self.len() - 1.0).abs() < 1e-6
	}

	/// Test that all components are neither NaN nor infinite.
	/// Intended for use with debug_assert!
	///
	///     use brilliance::math::*;
	///     assert!(!Vectorf(0.0/0.0, 0.0, 0.0).is_finite());
	///     assert!(!Vectorf(1.0/0.0, 0.0, 0.0).is_finite());
	///     assert!( Vectorf(1.0, 2.0, 3.0).is_finite());
	///
	pub fn is_finite(&self) -> bool {
		self[0].is_finite() && self[1].is_finite() && self[2].is_finite()
	}

	/// The zero vector.
	pub const ZERO: Vectorf = Self { el: [0.0, 0.0, 0.0] };
	/// Unit vector along X.
	pub const EX: Vectorf = Self { el: [1.0, 0.0, 0.0] };
	/// Unit vector along Y.
	pub const EY: Vectorf = Self { el: [0.0, 1.0, 0.0] };
	/// Unit vector along Z.
	pub const EZ: Vectorf = Self { el: [0.0, 0.0, 1.0] };

	/// Minimum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 3.0);
	///     assert_eq!(v.min3(), 1.0);
	///
	#[inline]
	pub fn min3(self) -> f32 {
		f32::min(f32::min(self[0], self[1]), self[2])
	}

	/// Maximum of all components.
	///
	///     use brilliance::math::*;
	///     let v = Vectorf(1.0, 2.0, 3.0);
	///     assert_eq!(v.max3(), 3.0);
	///
	#[inline]
	pub fn max3(self) -> f32 {
		f32::max(f32::max(self[0], self[1]), self[2])
	}
}
