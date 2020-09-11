use super::*;

pub trait Light: Send + Sync {
	// Object is rendered when a viewing ray sees the light directly.
	// E.g. for a spherical yellow light source, this object is a
	// bright yellow sphere.
	//
	// Care must be taken that the properties of this object
	// (size, surface brightness) exactly match the light intensity
	// obtained by calling Sample(). I.e. The amount of light
	// that a scene receives from this object via unidirectional path
	// tracing must be exactly equal to the amout of light
	// received from Sample() using bidirectional path tracing.
	fn object(&self) -> Option<DynObj>;

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
}

impl Light for DynLight {
	fn object(&self) -> Option<DynObj> {
		let inner: &dyn Light = self.0.borrow();
		inner.object()
	}

	fn sample(&self, rng: &mut Rng, target: Point) -> (Point, Color) {
		let inner: &dyn Light = self.0.borrow();
		inner.sample(rng, target)
	}
}
