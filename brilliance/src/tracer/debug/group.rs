use super::*;

// TODO: remove

/// An object composed of multiple smaller objects.
/// Intersection is not accellerated.
///
/// Useful for debugging.
pub struct Group<O: Object>(pub Vec<O>);

impl<O: Object> Group<O> {
	pub fn new(children: Vec<O>) -> Self {
		Self(children)
	}
}

impl<O: Object> Object for Group<O> {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		for obj in &self.0 {
			obj.intersect(r, h)
		}
	}
}

impl<B: Object + Bounded> Bounded for Group<B> {
	fn bounds(&self) -> BoundingBox {
		bounds_of(&self.0)
	}
}
