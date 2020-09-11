// use super::*;
//
// #[derive(Clone)]
// pub struct Triangle {
// 	pub o: Point,
// 	pub a: Vector,
// 	pub b: Vector,
// }
//
// impl Triangle {
// 	pub fn new(a: Point, b: Point, c: Point) -> Self {
// 		//debug_assert!((b - a).cross(c - a).len() > 1e-6);
// 		{
// 			let s = (b - a).cross(c - a).len();
// 			if s < 1e-6 {
// 				panic!("bad triangle: {}, {}, {}", a, b, c)
// 			}
// 		}
// 		Self {
// 			o: a,
// 			a: b - a,
// 			b: c - a,
// 		}
// 	}
//
// 	#[inline]
// 	pub fn intersects(&self, r: &Ray) -> bool {
// 		self.intersect_t(r) >= 0.0
// 	}
//
// 	pub fn v0(&self) -> Point {
// 		self.o
// 	}
// 	pub fn v1(&self) -> Point {
// 		self.o + self.a
// 	}
// 	pub fn v2(&self) -> Point {
// 		self.o + self.b
// 	}
//
// 	/// Bounding box around all vertices.
// 	///
// 	///     use brilliance::*;
// 	///     let t = Triangle::new(
// 	///         Point(0., 0., 3.,),
// 	///         Point(1., 0., 3.,),
// 	///         Point(0., 1., 3.,),
// 	///     );
// 	///     assert_eq!(
// 	///         t.bounds(),
// 	///         BoundingBox::new(
// 	///             Pointf(0., 0., 3.),
// 	///             Pointf(1., 1., 3.),
// 	///         )
// 	///     );
// 	pub fn bounds(&self) -> BoundingBox {
// 		let a = Pointf::from(self.v0());
// 		let b = Pointf::from(self.v1());
// 		let c = Pointf::from(self.v2());
// 		BoundingBox::new(
// 			Pointf::min(Pointf::min(a, b), c),
// 			Pointf::max(Pointf::max(a, b), c),
// 		)
// 	}
//
// 	#[inline]
// 	pub fn intersect_t(&self, r: &Ray) -> f64 {
// 		//let start = Pointf::from(r.start);
// 		//let dir = Vectorf::from(r.dir);
//
// 		let a = self.a;
// 		let b = self.b;
//
// 		let n = a.cross(b);
//
// 		let s = r.start - self.o;
// 		let t = -n.dot(s) / n.dot(r.dir);
//
// 		if t < 0.0 {
// 			return -1.0;
// 		}
// 		// TODO: possible early return if t < 0 || t > hitrecord.t
//
// 		let p = s + t * r.dir;
//
// 		// Barycentric coordinates for 3D triangle, after
// 		// Peter Shirley, Fundamentals of Computer Graphics, 2nd Edition.
// 		let nc = a.cross(p);
// 		let na = (b - a).cross(p - a);
// 		//let nb = p.cross(b);
// 		let n2 = n.dot(n);
// 		let l1 = n.dot(na) / n2;
// 		let l3 = n.dot(nc) / n2;
// 		//let l2 = n.dot(nb) / n2;
// 		let l2 = 1. - l1 - l3;
//
// 		//if !(l1 >= 0. && l2 >= 0. && l3 >= 0.) {
// 		//	// Note: `!(x>0)` handles NaN gracefully
// 		//	return false;
// 		//}
//
// 		//t >= 0. && l1 >= 0. && l2 >= 0. && l3 >= 0.
// 		if f64::min(f64::min(l1, l2), l3) < 0.0 {
// 			//if f64::min(f64::min(l1, l2), l3) < -(1.0 / (1024.0 * 1024.0)) {
// 			return -1.0;
// 		}
// 		t
// 	}
//
// 	pub fn normal(&self) -> Vectorf {
// 		self.a.cross(self.b).into()
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
