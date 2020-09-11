use super::*;

#[derive(Clone)]
pub struct Vertex {
	pub pos: Pointf,
	pub attr: Attr,
}

#[derive(Clone)]
pub struct Attr {
	pub shd_normal: Vectorf,
	pub tex_coords: TexCoords,
}

impl Attr {
	pub fn new(shd_normal: Vectorf, tex_coords: TexCoords) -> Self {
		Self { shd_normal, tex_coords }
	}

	pub fn default() -> Self {
		Self {
			shd_normal: Vectorf::default(),
			tex_coords: TexCoords::default(),
		}
	}
}
