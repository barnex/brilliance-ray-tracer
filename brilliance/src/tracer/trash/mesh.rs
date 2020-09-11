use super::*;
//use std::cmp::Ordering;
//
//pub struct Mesh<T> {
//	center: Point,
//	//scale: f64,
//	mat: DynMaterial,
//	root: Node<T>,
//}
//
//impl Mesh<Face4> {
//	pub fn new(mat: DynMaterial, faces: Vec<Face>) -> Self {
//		//todo!("scale faces to unit size and center 0.0, use BB4c");
//		let root = build_tree(faces);
//		println!("leaf_histogram: {:?}", root.leaf_histogram());
//		Self {
//			center: Point(0.0, 0.0, 0.0), // TODO
//			//scale: 1.0,                   // TODO
//			mat,
//			root,
//		}
//	}
//}
//
//impl<T: Shape> Object for Mesh<T> {
//	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
//		let r = Ray {
//			start: r.start - self.center,
//			dir: r.dir,
//		};
//		if self.root.intersect_coords(&r, &mut h.coords) {
//			h.material = self.mat.borrow();
//		}
//	}
//}
//
//impl<T: Bounded> Bounded for Mesh<T> {
//	fn bounds(&self) -> BoundingBox {
//		self.root.bounds()
//	}
//}
//
//pub enum Node<T> {
//	Inner {
//		bb4: BoundingBox4,
//		ch4: Box<[Node<T>; 4]>,
//	},
//	Leaf(T), // TODO: probably better to point
//}
//
//use Node::*;
//
//impl<T: Bounded> Bounded for Node<T> {
//	fn bounds(&self) -> BoundingBox {
//		match &self {
//			Leaf(faces) => faces.bounds(),
//			Inner { ch4, .. } => ch4
//				.iter()
//				.skip(1)
//				.fold(ch4[0].bounds(), |prev, curr| prev.join(&curr.bounds())),
//		}
//	}
//}
//
//impl<T: Shape> Node<T> {
//	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
//		match &self {
//			Leaf(face4) => face4.intersect_coords(r, h),
//			Inner { bb4, ch4 } => {
//				let inter = bb4.intersects(r, h.t as f32);
//				let mut intersects = false;
//
//				if inter[0] {
//					intersects |= ch4[0].intersect_coords(r, h);
//				}
//				if inter[1] {
//					intersects |= ch4[1].intersect_coords(r, h);
//				}
//				if inter[2] {
//					intersects |= ch4[2].intersect_coords(r, h);
//				}
//				if inter[3] {
//					intersects |= ch4[3].intersect_coords(r, h);
//				}
//
//				intersects
//			}
//		}
//	}
//}
//
//impl Node<Face4> {
//	fn leaf_histogram(&self) -> [u32; 4] {
//		match &self {
//			Leaf(face4) => {
//				let mut h = [0, 0, 0, 0];
//				h[(face4.len() - 1) as usize] = 1;
//				h
//			}
//			Inner { ch4, .. } => Self::add_hist(
//				Self::add_hist(ch4[0].leaf_histogram(), ch4[1].leaf_histogram()),
//				Self::add_hist(ch4[2].leaf_histogram(), ch4[3].leaf_histogram()),
//			),
//		}
//	}
//
//	fn add_hist(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
//		[a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
//	}
//}
//
//fn build_tree(ch: Vec<Face>) -> Node<Face4> {
//	// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!     must be <= 4 for face4_sse
//	if ch.len() <= 3 {
//		return Leaf(Face4::new(ch));
//	}
//
//	let s = split4(ch.len());
//	let s0 = s[0] + s[1];
//	let s1 = s[0];
//	let s2 = s[2];
//	let (left, right) = split_at(ch, s0);
//	let (a, b) = split_at(left, s1);
//	let (c, d) = split_at(right, s2);
//
//	let a = build_tree(a);
//	let b = build_tree(b);
//	let c = build_tree(c);
//	let d = build_tree(d);
//
//	Inner {
//		bb4: BoundingBox4::new([&a.bounds(), &b.bounds(), &c.bounds(), &d.bounds()]),
//		ch4: Box::new([a, b, c, d]),
//	}
//}
//
//fn split4(n: usize) -> [usize; 4] {
//	let a = nearest_pow4(n / 4);
//	let b = nearest_pow4((n - a) / 3);
//	let c = nearest_pow4((n - a - b) / 2);
//	let d = n - a - b - c;
//
//	let mut v = vec![a, b, c, d];
//	v.sort();
//	[v[0], v[1], v[2], v[3]]
//}
//
//fn split_at(mut ch: Vec<Face>, i: usize) -> (Vec<Face>, Vec<Face>) {
//	let bb = faces_bounds(&ch);
//	let size = bb.max - bb.min;
//	let splitdir = size.argmax();
//
//	fn cmp(a: f32, b: f32) -> Ordering {
//		if a < b {
//			Ordering::Less
//		} else {
//			Ordering::Greater
//		}
//	}
//	ch.sort_unstable_by(|a, b| cmp(a.bounds().center()[splitdir], b.bounds().center()[splitdir]));
//
//	let right = ch.split_off(i);
//	let left = ch;
//	(left, right)
//}
//
//fn nearest_pow4(n: usize) -> usize {
//	nearest_pow(n as u32, 4) as usize
//}
//
//fn nearest_pow(n: u32, base: u32) -> u32 {
//	let n = n as f64;
//	let down = base.pow(f64::log(n, base as f64).floor() as u32);
//	let up = base.pow(f64::log(n, base as f64).ceil() as u32);
//
//	if (n - down as f64).abs() < (n - up as f64).abs() {
//		down
//	} else {
//		up
//	}
//}
//
//#[cfg(test)]
//mod test {
//	use super::*;
//
//	#[test]
//	fn test_split4() {
//		////assert_eq!(split4(4), [4, 0 , 0, 0]);
//		//assert_eq!(split4(5), [4, 1, 0, 0]);
//		//assert_eq!(split4(6), [4, 2, 0, 0]);
//		//assert_eq!(split4(7), [4, 3, 0, 0]);
//	}
//
//	#[test]
//	fn test_nearest_pow2() {
//		assert_eq!(nearest_pow(1, 2), 1);
//		assert_eq!(nearest_pow(2, 2), 2);
//		//assert_eq!(nearest_pow(3, 2), ?);
//		assert_eq!(nearest_pow(4, 2), 4);
//		assert_eq!(nearest_pow(5, 2), 4);
//		//assert_eq!(nearest_pow(6, 2), ?);
//		assert_eq!(nearest_pow(7, 2), 8);
//		assert_eq!(nearest_pow(8, 2), 8);
//		assert_eq!(nearest_pow(9, 2), 8);
//		assert_eq!(nearest_pow(15, 2), 16);
//		assert_eq!(nearest_pow(16, 2), 16);
//		assert_eq!(nearest_pow(17, 2), 16);
//	}
//
//	#[test]
//	fn test_nearest_pow4() {
//		assert_eq!(nearest_pow(1, 2), 1);
//		//assert_eq!(nearest_pow(2, 2), ?);
//		assert_eq!(nearest_pow(3, 4), 4);
//		assert_eq!(nearest_pow(4, 4), 4);
//		assert_eq!(nearest_pow(5, 4), 4);
//		assert_eq!(nearest_pow(6, 4), 4);
//		assert_eq!(nearest_pow(7, 4), 4);
//		assert_eq!(nearest_pow(9, 4), 4);
//		assert_eq!(nearest_pow(12, 4), 16);
//		assert_eq!(nearest_pow(15, 4), 16);
//		assert_eq!(nearest_pow(16, 4), 16);
//		assert_eq!(nearest_pow(17, 4), 16);
//		assert_eq!(nearest_pow(40, 4), 64);
//		assert_eq!(nearest_pow(64, 4), 64);
//		assert_eq!(nearest_pow(65, 4), 64);
//	}
//}
//
//fn faces_bounds(ch: &[Face]) -> BoundingBox {
//	ch.iter()
//		.skip(1)
//		.fold(ch[0].bounds(), |prev, curr| prev.join(&curr.bounds()))
//}
//
