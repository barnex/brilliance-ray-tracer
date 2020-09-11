use super::*;

/// A point in 3D space.
/// The type is idential to a 3D vector,
/// but has it's own name for documentation/redability purposes.
pub type Point = Vec3<f64>;

/// Shorthand constructor with tuple-style syntax.
///
///      use brilliance::math::*;
///      let p = Point(1.0, 2.0, 3.0);
///
#[inline]
#[allow(non_snake_case)]
pub fn Point(x: f64, y: f64, z: f64) -> Point {
	Point::new(x, y, z)
}

/// A point in 3D space.
/// The type is idential to a 3D vector,
/// but has it's own name for documentation/redability purposes.
pub type Pointf = Vec3<f32>;

/// Shorthand constructor with tuple-style syntax.
///
///      use brilliance::math::*;
///      let p = Point(1.0, 2.0, 3.0);
///
#[inline]
#[allow(non_snake_case)]
pub fn Pointf(x: f32, y: f32, z: f32) -> Pointf {
	Pointf::new(x, y, z)
}
