use super::*;

/// 2D Texture coordinates (between 0.0 and 1.0, inclusive).
/// Compressed to 16 bit precision.
#[derive(Debug, Copy, Clone, Default)]
pub struct TexCoords {
	u: u16,
	v: u16,
}

impl TexCoords {
	pub fn new(u: f32, v: f32) -> Self {
		Self {
			u: compress(u),
			v: compress(v),
		}
	}

	pub fn u_f32(self) -> f32 {
		uncompress(self.u)
	}

	pub fn v_f32(self) -> f32 {
		uncompress(self.v)
	}
}

impl Into<Pointf> for TexCoords {
	fn into(self) -> Pointf {
		Pointf::new(uncompress(self.u), uncompress(self.v), 0.0)
	}
}

impl From<(f64, f64)> for TexCoords {
	fn from((u, v): (f64, f64)) -> Self {
		Self::new(u as f32, v as f32)
	}
}

fn compress(x: f32) -> u16 {
	//if x < 0.0 || x > 1.0 {
	//	panic!("invalid texture coordinate: {}", x)
	//}
	(x * 65536.0) as u16
}

fn uncompress(x: u16) -> f32 {
	(x as f32) / 65536.0
}
