// use super::*;
//
// /// A set of 4 `BoundingBox`es.
// ///
// /// The 4 boxes can be tested for ray intersection using instruction-level parallelism (SSE),
// /// I.e, for the same CPU cost as a single ray-box intersection.
// #[derive(Clone, Debug)]
// pub struct BoundingBox4c {
// 	min: Vector16x4, // 4 x's, 4 y's, 4 z's
// 	max: Vector16x4,
// }
//
// type Vector16x4 = [[i16; 4]; 3];
//
// impl BoundingBox4c {
// 	/// Construct a set of 4 bounding boxes from 4 indivual boxes.
// 	pub fn new(bb: [&BoundingBox; 4]) -> Self {
// 		Self {
// 			min: Self::transpose(bb[0].min, bb[1].min, bb[2].min, bb[3].min),
// 			max: Self::transpose(bb[0].max, bb[1].max, bb[2].max, bb[3].max),
// 		}
// 	}
//
// 	#[inline]
// 	fn transpose(a: Vectorf, b: Vectorf, c: Vectorf, d: Vectorf) -> Vector16x4 {
// 		[
// 			Self::compress(a[0], b[0], c[0], d[0]),
// 			Self::compress(a[1], b[1], c[1], d[1]),
// 			Self::compress(a[2], b[2], c[2], d[2]),
// 		]
// 	}
//
// 	#[inline]
// 	fn compress(a: f32, b: f32, c: f32, d: f32) -> [i16; 4] {
// 		[
// 			F16::compress(a),
// 			F16::compress(b),
// 			F16::compress(c),
// 			F16::compress(d),
// 		]
// 	}
//
// 	#[inline]
// 	fn decompress(x: Vector16x4) -> Vectorx4 {
// 		Vectorx4::new(
// 			Self::decompress4(x[0]),
// 			Self::decompress4(x[1]),
// 			Self::decompress4(x[2]),
// 		)
// 	}
//
// 	#[inline]
// 	fn decompress4(x: [i16; 4]) -> F32x4 {
// 		F32x4::new(
// 			F16::decompress(x[0]),
// 			F16::decompress(x[1]),
// 			F16::decompress(x[2]),
// 			F16::decompress(x[3]),
// 		)
// 	}
//
// 	/// Test each of the for 4 bounding boxes for ray intersection,
// 	/// accelerated by vector instructions.
// 	///
// 	/// The 4 returned booleans correspond to intersection with the 4 boxes
// 	/// originally passed to `BoundingBox4c::new`.
// 	#[inline]
// 	pub fn intersects(&self, r: &Ray) -> [bool; 4] {
// 		let start = Vectorf::from(r.start);
// 		let invdir = Vectorf::from(r.dir).inv();
// 		self.intersects1(start, invdir)
// 	}
//
// 	#[inline]
// 	fn intersects1(&self, start: Pointf, invdir: Vectorf) -> [bool; 4] {
// 		let start0 = F32x4::from(start[0]);
// 		let start1 = F32x4::from(start[1]);
// 		let start2 = F32x4::from(start[2]);
//
// 		let invdir0 = F32x4::from(invdir[0]);
// 		let invdir1 = F32x4::from(invdir[1]);
// 		let invdir2 = F32x4::from(invdir[2]);
//
// 		let min = Self::decompress(self.min);
// 		let max = Self::decompress(self.max);
//
// 		let tmin0 = (min[0] - start0) * invdir0;
// 		let tmin1 = (min[1] - start1) * invdir1;
// 		let tmin2 = (min[2] - start2) * invdir2;
//
// 		let tmax0 = (max[0] - start0) * invdir0;
// 		let tmax1 = (max[1] - start1) * invdir1;
// 		let tmax2 = (max[2] - start2) * invdir2;
//
// 		let ten0 = F32x4::min(tmin0, tmax0);
// 		let ten1 = F32x4::min(tmin1, tmax1);
// 		let ten2 = F32x4::min(tmin2, tmax2);
//
// 		let tex0 = F32x4::max(tmin0, tmax0);
// 		let tex1 = F32x4::max(tmin1, tmax1);
// 		let tex2 = F32x4::max(tmin2, tmax2);
//
// 		let ten = F32x4::max(F32x4::max(ten0, ten1), ten2);
// 		let tex = F32x4::min(F32x4::min(tex0, tex1), tex2).array();
//
// 		let reten = F32x4::max(F32x4::from(0.), ten).array();
//
// 		[
// 			tex[0] >= reten[0],
// 			tex[1] >= reten[1],
// 			tex[2] >= reten[2],
// 			tex[3] >= reten[3],
// 		]
// 	}
// }
//
// #[cfg(test)]
// mod test {
// 	use super::*;
//
// 	const EX: Vector = Vector::EX;
// 	const EY: Vector = Vector::EY;
// 	const EZ: Vector = Vector::EZ;
//
// 	// Test intersection of 4 random-ish `BoundingBox4c`s with a some random-ish rays.
// 	// Check that the result is the same as individual tests against 4 `BoundingBox`s
// 	// (which is well-tested)
// 	#[test]
// 	fn intersect4() {
// 		let bb0 = BoundingBox::new(Pointf(-0.1, -0.2, -0.3), Pointf(0.2, 0.3, 0.4));
// 		let bb1 = BoundingBox::new(Pointf(0.1, 0.2, 0.3), Pointf(0.2, 0.3, 0.4));
// 		let bb2 = BoundingBox::new(Pointf(-0.1, 0.2, 0.3), Pointf(0.2, 0.3, 0.4));
// 		let bb3 = BoundingBox::new(Pointf(-0.1, -0.2, -0.3), Pointf(-0.1, 0.3, 0.4)); // degenerate
//
// 		let bb4 = BoundingBox4c::new([&bb0, &bb1, &bb2, &bb3]);
//
// 		let starts = [
// 			Point(-0.15, 0.0, 0.25),
// 			Point(0.35, -0.45, 0.25),
// 			Point(0.0, -0.05, 0.05),
// 			Point(0.55, 0.65, 0.75),
// 		];
// 		let dirs = [
// 			EX,
// 			EY,
// 			EZ,
// 			-EX,
// 			-EY,
// 			-EZ,
// 			Vector(1., 1., 0.).normalized(),
// 			Vector(-1., 1., 0.).normalized(),
// 			Vector(0., 1., 1.).normalized(),
// 			Vector(1., 1., 1.).normalized(),
// 			Vector(-1., 1., -1.).normalized(),
// 		];
//
// 		for start in &starts {
// 			for dir in &dirs {
// 				let r = Ray(*start, *dir);
//
// 				let want = [
// 					bb0.intersects(&r),
// 					bb1.intersects(&r),
// 					bb2.intersects(&r),
// 					bb3.intersects(&r),
// 				];
//
// 				let got = bb4.intersects(&r);
//
// 				assert_eq!(got, want);
// 			}
// 		}
// 	}
// }
//
//use std::convert::{From, Into};
//
//pub struct F16(i16);
//
//impl F16 {
//	pub fn compress(x: f32) -> i16 {
//		if x < -0.5 || x > 0.5 {
//			panic!("F16: out of range: {}", x)
//		}
//		(x * 65534.0) as i16
//	}
//
//	pub fn decompress(x: i16) -> f32 {
//		(x as f32) / 65534.0
//	}
//}
//
//impl From<f32> for F16 {
//	fn from(x: f32) -> Self {
//		Self(Self::compress(x))
//	}
//}
//
//impl Into<f32> for F16 {
//	fn into(x: f32) -> Self {
//		Self(Self::compress(x))
//	}
//}
