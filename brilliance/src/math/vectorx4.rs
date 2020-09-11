use super::*;

/// A 3-component vector whose elements (x, y, z) are 4 packed floats (`F32x4`).
/// Conceptually, a `Vectorx4` represents a set of 4 vectors (i.e, `[Vectorf; 4]`),
/// but the internal representation is such that SSE instructions can be used efficiently.
///
/// Thus, calling vector operations (e.g, `dot`, `cross`,...) performs the operation
/// on 4 vectors separately and in parallel.
pub type Vectorx4 = Vec3<F32x4>;

impl Vectorx4 {
	/// Construct a `Vectorx4` from 4 vectors.
	/// TODO: From([Vectorf; 4])
	#[inline]
	pub fn transpose(a: Vectorf, b: Vectorf, c: Vectorf, d: Vectorf) -> Self {
		Self::new(
			F32x4::new(a[0], b[0], c[0], d[0]),
			F32x4::new(a[1], b[1], c[1], d[1]),
			F32x4::new(a[2], b[2], c[2], d[2]),
		)
	}

	/// Construct a `Vectorx4` holding the same vector `v` 4 times.
	/// I.e., the equivalent of `transpose(v, v, v, v)`.
	#[inline]
	pub fn broadcast(v: Vectorf) -> Self {
		Self::new(F32x4::from(v[0]), F32x4::from(v[1]), F32x4::from(v[2]))
	}

	/// Row i returns the i'th Vectorf packed inside this Vectorx4.
	/// i must be in range 0..4.
	pub fn row(&self, i: usize) -> Vectorf {
		Vectorf(self[0].index(i), self[1].index(i), self[2].index(i))
	}
}
