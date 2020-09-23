use super::*;

pub struct WithObject<L: Light, O: Object> {
	light: L, // TODO: this should be a trait?
	object: O,
}

impl<L: Light, O: Object> WithObject<L, O> {
	pub fn new(light: L, object: O) -> Self {
		Self { light, object }
	}
}

impl<L: Light, O: Object> Bounded for WithObject<L, O> {
	fn bounds(&self) -> BoundingBox {
		self.object.bounds()
	}
}

impl<L: Light, O: Object> Object for WithObject<L, O> {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		self.object.intersect(r, h)
	}
}

impl<L: Light, O: Object> Light for WithObject<L, O> {
	fn sample(&self, rng: &mut Rng, target: Point) -> (Point, Color) {
		self.light.sample(rng, target)
	}
}
