use super::*;

pub fn shade_normal(r: &Ray, n: &Vector) -> Color {
	debug_assert!(r.is_valid());
	debug_assert!(n.is_normalized());

	let v = n.dot(r.dir);
	if v < 0.0 {
		Color::CYAN * (-v as f32) // towards cam
	} else {
		Color::RED * (v as f32) // away from cam
	}
}

pub const TINY: f64 = 1.0 / (1024.0 * 1024.0);
