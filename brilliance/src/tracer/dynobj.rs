use super::*;

pub struct DynObj(pub Box<dyn Object>);

impl DynObj {
	pub fn new<O: Object + 'static>(obj: O) -> Self {
		Self(Box::new(obj))
	}
}

impl Object for DynObj {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>) {
		let inner: &dyn Object = self.0.borrow();
		inner.intersect(r, h)
	}
}

impl Bounded for DynObj {
	fn bounds(&self) -> BoundingBox {
		let inner: &dyn Object = self.0.borrow();
		inner.bounds()
	}
}
