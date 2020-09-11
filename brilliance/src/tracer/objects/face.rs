use super::*;

//#[derive(Clone)]
pub struct Face {
	o: Pointf,
	a: Vectorf,
	b: Vectorf,
	attr: [Attr; 3],
}

impl Face {
	pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
		Self {
			o: a.pos,
			a: b.pos - a.pos,
			b: c.pos - a.pos,
			attr: [a.attr, b.attr, c.attr],
		}
	}
	/// Vertex 0.
	pub fn o(&self) -> Pointf {
		self.o
	}
	/// Side 1 (Vertex1 - Vertex0).
	pub fn a(&self) -> Vectorf {
		self.a
	}
	/// Side 2 (Vertex2 - Vertex0).
	pub fn b(&self) -> Vectorf {
		self.b
	}

	/// Vertex 1.
	pub fn v1(&self) -> Pointf {
		self.o + self.a
	}

	/// Vertex 2.
	pub fn v2(&self) -> Pointf {
		self.o + self.b
	}

	/// Vertix attributes.
	pub fn attrs(&self) -> &[Attr; 3] {
		&self.attr
	}
}

impl Bounded for Face {
	fn bounds(&self) -> BoundingBox {
		BoundingBox::from_points([self.o, self.v1(), self.v2()].iter())
	}
}

impl Shape for Face {
	#[inline]
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
		// TODO: this is mostly done in f64 to avoid bleeding.
		// could offset the ray to start close to bounding box (if outside)
		// to mitigate this.

		let a = self.a;
		let b = self.b;
		let o = self.o;

		let start = r.start;
		let dir = r.dir;

		let a: Vector = a.into();
		let b: Vector = b.into();

		let n = a.cross(b);
		let o: Point = o.into();

		let s = start - o;
		let t64 = -n.dot(s) / n.dot(dir);
		let n2 = n.dot(n);

		// handles NaN gracefully
		if !(t64 > 0.0 && t64 < h.t) {
			return false;
		}

		let p = s + t64 * dir;
		let invn2 = 1.0 / n2;

		// Barycentric coordinates for 3D triangle, after
		// Peter Shirley, Fundamentals of Computer Graphics, 2nd Edition.
		let nc = a.cross(p);
		let nb = p.cross(b);
		let l3 = n.dot(nc) * invn2;
		let l2 = n.dot(nb) * invn2;
		let l1 = 1. - l2 - l3;

		if f64::min(f64::min(l1, l2), l3) < 0.0 {
			return false;
		}

		let l1 = l1 as f32;
		let l2 = l2 as f32;
		let l3 = l3 as f32;

		let shad_norm = self.attr[0].shd_normal * l1 + self.attr[1].shd_normal * l2 + self.attr[2].shd_normal * l3;

		let u = self.attr[0].tex_coords.u_f32() * l1 + self.attr[1].tex_coords.u_f32() * l2 + self.attr[2].tex_coords.u_f32() * l3;
		let v = self.attr[0].tex_coords.v_f32() * l1 + self.attr[1].tex_coords.v_f32() * l2 + self.attr[2].tex_coords.v_f32() * l3;
		let tex_coords = Pointf::new(u, v, 0.0);

		h.update_checked(t64, n.into(), shad_norm, tex_coords);
		true
	}

	//  #[inline]
	//  pub fn intersect_coords_fast(&self, r: &Ray, hc: &mut HitCoords) -> bool {
	//  	let start: Pointf = r.start.into();
	//  	let dir: Pointf = r.dir.into();
	//  	let v0 = self.vertex[0].pos;
	//  	let v1 = self.vertex[1].pos;
	//  	let v2 = self.vertex[2].pos;
	//  	let edge1 = v1 - v0;
	//  	let edge2 = v2 - v0;
	//  	let h = dir.cross(edge2);
	//  	let a = edge1.dot(h);
	//  	//    if (a > -EPSILON && a < EPSILON)
	//  	//        return false;    // This ray is parallel to this triangle.
	//  	let f = 1.0 / a;
	//  	let s = start - v0;
	//  	let q = s.cross(edge1);
	//  	let u = f * s.dot(h);
	//  	let v = f * dir.dot(q);
	//  	if (u < 0.0 || u > 1.0) || (v < 0.0 || u + v > 1.0) {
	//  		return false;
	//  	}
	//  	// At this stage we can compute t to find out where the intersection point is on the line.
	//  	let t = f * edge2.dot(q);
	//  	//if t >= 0.0 {}
	//  	let n = edge1.cross(edge2);
	//  	hc.update_checked(t as f64, n, n)
	//  	//    if (t > EPSILON) // ray intersection
	//  	//    {
	//  	//        outIntersectionPoint = rayOrigin + rayVector * t;
	//  	//        return true;
	//  	//    }
	//  	//    else // This means that there is a line intersection but not a ray intersection.
	//  	//        return false;
	//  	//}
	//  	//todo!();
	//  }
}

// #[cfg(test)]
// mod test {
// 	use super::*;
//
// 	const EZ: Vector = Vector::EZ;
//
// 	/*
//
// 			 * (3,4)
// 			/|
// 		   / |
// 		  /  |
// 	(1,2)*---* (3,2)
//
// 	*/
// 	#[test]
// 	fn intersects() {
// 		let t = Triangle::new(Point(1., 2., -1.), Point(3., 2., -1.), Point(3., 4., -1.));
//
// 		assert!(!t.intersects(&Ray(Point(0., 0., 0.,), -EZ)));
// 		assert!(!t.intersects(&Ray(Point(0., 0., 0.,), EZ)));
//
// 		assert!(t.intersects(&Ray(Point(2., 3., 0.,), -EZ)));
// 		assert!(!t.intersects(&Ray(Point(2., 3., 0.,), EZ)));
//
// 		assert!(!t.intersects(&Ray(Point(4., 3., 0.,), -EZ)));
// 		assert!(!t.intersects(&Ray(Point(4., 3., 0.,), EZ)));
// 		assert!(!t.intersects(&Ray(Point(2., -3., 0.,), -EZ)));
// 		assert!(!t.intersects(&Ray(Point(2., -3., 0.,), EZ)));
//
// 		assert!(!t.intersects(&Ray(Point(0., 0., -2.,), EZ)));
// 		assert!(!t.intersects(&Ray(Point(0., 0., -2.,), -EZ)));
//
// 		assert!(t.intersects(&Ray(Point(2., 3., -2.,), EZ)));
// 		assert!(!t.intersects(&Ray(Point(2., 3., -2.,), -EZ)));
// 	}
// }
//
