use super::*;

/// Axis Aligned Box (https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box).
/// Bounding boxes are combined into `BoundingBox4`s by `QTree` to accelerate Ray-Object intersection.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
	pub min: Pointf,
	pub max: Pointf,
}

impl BoundingBox {
	/// A BoundingBox enclosing points `min` and `max`.
	pub fn new(min: Pointf, max: Pointf) -> Self {
		debug_assert!(min[0] <= max[0]);
		debug_assert!(min[1] <= max[1]);
		debug_assert!(min[2] <= max[2]);
		Self { min, max }
	}

	pub fn from_points<'a, I: Iterator<Item = &'a Pointf>>(mut points: I) -> Self {
		let mut bb = Self::empty(*(points.next().expect("BoundingBox::from_points: called on 0 points, expected >= 1")));
		for pt in points {
			bb.add(*pt)
		}
		bb
	}

	/// An empty bounding box (`min == max`) at position `pos`.
	/// Used by QTree to represent empty nodes.
	pub fn empty(pos: Pointf) -> Self {
		Self { min: pos, max: pos }
	}

	/// Join bounding boxes `self` and `rhs` into a bounding box that encloses both.
	///
	///     use brilliance::*;
	///     let a = BoundingBox::new(Pointf( 1., 2., 3.), Pointf(4., 5., 6.));
	///     let b = BoundingBox::new(Pointf(-1., 3., 2.), Pointf(0., 4., 8.));
	///     assert_eq!(a.join(&b), BoundingBox::new(Pointf(-1., 2., 2.), Pointf(4., 5., 8.)));
	///
	pub fn join(&self, rhs: &Self) -> Self {
		Self {
			min: Pointf::min(self.min, rhs.min),
			max: Pointf::max(self.max, rhs.max),
		}
	}

	/// The bounding box's center.
	pub fn center(&self) -> Pointf {
		(self.min + self.max) * 0.5
	}

	/// TODO: replace by From<IntoIter<Pointf>>?
	pub fn add(&mut self, rhs: Pointf) {
		self.min = Pointf::min(self.min, rhs);
		self.max = Pointf::max(self.max, rhs);
	}
}

impl Bounded for BoundingBox {
	fn bounds(&self) -> BoundingBox {
		self.clone()
	}
}
