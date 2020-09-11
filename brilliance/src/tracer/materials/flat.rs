use super::*;

/// Flat is a material with flat shading.
/// I.e., returns its colors as-is, disregarding any lighting, shadows, etc.
/// Such materials emit light (in an indirect way). Suited for large,
/// dimly luminous surfaces like computer screens, the sky, etc.
pub struct Flat<T: Texture> {
	tex: T,
}

impl<T: Texture> Flat<T> {
	pub fn new(tex: T) -> Self {
		Self { tex }
	}
}

impl<T: Texture> Material for Flat<T> {
	fn shade(&self, _: &Scene, _: &Ray, h: &HitCoords, _: &mut Rng, _: u32) -> Color {
		self.tex.color_at(h.tex_coords)
	}
}
