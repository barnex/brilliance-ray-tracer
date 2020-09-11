use super::*;

// A Ray is a half-line with a start point (exclusive),
// extending in direction dir (unit vector).
#[derive(Clone)]
pub struct Ray {
	pub start: Point,
	pub dir: Vector,
}

impl Ray {
	/// Constructs a ray with given starting point and direction.
	/// Both must be finite, and dir must be a unit vector.
	#[inline]
	pub fn new(start: Point, dir: Vector) -> Self {
		debug_assert!(start.is_finite());
		debug_assert!(dir.is_normalized());
		Ray { start, dir }
	}

	/// Point at distance `t` (positive) from the start.
	///
	///     use brilliance::tracer::*;
	///     let r = Ray::new(Point(1.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0));
	///     assert_eq!(r.at(0.0), Point(1.0, 0.0, 0.0));
	///     assert_eq!(r.at(1.0), Point(1.0, 1.0, 0.0));
	///     assert_eq!(r.at(2.0), Point(1.0, 2.0, 0.0));
	///
	#[inline]
	pub fn at(&self, t: f64) -> Point {
		debug_assert!(t >= 0.0);
		self.start + t * self.dir
	}

	/// Checks that the ray is free of NaNs and has an approximately normalized direction.
	/// Intended for use with `debug_assert`.
	pub fn is_valid(&self) -> bool {
		self.start.is_finite() && self.dir.is_normalized()
	}
}
