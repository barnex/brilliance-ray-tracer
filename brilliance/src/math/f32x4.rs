use core::arch::x86_64::*;
use std::convert::From;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::*;

/// 4 packed `f32`s with SSE accelerated arithmetic operations.
#[derive(Copy, Clone)]
pub struct F32x4 {
	mm: __m128,
}

/// Shorthand for `F32x4::new`.
#[inline]
#[allow(non_snake_case)]
pub fn F32x4(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
	F32x4::new(a, b, c, d)
}

impl From<[f32; 4]> for F32x4 {
	#[inline]
	fn from(v: [f32; 4]) -> Self {
		Self {
			mm: unsafe { mem::transmute(v) },
		}
	}
}

impl From<__m128> for F32x4 {
	#[inline]
	fn from(mm: __m128) -> Self {
		Self { mm }
	}
}

impl Into<[f32; 4]> for F32x4 {
	#[inline]
	fn into(self) -> [f32; 4] {
		unsafe { mem::transmute(self.mm) }
	}
}

// TODO: rename broadcast, as from is ambiguous.
impl From<f32> for F32x4 {
	/// Constructs a vector with all components set to the same number.
	///
	///     use brilliance::math::*;
	///     let v = F32x4::from(9.9);
	///     assert_eq!(v, F32x4(9.9, 9.9, 9.9, 9.9));
	///
	#[inline]
	fn from(v: f32) -> Self {
		unsafe { _mm_set1_ps(v).into() }
	}
}

impl Display for F32x4 {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		self.fmt(f)
	}
}

impl Debug for F32x4 {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		self.fmt(f)
	}
}

impl F32x4 {
	/// Construct a 4-component vector.
	///
	///     use brilliance::math::*;
	///     let v = F32x4::new(1.0, 2.0, 3.0, 4.0);
	///
	#[inline]
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		unsafe { _mm_set_ps(w, z, y, x) }.into()
	}

	#[inline]
	pub fn array(self) -> [f32; 4] {
		unsafe { mem::transmute(self) }
	}

	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		let a = self.array();
		write!(f, "<{}, {}, {}, {}>", a[0], a[1], a[2], a[3])
	}

	/// horizontal minimum.
	///
	///     use brilliance::math::*;
	///     assert_eq!(F32x4(1.0, 2.0, 3.0, 4.0).hmin(), 1.0);
	///     assert_eq!(F32x4(-1.0, -2.0, -3.0, -4.0).hmin(), -4.0);
	///     //assert_eq!(F32x4(-1.0, 0.0/0.0, -3.0, -4.0).hmin(), 0.0/0.0);
	///
	pub fn hmin(self) -> f32 {
		let v = self.array();
		f32::min(f32::min(v[0], v[1]), f32::min(v[2], v[3]))
	}

	/// horizontal maximum.
	///
	///     use brilliance::math::*;
	///     assert_eq!(F32x4(1.0, 2.0, 3.0, 4.0).hmax(), 4.0);
	///     assert_eq!(F32x4(-1.0, -2.0, -3.0, -4.0).hmax(), -1.0);
	///     //assert_eq!(F32x4(-1.0, 0.0/0.0, -3.0, -4.0).hmin(), 0.0/0.0);
	///
	pub fn hmax(self) -> f32 {
		let v = self.array();
		f32::max(f32::max(v[0], v[1]), f32::max(v[2], v[3]))
	}

	/// Component-wise minimum.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
	///     let b = F32x4(2.0, 1.0, 0.0, 5.0);
	///     assert_eq!(F32x4::min(a, b), F32x4(1.0, 1.0, 0.0, 4.0));
	///
	#[inline]
	pub fn min(a: Self, b: Self) -> Self {
		unsafe { _mm_min_ps(a.mm, b.mm) }.into()
	}

	/// Component-wise maximum.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
	///     let b = F32x4(2.0, 1.0, 0.0, 5.0);
	///     assert_eq!(F32x4::max(a, b), F32x4(2.0, 2.0, 3.0, 5.0));
	///
	#[inline]
	pub fn max(a: Self, b: Self) -> Self {
		unsafe { _mm_max_ps(a.mm, b.mm) }.into()
	}

	/// Approximate reciprocal
	///
	///     use brilliance::math::*;
	///     let a = F32x4(0.5, 1.0, 2.0, 4.0);
	///     assert_eq!(a.approx_recip(), F32x4(2.0, 1.0, 0.5, 0.25));
	///
	#[inline]
	pub fn approx_recip(self) -> Self {
		// TODO: use special instruction
		Self::from(1.0) / self
	}

	#[inline]
	pub fn index(self, i: usize) -> f32 {
		self.array()[i]
	}
}
/*
/// Element-wise compare
///
///     use brilliance::math::*;
///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
///     let b = F32x4(1.0, 0.0, 4.0, 2.0);
///     assert_eq!(a.lt(b), [0, 0, -1, 0]);
///
pub fn lt(self, rhs: Self) -> [i32; 4] {
	unsafe { mem::transmute(_mm_cmp_ps(self.mm, rhs.mm, _CMP_LT_OQ)) }
}


/// Element-wise compare
///
///     use brilliance::math::*;
///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
///     let b = F32x4(1.0, 0.0, 4.0, 2.0);
///     assert_eq!(a.gt(b), [0, -1, 0, -1]);
///
pub fn gt(self, rhs: Self) -> [i32; 4] {
	unsafe { mem::transmute(_mm_cmp_ps(self.mm, rhs.mm, _CMP_GT_OQ)) }
}
*/

