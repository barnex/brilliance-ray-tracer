use super::*;

pub struct PointLight {
	pos: Point,
	power: Color,
}

pub fn point_light(pos: Point, power: Color) -> DynLight {
	DynLight::new(PointLight::new(pos, power))
}

impl PointLight {
	pub fn new(pos: Point, power: Color) -> Self {
		Self { pos, power }
	}
}

impl Bounded for PointLight {
	fn bounds(&self) -> BoundingBox {
		BoundingBox::empty(Pointf::default())
	}
}

impl Object for PointLight {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		// never intersects
	}
}

impl Light for PointLight {
	fn sample(&self, _: &mut Rng, target: Point) -> (Point, Color) {
		//(self.pos, self.power * (((1. / (4. * PI)) / ((target - self.pos).len2())) as f32))
		(self.pos, self.power) // planetarium hack
	}
}
