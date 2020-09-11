use super::internal::*;
use super::*;

pub struct Reflective(pub Color);

impl Material for Reflective {
	fn shade(&self, s: &Scene, r: &Ray, h: &HitCoords, rng: &mut Rng, depth: u32) -> Color {
		let pos = r.at(h.t - TINY);
		let sec = Ray::new(pos, reflect(r.dir, h.shading_normal()));
		s.lightfield(&sec, rng, depth + 1) * self.0
	}
}

// reflect v along normal vector n.
//      n
//  v   |   reflected
//   \  |  /
//    \θ|θ/
//     \|/
//  --------- surface.
//
// See https://en.wikipedia.org/wiki/Ray_tracing_(graphics)#Example
fn reflect(v: Vector, n: Vector) -> Vector {
	v - 2.0 * v.dot(n) * n
}
