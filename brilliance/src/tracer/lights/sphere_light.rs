use super::*;

pub fn sphere_light(pos: Point, diam: f64, power: Color) -> DynLight {
	let r = diam / 2.0;
	let brilliance = power / ((4.0 * PI * r * r) as f32);
	DynLight::new(WithObject::new(
		PointLight::new(pos, power), // TODO
		Sphere::new(pos, diam).paint(Flat::new(brilliance)),
	))
}
