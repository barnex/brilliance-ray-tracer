use super::*;

pub trait Shape: Bounded + Send + Sync + Sized {
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool;

	fn paint<M: Material>(self, mat: M) -> WithMaterial<Self, M> {
		WithMaterial::new(self, mat)
	}
}
