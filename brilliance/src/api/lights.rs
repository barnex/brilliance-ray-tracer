use crate::*;

pub fn point_light(pos: Point, intensity: Color) -> DynLight {
	DynLight::new(PointLight::new(pos, intensity))
}