/*
#[inline]
pub fn fmadd(self, t: f64, b: Vec64) -> Self {
	_mm_fmadd()
	//unsafe { mem::transmute(_mm256_fmadd_pd(b.mm(), _mm256_broadcast_sd(&t), self.mm())) }
}
*/

//impl Index<usize> for F32x4 {
//	type Output = f32;
//
//	#[inline]
//	fn index(&self, i: usize) -> &f32 {
//		let arr = self.array();
//		&arr[i]
//	}
//}

impl Add for F32x4 {
	type Output = Self;

	/// Element-wise sum.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
	///     let b = F32x4(5.0, 6.0, 7.0, 8.0);
	///     assert_eq!(a+b, F32x4(6.0, 8.0, 10.0, 12.0));
	///
	#[inline]
	fn add(self, rhs: Self) -> Self {
		unsafe { _mm_add_ps(self.mm, rhs.mm) }.into()
	}
}

impl AddAssign for F32x4 {
	/// Element-wise sum.
	///
	///     use brilliance::math::*;
	///     let mut v = F32x4(1.0, 2.0, 3.0, 4.0);
	///     v += F32x4(5.0, 6.0, 7.0, 8.0);
	///     assert_eq!(v, F32x4(6.0, 8.0, 10.0, 12.0));
	///
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Mul for F32x4 {
	type Output = Self;

	/// Element-wise product.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
	///     let b = F32x4(5.0, 6.0, 7.0, 8.0);
	///     assert_eq!(a*b, F32x4(5.0, 12.0, 21.0, 32.0));
	///
	#[inline]
	fn mul(self, rhs: Self) -> Self {
		unsafe { _mm_mul_ps(self.mm, rhs.mm) }.into()
	}
}

impl MulAssign for F32x4 {
	/// Element-wise product.
	///
	///     use brilliance::math::*;
	///     let mut v = F32x4(1.0, 2.0, 3.0, 4.0);
	///     v *= F32x4(5.0, 6.0, 7.0, 8.0);
	///     assert_eq!(v, F32x4(5.0, 12.0, 21.0, 32.0));
	///
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl Div for F32x4 {
	type Output = Self;

	/// Element-wise division.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(1.0, 2.0, 3.0, 4.0);
	///     let b = F32x4(1.0, 2.0, 4.0, 8.0);
	///     assert_eq!(a/b, F32x4(1.0, 1.0, 0.75, 0.5));
	///
	#[inline]
	fn div(self, rhs: Self) -> Self {
		unsafe { _mm_div_ps(self.mm, rhs.mm) }.into()
	}
}

impl DivAssign for F32x4 {
	/// Element-wise division.
	///
	///     use brilliance::math::*;
	///     let mut v = F32x4(1.0, 2.0, 3.0, 4.0);
	///     v /= F32x4(1.0, 2.0, 4.0, 8.0);
	///     assert_eq!(v, F32x4(1.0, 1.0, 0.75, 0.5));
	///
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}

impl PartialEq for F32x4 {
	fn eq(&self, rhs: &Self) -> bool {
		self.array() == rhs.array()
	}
}

impl Sub for F32x4 {
	type Output = Self;

	/// Element-wise subtraction.
	///
	///     use brilliance::math::*;
	///     let a = F32x4(100.0, 200.0, 300.0, 400.0);
	///     let b = F32x4(1.0, 2.0, 3.0, 4.0);
	///     assert_eq!(a-b, F32x4(99.0, 198.0, 297.0, 396.0));
	///
	#[inline]
	fn sub(self, rhs: F32x4) -> Self {
		unsafe { _mm_sub_ps(self.mm, rhs.mm) }.into()
	}
}

impl SubAssign for F32x4 {
	/// Element-wise subtraction.
	///
	///     use brilliance::math::*;
	///     let mut v = F32x4(100.0, 200.0, 300.0, 400.0);
	///     v -= F32x4(1.0, 2.0, 3.0, 4.0);
	///     assert_eq!(v, F32x4(99.0, 198.0, 297.0, 396.0));
	///
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Neg for F32x4 {
	type Output = Self;

	/// Negative of all elements.
	///
	///     use brilliance::math::*;
	///     let mut v = F32x4(1.0, -2.0, 3.0, -4.0);
	///     assert_eq!(-v, F32x4(-1.0, 2.0, -3.0, 4.0));
	///
	#[inline]
	fn neg(self) -> Self::Output {
		unsafe { _mm_xor_ps(self.mm, _mm_set1_ps(-0.0f32)) }.into()
	}
}
