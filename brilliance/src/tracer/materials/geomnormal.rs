use super::internal;
use super::*;

/// GeomNormal is like ShadingNormal but reveals the geometric normal.
pub struct GeomNormal();

impl GeomNormal {
	pub fn new() -> DynMaterial {
		DynMaterial::new(Self())
	}
}

impl Material for GeomNormal {
	fn shade(&self, _: &Scene, r: &Ray, h: &HitCoords, _: &mut Rng, _depth: u32) -> Color {
		internal::shade_normal(r, &h.geom_normal())
	}
}
