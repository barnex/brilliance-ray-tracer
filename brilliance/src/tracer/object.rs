use super::*;

pub trait Object: Bounded + Send + Sync {
	fn intersect<'s>(&'s self, r: &Ray, h: &mut HitRecord<'s>);
}

pub trait Bounded {
	fn bounds(&self) -> BoundingBox;
}
