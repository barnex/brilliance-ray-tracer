use super::*;

pub struct Sphere {
	origin: Point,
	r2: f64, // radius squared
}

impl Sphere {
	pub fn new(origin: Point, diam: f64) -> Self {
		let r = diam / 2.0;
		Self { origin, r2: r * r }
	}

	fn tex_coords(&self, p: Point) -> Pointf {
		let p = (p - self.origin).normalized();
		sphere_map(p.into())
	}
}

impl Bounded for Sphere {
	fn bounds(&self) -> BoundingBox {
		let r = self.r2.sqrt() as f32;
		let o = self.origin.into();
		BoundingBox::new(Pointf(-r, -r, -r) + o, Pointf(r, r, r) + o)
	}
}

impl Shape for Sphere {
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
		let v = r.start - self.origin;
		let d = r.dir;
		let vd = v.dot(d);
		let discr = vd * vd - (v.len2() - self.r2);
		if discr < 0.0 {
			return false;
		}
		let sqrtd = discr.sqrt();
		let t1 = -vd - sqrtd;
		let t2 = -vd + sqrtd;

		let mut t = t1;
		if t < 0.0 {
			t = t2
		}
		if t > 0.0 {
			let p = r.at(t);
			let n = (p - self.origin).normalized().into();
			return h.update_checked(t, n, n, self.tex_coords(p));
		}
		false
	}
}
