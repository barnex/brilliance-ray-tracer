use super::*;

/// A set of 4 `BoundingBox`es.
///
/// The 4 boxes can be tested for ray intersection using instruction-level parallelism (SSE),
/// I.e, for the same CPU cost as a single ray-box intersection.
#[derive(Clone, Debug)]
pub struct BoundingBox4 {
	pub min: Vectorx4,
	pub max: Vectorx4,
}

impl BoundingBox4 {
	/// Construct a set of 4 bounding boxes from 4 indivual boxes.
	pub fn new(bb: [&BoundingBox; 4]) -> Self {
		Self {
			min: Vectorx4::transpose(bb[0].min, bb[1].min, bb[2].min, bb[3].min),
			max: Vectorx4::transpose(bb[0].max, bb[1].max, bb[2].max, bb[3].max),
		}
	}

	/// Test each of the for 4 bounding boxes for ray intersection,
	/// accelerated by vector instructions.
	///
	/// The 4 returned booleans correspond to intersection with the 4 boxes
	/// originally passed to `BoundingBox4::new`.
	#[inline]
	pub fn intersects_slow(&self, r: &Ray, max: f32) -> [bool; 4] {
		let start = Vectorf::from(r.start);
		let invdir = Vectorf::from(r.dir).inv();
		self.intersects_fast(start, invdir, max)
	}

	#[inline]
	pub fn intersects_fast(&self, start: Pointf, invdir: Vectorf, max: f32) -> [bool; 4] {
		let start0 = F32x4::from(start[0]);
		let start1 = F32x4::from(start[1]);
		let start2 = F32x4::from(start[2]);

		let invdir0 = F32x4::from(invdir[0]);
		let invdir1 = F32x4::from(invdir[1]);
		let invdir2 = F32x4::from(invdir[2]);

		let tmin0 = (self.min[0] - start0) * invdir0;
		let tmin1 = (self.min[1] - start1) * invdir1;
		let tmin2 = (self.min[2] - start2) * invdir2;

		let tmax0 = (self.max[0] - start0) * invdir0;
		let tmax1 = (self.max[1] - start1) * invdir1;
		let tmax2 = (self.max[2] - start2) * invdir2;

		let ten0 = F32x4::min(tmin0, tmax0);
		let ten1 = F32x4::min(tmin1, tmax1);
		let ten2 = F32x4::min(tmin2, tmax2);

		let tex0 = F32x4::max(tmin0, tmax0);
		let tex1 = F32x4::max(tmin1, tmax1);
		let tex2 = F32x4::max(tmin2, tmax2);

		let ten = F32x4::max(F32x4::max(ten0, ten1), ten2);
		let tex = F32x4::min(F32x4::min(tex0, tex1), tex2).array();

		let reten = F32x4::max(F32x4::from(0.), ten).array();
		let ten = ten.array();

		[
			tex[0] >= reten[0] && ten[0] < max,
			tex[1] >= reten[1] && ten[1] < max,
			tex[2] >= reten[2] && ten[2] < max,
			tex[3] >= reten[3] && ten[3] < max,
		]
	}
}

