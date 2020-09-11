use super::*;

/// A HitRecord records the position and material where a Ray intersected an Object.
/// This allows for lazy evaluation of the Color seen by the Ray, after it has been
/// established that the color is actually going to be used
/// (i.e., the hit point is not occluded by another Object).
/// Without this lazy evaluation, the complexity of recursive ray tracing would
/// become exponential in the recursion depth.
pub struct HitRecord<'s> {
	pub coords: HitCoords,

	/// Material at intersection
	pub material: &'s dyn Material,
}

impl<'s> HitRecord<'s> {
	pub fn background(material: &'s dyn Material, dir: Vectorf) -> Self {
		Self {
			coords: HitCoords::background(dir),
			material,
		}
	}

	#[inline]
	pub fn t(&self) -> f64 {
		self.coords.t
	}

	/// is_valid is intended for use with debug_assert!
	pub fn is_valid(&self) -> bool {
		self.coords.is_valid()
	}

	#[inline]
	pub fn update_checked(&mut self, t: f64, geo_norm: Vectorf, shd_norm: Vectorf, tex_coords: Pointf, mat: &'s dyn Material) {
		if self.coords.update_checked(t, geo_norm, shd_norm, tex_coords) {
			self.material = mat;
		}
	}

	#[inline]
	pub fn update_unchecked_be_careful(&mut self, t: f64, geo_norm: Vectorf, shd_norm: Vectorf, tex_coords: Pointf, mat: &'s dyn Material) {
		self.coords.update_unchecked_be_careful(t, geo_norm, shd_norm, tex_coords);
		self.material = mat;
	}
}

#[derive(Debug)]
pub struct HitCoords {
	/// Position along the ray
	pub t: f64,

	//pub pos: Point,

	// TODO: Point?
	/// True, geometrical normal vector at intersection.
	/// (needs not have unit length).
	pub geom_normalf: Vectorf,

	/// Shading vector vector at intersection.
	/// (needs not have unit length).
	pub shading_normalf: Vectorf,
	// Local U,V coordinates at intersection, chosen by the Object.
	pub tex_coords: Pointf,
}

impl HitCoords {
	pub fn background(dir: Vectorf) -> Self {
		Self {
			t: INF,
			geom_normalf: dir,
			shading_normalf: dir,
			tex_coords: dir,
		}
	}

	#[inline]
	#[must_use]
	pub fn update_checked(&mut self, t: f64, geom_norm: Vectorf, shad_norm: Vectorf, tex_coords: Pointf) -> bool {
		debug_assert!(t.is_finite());
		debug_assert!(geom_norm.len() > 1e-5);
		debug_assert!(shad_norm.len() > 1e-5);
		if t >= 0.0 && t < self.t {
			self.update_unchecked_be_careful(t, geom_norm, shad_norm, tex_coords);
			true
		} else {
			false
		}
	}

	#[inline]
	pub fn update_unchecked_be_careful(&mut self, t: f64, geom_norm: Vectorf, shad_norm: Vectorf, tex_coords: Pointf) {
		debug_assert!(t >= 0.0);
		debug_assert!(t < self.t);
		self.t = t;
		self.geom_normalf = geom_norm;
		self.shading_normalf = shad_norm;
		self.tex_coords = tex_coords;
	}

	/// is_valid is intended for use with debug_assert!
	pub fn is_valid(&self) -> bool {
		let valid = self.t >= 0.0 && self.geom_normal().is_normalized() && self.shading_normal().is_normalized();
		//&& self.pos.is_finite();
		if !valid {
			println!("invalid: {:?}", &self);
		}
		valid
	}

	#[inline]
	pub fn geom_normal(&self) -> Vector {
		self.geom_normalf.normalized().into()
	}

	#[inline]
	pub fn shading_normal(&self) -> Vector {
		self.shading_normalf.normalized().into()
	}
}
