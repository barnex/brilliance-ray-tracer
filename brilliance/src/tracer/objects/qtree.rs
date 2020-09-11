use super::*;
use std::cmp::Ordering;

pub struct QTree<T> {
	bb4: BoundingBox4,
	ch: Children<T>,
}

enum Children<T> {
	Inner(Box<[QTree<T>; 4]>),
	Leafs(Vec<T>),
}

use Children::*;

impl<T: Bounded> QTree<T> {
	pub fn new(ch: Vec<T>) -> Self {
		assert!(ch.len() > 0);

		if ch.len() <= 4 {
			Self::build_leafs(ch)
		} else {
			Self::build_inner(ch)
		}
	}

	fn build_leafs(ch: Vec<T>) -> Self {
		assert!(ch.len() > 0 && ch.len() <= 4);
		let mut bbs = Vec::<BoundingBox>::with_capacity(4);
		for ch in &ch {
			bbs.push(ch.bounds())
		}
		while bbs.len() < 4 {
			// an empty box that does not increase the overall size
			bbs.push(BoundingBox::empty(ch[0].bounds().min))
		}

		Self {
			bb4: BoundingBox4::new([&bbs[0], &bbs[1], &bbs[2], &bbs[3]]),
			ch: Leafs(ch),
		}
	}

	fn build_inner(ch: Vec<T>) -> Self {
		let cut = Self::split4(ch.len());
		let (l, r) = Self::split_at(ch, cut[0] + cut[1]);
		let (a, b) = Self::split_at(l, cut[0]);
		let (c, d) = Self::split_at(r, cut[2]);

		let a = Self::new(a);
		let b = Self::new(b);
		let c = Self::new(c);
		let d = Self::new(d);

		Self {
			bb4: BoundingBox4::new([&a.bounds(), &b.bounds(), &c.bounds(), &d.bounds()]),
			ch: Inner(Box::new([a, b, c, d])),
		}
	}

	fn split4(n: usize) -> [usize; 4] {
		let a = nearest_pow4(n / 4);
		let b = nearest_pow4((n - a) / 3);
		let c = nearest_pow4((n - a - b) / 2);
		let d = n - a - b - c;
		let mut v = vec![a, b, c, d];
		v.sort();
		[v[0], v[1], v[2], v[3]]
	}

	fn split_at(mut ch: Vec<T>, i: usize) -> (Vec<T>, Vec<T>) {
		let bb = bounds_of_centers(&ch);
		let size = bb.max - bb.min;
		let splitdir = size.argmax();

		fn cmp(a: f32, b: f32) -> Ordering {
			if a < b {
				Ordering::Less
			} else {
				Ordering::Greater
			}
		}
		ch.sort_unstable_by(|a, b| cmp(a.bounds().center()[splitdir], b.bounds().center()[splitdir]));

		let right = ch.split_off(i);
		let left = ch;
		(left, right)
	}
}

//
fn bounds_of_centers<T: Bounded>(ch: &[T]) -> BoundingBox {
	let pt = ch[0].bounds().center();
	let mut bb = BoundingBox::empty(pt);
	for ch in ch {
		bb.add(ch.bounds().center())
	}
	bb
}

impl<T: Bounded> Bounded for QTree<T> {
	fn bounds(&self) -> BoundingBox {
		self.bb4.bounds()
	}
}

impl<T: Shape> Shape for QTree<T> {
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
		let inter = self.bb4.intersects_slow(r, h.t as f32);
		let mut hit = false;
		// TODO: could drill down into the nearest child first,
		// potentially return early because of occlusion.
		match &self.ch {
			Leafs(ch4) => {
				// TODO: skip bounds checks
				if inter[0] {
					hit |= ch4[0].intersect_coords(r, h);
				}
				if inter[1] {
					hit |= ch4[1].intersect_coords(r, h);
				}
				if inter[2] {
					hit |= ch4[2].intersect_coords(r, h);
				}
				if inter[3] {
					hit |= ch4[3].intersect_coords(r, h);
				}
			}
			Inner(ch4) => {
				if inter[0] {
					hit |= ch4[0].intersect_coords(r, h);
				}
				if inter[1] {
					hit |= ch4[1].intersect_coords(r, h);
				}
				if inter[2] {
					hit |= ch4[2].intersect_coords(r, h);
				}
				if inter[3] {
					hit |= ch4[3].intersect_coords(r, h);
				}
			}
		}
		hit
	}
}

impl<T: Object> Object for QTree<T> {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		let inter = self.bb4.intersects_slow(r, h.t() as f32);
		// TODO: could drill down into the nearest child first,
		// potentially return early because of occlusion.
		match &self.ch {
			Leafs(ch4) => {
				// TODO: skip bounds checks
				if inter[0] {
					ch4[0].intersect(r, h);
				}
				if inter[1] {
					ch4[1].intersect(r, h);
				}
				if inter[2] {
					ch4[2].intersect(r, h);
				}
				if inter[3] {
					ch4[3].intersect(r, h);
				}
			}
			Inner(ch4) => {
				if inter[0] {
					ch4[0].intersect(r, h);
				}
				if inter[1] {
					ch4[1].intersect(r, h);
				}
				if inter[2] {
					ch4[2].intersect(r, h);
				}
				if inter[3] {
					ch4[3].intersect(r, h);
				}
			}
		}
	}
}

fn nearest_pow4(n: usize) -> usize {
	nearest_pow(n as u32, 4) as usize
}

fn nearest_pow(n: u32, base: u32) -> u32 {
	let n = n as f64;
	let down = base.pow(f64::log(n, base as f64).floor() as u32);
	let up = base.pow(f64::log(n, base as f64).ceil() as u32);

	if (n - down as f64).abs() < (n - up as f64).abs() {
		down
	} else {
		up
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_nearest_pow2() {
		assert_eq!(nearest_pow(1, 2), 1);
		assert_eq!(nearest_pow(2, 2), 2);
		//assert_eq!(nearest_pow(3, 2), ?);
		assert_eq!(nearest_pow(4, 2), 4);
		assert_eq!(nearest_pow(5, 2), 4);
		//assert_eq!(nearest_pow(6, 2), ?);
		assert_eq!(nearest_pow(7, 2), 8);
		assert_eq!(nearest_pow(8, 2), 8);
		assert_eq!(nearest_pow(9, 2), 8);
		assert_eq!(nearest_pow(15, 2), 16);
		assert_eq!(nearest_pow(16, 2), 16);
		assert_eq!(nearest_pow(17, 2), 16);
	}

	#[test]
	fn test_nearest_pow4() {
		assert_eq!(nearest_pow(1, 2), 1);
		//assert_eq!(nearest_pow(2, 2), ?);
		assert_eq!(nearest_pow(3, 4), 4);
		assert_eq!(nearest_pow(4, 4), 4);
		assert_eq!(nearest_pow(5, 4), 4);
		assert_eq!(nearest_pow(6, 4), 4);
		assert_eq!(nearest_pow(7, 4), 4);
		assert_eq!(nearest_pow(9, 4), 4);
		assert_eq!(nearest_pow(12, 4), 16);
		assert_eq!(nearest_pow(15, 4), 16);
		assert_eq!(nearest_pow(16, 4), 16);
		assert_eq!(nearest_pow(17, 4), 16);
		assert_eq!(nearest_pow(40, 4), 64);
		assert_eq!(nearest_pow(64, 4), 64);
		assert_eq!(nearest_pow(65, 4), 64);
	}
}
