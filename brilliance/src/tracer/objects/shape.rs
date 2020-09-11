use super::*;

pub trait Shape: Bounded + Send + Sync + Sized {
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool;

	fn paint(self, mat: DynMaterial) -> WithMaterial<Self, DynMaterial> {
		WithMaterial::new(self, mat)
	}
}