impl Bounded for BoundingBox4 {
	fn bounds(&self) -> BoundingBox {
		let xmin = self.min[0].hmin();
		let ymin = self.min[1].hmin();
		let zmin = self.min[2].hmin();
		let xmax = self.max[0].hmax();
		let ymax = self.max[1].hmax();
		let zmax = self.max[2].hmax();
		BoundingBox::new(Pointf(xmin, ymin, zmin), Pointf(xmax, ymax, zmax))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EX: Vector = Vector::EX;
	const EY: Vector = Vector::EY;
	const EZ: Vector = Vector::EZ;

	impl BoundingBox {
		pub fn intersects(&self, r: &Ray) -> bool {
			let start = Pointf::from(r.start);
			let invdir = Vectorf::from(r.dir).inv();

			let tmin = (self.min - start) * invdir;
			let tmax = (self.max - start) * invdir;

			let ten = Vectorf::min(tmin, tmax);
			let tex = Vectorf::max(tmin, tmax);

			let ten = f32::max(f32::max(ten[0], ten[1]), ten[2]);
			let tex = f32::min(f32::min(tex[0], tex[1]), tex[2]);

			// `>=` aims to cover the degenerate case where
			// the box has size 0 along a dimension
			// (e.g. when wrapping an axis-aligned rectangle).
			tex >= f32::max(0.0, ten)
		}
	}

	#[allow(non_snake_case)]
	fn Ray(start: Point, dir: Vector) -> Ray {
		Ray::new(start, dir)
	}

	#[test]
	fn intersect() {
		let min = Pointf(1.0, 2.0, 3.0);
		let max = Pointf(2.0, 5.0, 6.0);
		let bb = BoundingBox::new(min, max);

		/*
			Cases with the ray along X:

			<-(14)  (13)->     <-(16) (15)->   <-(18) (17)->

							  +-----------+(2,5,6)
							  |           |
							  |           |
			<-(2)  (1)->      |<-(4) (3)->|  <-(6) (5)->
							  |           |
							  |           |
					   (1,2,3)+-----------+

			<-(8)  (7)->       <-(9) (10)->   <-(12) (11)->
		*/
		assert!(bb.intersects(&Ray(Point(0.0, 3.0, 4.0), EX))); //   (1)
		assert!(!bb.intersects(&Ray(Point(0.0, 3.0, 4.0), -EX))); // (2)
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), EX))); //   (3)
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), -EX))); //  (4)
		assert!(!bb.intersects(&Ray(Point(2.5, 3.0, 4.0), EX))); //  (5)
		assert!(bb.intersects(&Ray(Point(2.5, 3.0, 4.0), -EX))); //  (6)

		// as above, but shifted down (Y) to miss the box.
		assert!(!bb.intersects(&Ray(Point(0.0, -1.0, 4.0), EX))); // (7)
		assert!(!bb.intersects(&Ray(Point(0.0, -1.0, 4.0), -EX))); //(8)
		assert!(!bb.intersects(&Ray(Point(1.5, -1.0, 4.0), EX))); // (9)
		assert!(!bb.intersects(&Ray(Point(1.5, -1.0, 4.0), -EX))); //(10)
		assert!(!bb.intersects(&Ray(Point(2.5, -1.0, 4.0), EX))); // (11)
		assert!(!bb.intersects(&Ray(Point(2.5, -1.0, 4.0), -EX))); //(12)

		// as above, but shifted up (Y) to miss the box.
		assert!(!bb.intersects(&Ray(Point(0.0, 6.0, 4.0), EX))); // (13)
		assert!(!bb.intersects(&Ray(Point(0.0, 6.0, 4.0), -EX))); //(14)
		assert!(!bb.intersects(&Ray(Point(1.5, 6.0, 4.0), EX))); // (15)
		assert!(!bb.intersects(&Ray(Point(1.5, 6.0, 4.0), -EX))); //(16)
		assert!(!bb.intersects(&Ray(Point(2.5, 6.0, 4.0), EX))); // (17)
		assert!(!bb.intersects(&Ray(Point(2.5, 6.0, 4.0), -EX))); //(18)

		/*
			Cases with the ray along Y:

								   ^
								   |
								  (6)
								  (5)
								   |
								   v
							  +-----------+(2,5,6)
							  |    ^      |
							  |    |      |
							  |   (3)     |
							  |   (4)     |
							  |    |      |
							  |    v      |
					   (1,2,3)+-----------+
									^
									|
								   (1)
								   (2)
									|
									v

		*/
		assert!(bb.intersects(&Ray(Point(1.5, 1.0, 4.0), EY))); //   (1)
		assert!(!bb.intersects(&Ray(Point(1.5, 1.0, 4.0), -EY))); // (2)
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), EY))); //   (3)
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), -EY))); //  (4)
		assert!(bb.intersects(&Ray(Point(1.5, 6.0, 4.0), -EY))); //  (5)
		assert!(!bb.intersects(&Ray(Point(1.5, 6.0, 4.0), EY))); //  (6)

		// as above, but shifted left to miss the box
		assert!(!bb.intersects(&Ray(Point(0.5, 1.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(0.5, 1.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(0.5, 3.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(0.5, 3.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(0.5, 6.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(0.5, 6.0, 4.0), EY)));

		// as above, but shifted right to miss the box
		assert!(!bb.intersects(&Ray(Point(3.0, 1.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 1.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 3.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 3.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 6.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 6.0, 4.0), EY)));

		// as above, but shifted right to miss the box
		assert!(!bb.intersects(&Ray(Point(3.0, 1.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 1.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 3.0, 4.0), EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 3.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 6.0, 4.0), -EY)));
		assert!(!bb.intersects(&Ray(Point(3.0, 6.0, 4.0), EY)));

		// Similar cases with the ray along Z:
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 2.0), EZ)));
		assert!(!bb.intersects(&Ray(Point(1.5, 3.0, 2.0), -EZ)));
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), EZ)));
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 4.0), -EZ)));
		assert!(bb.intersects(&Ray(Point(1.5, 3.0, 7.0), -EZ)));
		assert!(!bb.intersects(&Ray(Point(1.5, 3.0, 7.0), EZ)));

		// as above, but shifted to miss the box
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 2.0), EZ)));
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 2.0), -EZ)));
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 4.0), EZ)));
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 4.0), -EZ)));
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 7.0), -EZ)));
		assert!(!bb.intersects(&Ray(Point(-1.0, 3.0, 7.0), EZ)));
	}

	#[test]
	fn degenerate() {
		// Corner case: bounding box with size zero in one dimension.
		// It should still work (e.g.: this may happen when bounding a 2D shape).
		let bb = BoundingBox::new(Pointf(-1., -1., 0.), Pointf(1., 1., 0.));
		assert!(bb.intersects(&Ray(Point(0., 0., 1.), -EZ)));
	}

	// Test intersection of 4 random-ish `BoundingBox4`s with a some random-ish rays.
	// Check that the result is the same as individual tests against 4 `BoundingBox`s
	// (which is well-tested)
	#[test]
	fn intersect4() {
		let bb0 = BoundingBox::new(Pointf(-1., -2., -3.), Pointf(2., 3., 4.));
		let bb1 = BoundingBox::new(Pointf(1., 2., 3.), Pointf(2., 3., 4.));
		let bb2 = BoundingBox::new(Pointf(-1., 2., 3.), Pointf(2., 3., 4.));
		let bb3 = BoundingBox::new(Pointf(-1., -2., -3.), Pointf(-1., 3., 4.)); // degenerate

		let bb4 = BoundingBox4::new([&bb0, &bb1, &bb2, &bb3]);

		let starts = [Point(-1.5, 0., 2.5), Point(3.5, -4.5, 2.5), Point(0., -0.5, 0.5), Point(5.5, 6.5, 7.5)];
		let dirs = [
			EX,
			EY,
			EZ,
			-EX,
			-EY,
			-EZ,
			Vector(1., 1., 0.).normalized(),
			Vector(-1., 1., 0.).normalized(),
			Vector(0., 1., 1.).normalized(),
			Vector(1., 1., 1.).normalized(),
			Vector(-1., 1., -1.).normalized(),
		];

		for start in &starts {
			for dir in &dirs {
				let r = Ray::new(*start, *dir);

				let want = [bb0.intersects(&r), bb1.intersects(&r), bb2.intersects(&r), bb3.intersects(&r)];

				let got = bb4.intersects_slow(&r, INF32);

				assert_eq!(got, want);
			}
		}
	}
}
