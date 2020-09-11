use super::*;

pub struct WithMaterial<S: Shape, M: Material> {
	shape: S,
	material: M,
}

impl<S: Shape, M: Material> WithMaterial<S, M> {
	pub fn new(shape: S, material: M) -> Self {
		Self { shape, material }
	}
}

impl<S: Shape, M: Material> Object for WithMaterial<S, M> {
	#[inline]
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		if self.shape.intersect_coords(r, &mut h.coords) {
			h.material = &self.material
		}
	}
}

impl<S: Shape, M: Material> Bounded for WithMaterial<S, M> {
	fn bounds(&self) -> BoundingBox {
		self.shape.bounds()
	}
}
