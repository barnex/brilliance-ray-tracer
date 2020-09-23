use super::internal::*;
use super::*;

pub struct Translucent<T: Texture, S: Texture> {
	transmitted: T,
	scattered: S,
}

impl<T: Texture, S: Texture> Translucent<T, S> {
	pub fn new(transmitted: T, scattered: S) -> Self {
		Self { transmitted, scattered }
	}
}

impl<T: Texture, S: Texture> Material for Translucent<T, S> {
	fn occlude(&self, orig: Color, pos: Point) -> Color {
		orig * self.transmitted.color_at(pos.into())
	}

	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
		let mut acc = Color::BLACK;

		let p = r.at(h.t + 2.0 * TINY);

		// transmitted
		{
			let transm = Ray::new(p, r.dir);
			acc += self.transmitted.color_at(p.into()) * s.lightfield(&transm, rng, depth); // no need to increase depth, ray direction unchanged
		}

		// scattered
		for l in s.lights() {
			let (lpos, intens) = l.sample(rng, p);
			if intens == (Color::BLACK) {
				continue; // potential shortcut for directional lights
			}

			let ldelta = lpos - p;
			let ldir = ldelta.normalized();

			let sec = Ray::new(p, ldir);
			let ldist = ldelta.len();
			let intens = s.occlude(&sec, ldist, intens);
			acc += intens * self.scattered.color_at(p.into());
		}

		acc
	}
}
