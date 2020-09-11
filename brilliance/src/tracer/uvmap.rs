use super::*;

pub fn sphere_map(p: Pointf) -> Pointf {
	let (x, y, z) = (p[0] as f32, p[1] as f32, p[2] as f32);
	let u = 0.5 + (f32::atan2(z, x)) / (2.0 * PI as f32);
	let v = 0.5 - f32::asin(y) / (PI as f32); // sphere
	Pointf::new(u, v, 0.0)
}

pub fn cylinder_map(p: Pointf) -> Pointf {
	let (x, y, z) = (p[0] as f32, p[1] as f32, p[2] as f32);
	let u = 0.5 + (f32::atan2(z, x)) / (2.0 * PI as f32);
	let v = 0.5 - y / 2.0; // cylinder
	Pointf::new(u, v, 0.0)
}

type UVMap = fn(Pointf) -> Pointf;

pub struct UVMapped<T: Texture> {
	tex: T,
	uvmap: UVMap,
}

impl<T: Texture> UVMapped<T> {
	pub fn sphere(tex: T) -> Self {
		Self { tex, uvmap: sphere_map }
	}
	pub fn cylinder(tex: T) -> Self {
		Self { tex, uvmap: cylinder_map }
	}
}

impl<T: Texture> Texture for UVMapped<T> {
	fn color_at(&self, p: Pointf) -> Color {
		self.tex.color_at((self.uvmap)(p))
	}
}
