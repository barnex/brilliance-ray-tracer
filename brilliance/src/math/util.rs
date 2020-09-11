pub use std::f64::consts::PI;

/// One degree in radians.
/// E.g.: `90.0 * DEG` is a right angle.
pub const DEG: f64 = PI / 180.0;

pub const INF: f64 = 1.0 / 0.0;
pub const INF32: f32 = 1.0 / 0.0;

// TODO: min, max on PartialOrd, handling NaN !>, !<
