pub use std::f64::consts::PI;
use std::ops::Mul;

/// One degree in radians.
/// E.g.: `90.0 * DEG` is a right angle.
pub const DEG: f64 = PI / 180.0;

pub const INF: f64 = 1.0 / 0.0;
pub const INF32: f32 = 1.0 / 0.0;

// TODO: min, max on PartialOrd, handling NaN !>, !<

/// Return x if > 0, 0 otherwise.
///
///     use brilliance::*;
///     assert_eq!(re(4), 4);
///     assert_eq!(re(0.0), 0.0);
///     assert_eq!(re(-1.0), 0.0);
///
pub fn re<T>(x: T) -> T
where
	T: PartialOrd + Default,
{
	if x < T::default() {
		T::default()
	} else {
		x
	}
}

pub fn sqr<T>(x: T) -> T
where
	T: Mul<T, Output = T> + Copy,
{
	x * x
}
