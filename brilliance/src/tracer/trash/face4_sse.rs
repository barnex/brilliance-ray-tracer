// use super::*;
//
// pub struct Face4 {
// 	o: Vectorx4,
// 	a: Vectorx4,
// 	b: Vectorx4,
// 	attrs: [[Attr; 3]; 4],
// 	len: u8,
// }
//
// impl Face4 {
// 	pub fn new(mut ch: Vec<Face>) -> Self {
// 		//print!("{} ", ch.len());
// 		assert!(ch.len() <= 4);
// 		assert!(ch.len() > 0);
//
// 		let len = ch.len() as u8;
// 		// append dummy faces, making sure they do not extend the bounding box
// 		let dum = ch[0].vertex[0].clone(); // a point in the bounding box
// 		while ch.len() < 4 {
// 			ch.push(Face::new(dum.clone(), dum.clone(), dum.clone()));
// 		}
//
// 		Self {
// 			o: Vectorx4::transpose(ch[0].o(), ch[1].o(), ch[2].o(), ch[3].o()),
// 			a: Vectorx4::transpose(ch[0].a(), ch[1].a(), ch[2].a(), ch[3].a()),
// 			b: Vectorx4::transpose(ch[0].b(), ch[1].b(), ch[2].b(), ch[3].b()),
// 			attrs: [
// 				ch[0].attrs().clone(),
// 				ch[1].attrs().clone(),
// 				ch[2].attrs().clone(),
// 				ch[3].attrs().clone(),
// 			],
// 			len,
// 		}
// 	}
// 	pub fn len(&self) -> u8 {
// 		self.len
// 	}
// }
//
// impl Shape for Face4 {
// 	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
// 		let start = Vectorx4::broadcast(r.start.into());
// 		let dir = Vectorx4::broadcast(r.dir.into());
//
// 		let o = self.o;
// 		let a = self.a;
// 		let b = self.b;
//
// 		let n = a.cross(b);
// 		let s = start - o;
// 		let t = -n.dot(s) / n.dot(dir);
//
// 		// handles NaN gracefully
// 		//if !(t64 > 0.0 && t64 < h.t) {
// 		//return false;
// 		//}
//
// 		let p = s + dir * t;
// 		let n2 = n.dot(n);
// 		let invn2 = n2.approx_recip();
//
// 		// Barycentric coordinates for 3D triangle, after
// 		// Peter Shirley, Fundamentals of Computer Graphics, 2nd Edition.
// 		let na = (b - a).cross(p - a);
// 		let nc = a.cross(p);
// 		let nb = p.cross(b);
// 		let l3 = n.dot(nc) * invn2;
// 		let l1 = n.dot(na) * invn2;
// 		let l2 = n.dot(nb) * invn2;
// 		//let l2 = F32x4::from(1.0) - l1 - l3;
//
// 		// TODO: re-order to minimize dependency on l2?
// 		//let mins = F32x4::min(F32x4::min(l1, l3), F32x4::min(l2, t));
//
// 		let min1 = F32x4::min(l1, l3);
// 		let min2 = F32x4::min(l2, t);
//
// 		let mins = F32x4::min(min1, min2);
//
// 		// TODO: also check for h.t, early return?
//
// 		// find minimum positive t
// 		let mins = mins.array();
// 		let ts = t.array();
//
// 		let mut i = 666;
// 		let mut t = INF32;
//
// 		if mins[0] > 0.0 {
// 			// && ts[0] > 0.0 {
// 			t = ts[0];
// 			i = 0;
// 		}
// 		if mins[1] > 0.0 && ts[1] < t {
// 			// && ts[1] > 0.0
// 			t = ts[1];
// 			i = 1;
// 		}
// 		if mins[2] > 0.0 && ts[2] < t {
// 			// && ts[2] > 0.0
// 			t = ts[2];
// 			i = 2;
// 		}
// 		if mins[3] > 0.0 && ts[3] < t {
// 			// && ts[3] > 0.0
// 			t = ts[3];
// 			i = 3;
// 		}
//
// 		if i == 666 {
// 			return false;
// 		}
//
// 		//TODO: re-calculate t, as f64
// 		//let start = r.start;
// 		//let dir = r.dir;
// 		//let o: Point = self.vertex[0].pos.into();
// 		//let v1: Point = self.vertex[1].pos.into();
// 		//let v2: Point = self.vertex[2].pos.into();
// 		//let a: Vector = v1 - o;
// 		//let b: Vector = v2 - o;
// 		//let n = a.cross(b);
// 		//let s = start - o;
// 		//let t64 = -n.dot(s) / n.dot(dir);
//
// 		let attr = &self.attrs[i];
// 		let shad_norm = attr[0].shd_normal * l1.index(i)
// 			+ attr[1].shd_normal * l2.index(i)
// 			+ attr[2].shd_normal * l3.index(i);
//
// 		// TODO: f64
// 		//h.update_unchecked(t as f64, n.row(i), shad_norm);
// 		h.update_checked(t as f64, n.row(i), shad_norm)
// 	}
//
// 	fn bounds(&self) -> BoundingBox {
// 		let ini = self.o.row(0);
// 		let mut bb = BoundingBox::new(ini, ini);
// 		let v0 = self.o;
// 		let v1 = self.o + self.a;
// 		let v2 = self.o + self.b;
// 		for i in 0..4 {
// 			bb.add(v0.row(i));
// 			bb.add(v1.row(i));
// 			bb.add(v2.row(i));
// 		}
// 		bb
// 	}
// }
//
