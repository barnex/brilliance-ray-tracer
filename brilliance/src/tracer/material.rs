use super::*;

pub trait Material: Send + Sync {
	/// Shade returns the brightness seen by Ray r which intersects
	/// the scene at the given hit coordinates.
	///
	/// Implementations may call s.LightField recursively,
	/// TODO: recursion depth is limited automatically through the context ctx.
	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color;

	// by default, objects are opaque.
	fn occlude(&self, c: Color, pos: Point) -> Color {
		Color::BLACK
	}
}

pub struct DynMaterial(Box<dyn Material>);

impl DynMaterial {
	pub fn new<M>(mat: M) -> Self
	where
		M: Material + 'static,
	{
		Self(Box::new(mat))
	}
}

impl Material for DynMaterial {
	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
		let inner: &dyn Material = self.0.borrow();
		inner.shade(s, r, h, rng, depth)
	}
}
