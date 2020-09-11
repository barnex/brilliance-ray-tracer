use super::*;

pub struct Quad {
	pos: [Pointf; 4]
	attr: [Attr; 4],
}

impl Quad {
	pub fn new(v: [Vertex; 4]) -> Self {
		Self {
			pos: [v[0].pos, v[1].pos, v[2].pos, v[3].pos],
			attr: [v[0].attr, v[1].attr, v[2].attr, v[3].attr],
		}
	}
}

impl Bounded for Face {
	fn bounds(&self) -> BoundingBox {
		BoundingBox::from(&self.pos)
	}
}

impl Shape for Quad {
	#[inline]
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {

		todo!();
		/*
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

		let shad_norm = self.attr[0].shd_normal * l1
			+ self.attr[1].shd_normal * l2
			+ self.attr[2].shd_normal * l3;

		h.update_unchecked(t64, n.into(), shad_norm);
		true
		*/
	}
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
