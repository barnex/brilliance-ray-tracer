use super::*;

pub struct Scene {
	pub objects: Vec<DynObj>, // TODO: QTree<DynObj>
	pub lights: Vec<DynLight>,
	pub ambient: Color,
	pub background: DynMaterial,
	pub max_recursion_depth: u32,
	pub max_iter: u32,
}

impl Scene {
	pub fn image_fn(&self, rng: &mut Rng, c: &Camera, uv: (f64, f64)) -> Color {
		let ray = &c.ray_from(rng, uv);
		self.lightfield(ray, rng, 0)
	}

	pub fn lights(&self) -> &[DynLight] {
		&self.lights
	}

	pub fn lightfield(&self, r: &Ray, rng: &mut Rng, depth: u32) -> Color {
		debug_assert!(r.is_valid());

		if depth > self.max_recursion_depth {
			return self.ambient;
		}

		let mut h = HitRecord::background(&self.background, r.dir.into());

		for o in &self.objects {
			o.intersect(r, &mut h);
			debug_assert!(h.is_valid());
		}

		for o in &self.lights {
			o.intersect(r, &mut h);
			debug_assert!(h.is_valid());
		}

		// Object.intersect does not need to normalize normals to unit length,
		// so do it here.
		h.coords.geom_normalf.normalize();
		h.coords.shading_normalf.normalize();

		h.material.shade(self, r, &h.coords, rng, depth)
	}

	pub fn lightfield_indirect(&self, r: &Ray, rng: &mut Rng, depth: u32) -> Color {
		self.lightfield(r, rng, depth) // TODO
	}

	pub fn occlude(&self, r: &Ray, len: f64, orig: Color) -> Color {
		let mut occluded = orig;

		let background = Flat::new(Color::BLACK);
		let mut h = HitRecord::background(&background, r.dir.into());
		for o in &self.objects {
			o.intersect(r, &mut h);
			debug_assert!(h.is_valid());
			if h.coords.t < len {
				occluded = h.material.occlude(occluded, r.at(h.t()));
				if occluded == Color::BLACK {
					return occluded;
				}
			}
		}

		occluded
	}
}
