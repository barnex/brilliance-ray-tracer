//use super::internal::*;
//use super::*;

pub struct Specular {
	exponent: f32,
}

impl Specular {
	pub fn new(exponent: f32) -> Self {
		Self { exponent }
	}
}

//impl Material for Specular {
//	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
//
//	}
//}
