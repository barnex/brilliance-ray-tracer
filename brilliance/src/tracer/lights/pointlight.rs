use super::*;

pub struct PointLight {
	pos: Point,
	power: Color,
}

impl PointLight {
	pub fn new(pos: Point, power: Color) -> Self {
		Self { pos, power }
	}
}

impl Light for PointLight {
	fn object(&self) -> Option<DynObj> {
		None
	}

	fn sample(&self, _: &mut Rng, target: Point) -> (Point, Color) {
		(self.pos, self.power * (((1. / (4. * PI)) / ((target - self.pos).len2())) as f32))
	}
}
