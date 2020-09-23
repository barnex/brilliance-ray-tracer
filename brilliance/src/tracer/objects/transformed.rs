use super::*;

pub struct TransformedObj<O: Object> {
	orig: O,
	scale: f64,
	transl: Point,
}

impl<O: Object> TransformedObj<O> {
	pub fn new(orig: O, scale: f64, transl: Point) -> Self {
		Self {
			orig,
			scale: 1.0 / scale,
			transl: -transl,
		}
	}
}

impl<O: Object> Object for TransformedObj<O> {
	#[inline]
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		let r2 = Ray::new((r.start + self.transl) * self.scale, r.dir);
		self.orig.intersect(&r2, h)
	}
}

impl<O: Object> Bounded for TransformedObj<O> {
	fn bounds(&self) -> BoundingBox {
		todo!();
	}
}
