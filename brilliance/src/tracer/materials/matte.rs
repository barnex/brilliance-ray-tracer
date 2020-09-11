use super::internal::*;
use super::*;

// A material with Lambertian ("diffuse") reflectance.
// E.g.: plaster, paper, rubber.
//
// TODO: The texture determines the reflectivity in each point ("diffuse map").
// It should be between 0 and 1 (in each color channel), for physicaly possible materials.
//
// See https://en.wikipedia.org/wiki/Lambertian_reflectance.
//
// The ray tracing algorithm implemented here is a flavour of bidirectional path tracing:
// A ray is shot forward from the camera onto the scene.
// When it hits a matte surface, we gather the light from all light sources
// to give the direct illumination. To that we add the (appropriately weighted)
// contribution of one random ray. This gives the indirect illumination.
// The random ray's color is determined recurively, thus again
// taking into account all light sources, etc. (up to a maximum depth).
//
// E.g. in the sketch below, Ray a goes from the camera to a matte surface.
// At the intersection point we take into account the intensity of the light
// (properly checking for shadows, of course). To this we add the brightness seen by
// a properly chosen random ray b. This brightness again contains a contribution
// of the light source at the point where ray b intersects a matter surface,
// and so on.
//
//         light
//           | \
//           |  \  #
//           |   v #
//     cam   |   / #
//        \  |  /b #
//        a\ v /   #
//   ###############
//
// This separation of direct and indirect illumination causes significantly
// faster convergence for the common case of relatively small light sources.
pub struct Matte<T: Texture> {
	tex: T,
}

impl<T: Texture> Matte<T> {
	pub fn new(tex: T) -> Self {
		Self { tex }
	}
}

impl<T: Texture> Material for Matte<T> {
	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
		let mut acc = Color::BLACK;

		// If neccessary, flip the normal vectors to point towards the camera.
		let mut geo_norm = h.geom_normal();
		let mut shd_norm = h.shading_normal();
		if geo_norm.dot(r.dir) > 0.0 {
			geo_norm = -geo_norm;
			shd_norm = -shd_norm;
		}

		// Offset intersection point by a tiny amount so that the secondary rays
		// won't immediately intersect the surface they eminate from. This avoids "shadow acne"
		// (http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/#result---shadow-acne).
		let p = r.at(h.t) + TINY * geo_norm;

		for l in s.lights() {
			let (lpos, intens) = l.sample(rng, p);
			if intens == (Color::BLACK) {
				continue; // potential shortcut for directional lights
			}

			let ldelta = lpos - p;
			let ldir = ldelta.normalized();

			// angle between surface and light
			// use shading normal, but clip to geometric normal
			// in case shading normal points in opposite direction
			// (as can happen under grazing incidence)
			let costheta = ldir.dot(shd_norm);

			// not clear if this is needed:
			//if costheta <= 0.0 {
			//	costheta = ldir.dot(geo_norm);
			//}

			if costheta <= 0.0 {
				continue; // backlit
			}

			let sec = Ray::new(p, ldir);
			let ldist = ldelta.len();
			let intens = s.occlude(&sec, ldist, intens);
			acc += intens * (costheta as f32);
		}

		// ambient
		let (u, v) = rng.quasi_random2();
		let dir = cosine_sphere((u as f64, v as f64), shd_norm);
		let sec = Ray::new(p, dir);
		acc += s.lightfield_indirect(&sec, rng, depth + 1);

		acc * self.tex.color_at(h.tex_coords)
	}
}
