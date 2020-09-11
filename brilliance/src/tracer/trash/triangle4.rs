// use super::*;
//
// /// A set of 4 `Triangle`s.
// ///
// /// The 4 triangles can be tested for ray intersection using instruction-level parallelism (SSE),
// /// I.e, for the same CPU cost as a single ray-box intersection.
// #[derive(Clone)]
// pub struct Triangle4 {
// 	o: Vectorx4, // Point
// 	a: Vectorx4, // Vector
// 	b: Vectorx4, // Vector
// }
//
// impl Triangle4 {
// 	pub fn new(t: [&Triangle; 4]) -> Self {
// 		Self {
// 			o: Vectorx4::transpose(t[0].o.into(), t[1].o.into(), t[2].o.into(), t[3].o.into()),
// 			a: Vectorx4::transpose(t[0].a.into(), t[1].a.into(), t[2].a.into(), t[3].a.into()),
// 			b: Vectorx4::transpose(t[0].b.into(), t[1].b.into(), t[2].b.into(), t[3].b.into()),
// 		}
// 	}
//
// 	#[inline]
// 	pub fn intersects(&self, r: &Ray) -> [bool; 4] {
// 		let start = Vectorx4::broadcast(Pointf::from(r.start));
// 		let dir = Vectorx4::broadcast(Vectorf::from(r.dir));
// 		self.intersects1(start, dir)
// 	}
//
// 	#[inline]
// 	fn intersects1(&self, start: Vectorx4, dir: Vectorx4) -> [bool; 4] {
// 		let a = self.a;
// 		let b = self.b;
// 		let n = a.cross(b);
//
// 		let s = start - self.o;
// 		let t = -n.dot(s) / n.dot(dir);
// 		// TODO: possible early return if t < 0 || t > hitrecord.t
//
// 		let p = s + dir * t;
//
// 		// Barycentric coordinates for 3D triangle, after
// 		// Peter Shirley, Fundamentals of Computer Graphics, 2nd Edition.
// 		let nc = a.cross(p);
// 		let na = (b - a).cross(p - a);
// 		let n2 = n.dot(n);
// 		let l1 = n.dot(na) / n2;
// 		let l3 = n.dot(nc) / n2;
// 		let l2 = F32x4::from(1.0) - l1 - l3;
//
// 		//if !(l1 >= 0. && l2 >= 0. && l3 >= 0.) {
// 		//	// Note: `!(x>0)` handles NaN gracefully
// 		//	return false;
// 		//}
//
// 		let min = F32x4::min(F32x4::min(t, l1), F32x4::min(l2, l3)).array();
// 		[
// 			min[0] >= 0.0, // t >= 0 && l1 >= 0 && l2 >= 0 && l3 >= 0
// 			min[1] >= 0.0,
// 			min[2] >= 0.0,
// 			min[3] >= 0.0,
// 		]
// 	}
// }
//
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
