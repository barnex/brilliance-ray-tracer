use super::*;

pub trait Light: Object {
	// Return a random point on the light's surface,
	// and the intensity at given target position.
	fn sample(&self, rng: &mut Rng, target: Point) -> (Point, Color);
}

pub struct DynLight(pub Box<dyn Light>);

impl DynLight {
	pub fn new<L>(light: L) -> Self
	where
		L: Light + 'static,
	{
		Self(Box::new(light))
	}

	fn inner(&self) -> &dyn Light {
		let inner: &dyn Light = self.0.borrow();
		inner
	}
}

impl Bounded for DynLight {
	fn bounds(&self) -> BoundingBox {
		self.inner().bounds()
	}
}

impl Object for DynLight {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		self.inner().intersect(r, h)
	}
}

impl Light for DynLight {
	fn sample(&self, rng: &mut Rng, target: Point) -> (Point, Color) {
		self.inner().sample(rng, target)
	}
}
