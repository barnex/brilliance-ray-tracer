use super::*;

pub struct Blend<A: Material, B: Material> {
	mat_a: A,
	weight_a: f32,
	mat_b: B,
	weight_b: f32,
}

impl<A: Material, B: Material> Material for Blend<A, B> {
	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
		// TODO: don't branch on integration if weight < some limit?
		self.weight_a * self.mat_a.shade(s, r, h, rng, depth) + self.weight_b * self.mat_b.shade(s, r, h, rng, depth)
	}
}

pub fn shiny<T: Texture>(base: T, shine: f32) -> DynMaterial {
	DynMaterial::new(Blend {
		mat_a: Matte::new(base),
		weight_a: 1.0 - shine,
		mat_b: Reflective(Color::WHITE),
		weight_b: shine,
	})
}
