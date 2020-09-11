use super::*;

// TODO: this is not SIMD accellerated
// TODO: is really a Face1-3
pub struct Face4 {
	ch: [Face; 3],
	len: u8,
}

impl Face4 {
	pub fn new(mut ch: Vec<Face>) -> Self {
		assert!(ch.len() <= 3);
		assert!(ch.len() > 0);

		let len = ch.len() as u8;
		// append dummy faces, making sure they do not extend the bounding box
		let dum = Vertex {
			pos: ch[0].o(),
			attr: Attr::default(),
		};
		while ch.len() < 3 {
			ch.push(Face::new(dum.clone(), dum.clone(), dum.clone()));
		}

		Self {
			ch: [ch.remove(0), ch.remove(0), ch.remove(0)],
			len,
		}
	}

	pub fn len(&self) -> u8 {
		self.len
	}
}

impl Shape for Face4 {
	#[inline]
	fn intersect_coords(&self, r: &Ray, h: &mut HitCoords) -> bool {
		let mut intersects = false;
		for i in 0..self.len {
			intersects |= self.ch[i as usize].intersect_coords(r, h);
		}
		intersects
	}
}

impl Bounded for Face4 {
	fn bounds(&self) -> BoundingBox {
		bounds_of(&self.ch)
	}
}
