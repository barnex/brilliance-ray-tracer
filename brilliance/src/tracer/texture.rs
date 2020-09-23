use super::*;
use std::sync::Arc;

pub trait Texture: Send + Sync + 'static {
	// + Clone?
	fn color_at(&self, p: Pointf) -> Color;
}

impl Texture for Color {
	fn color_at(&self, _: Pointf) -> Color {
		self.clone()
	}
}

impl Texture for Image<Color> {
	fn color_at(&self, p: Pointf) -> Color {
		//self.at(nearest(p, self.dimensions()))
		bilinear(self, (p[0], p[1]))
	}
}

impl Texture for Image<RGB> {
	fn color_at(&self, p: Pointf) -> Color {
		//self.at(nearest(p, self.dimensions())).into()
		bilinear(&self, (p[0], p[1]))
	}
}

impl<T: Texture> Texture for Arc<T> {
	fn color_at(&self, p: Pointf) -> Color {
		let t: &T = self.borrow();
		t.color_at(p)
	}
}

impl Texture for Arc<dyn Texture> {
	fn color_at(&self, p: Pointf) -> Color {
		let t: &dyn Texture = self.borrow();
		t.color_at(p)
	}
}

fn bilinear<C: Into<Color> + Copy + Default>(img: &Image<C>, (u, v): (f32, f32)) -> Color {
	let (w, h) = img.dimensions();

	let X = warp(u) * (w - 1) as f32;
	let Y = warp(v) * (h - 1) as f32;
	let x0 = X as u32;
	let y0 = Y as u32;

	let mut x1 = x0 + 1;
	if x1 >= w {
		x1 = 0;
	}
	let mut y1 = y0 + 1;
	if y1 >= h {
		y1 = 0;
	}
	let x = X % 1.0;
	let y = Y % 1.0;

	let c00: Color = img.at((x0, y0)).into();
	let c01: Color = img.at((x0, y1)).into();
	let c10: Color = img.at((x1, y0)).into();
	let c11: Color = img.at((x1, y1)).into();

	Color::new(
		bilin(c00.r(), c01.r(), c10.r(), c11.r(), x, y),
		bilin(c00.g(), c01.g(), c10.g(), c11.g(), x, y),
		bilin(c00.b(), c01.b(), c10.b(), c11.b(), x, y),
	)
}

fn bilin(f00: f32, f01: f32, f10: f32, f11: f32, x: f32, y: f32) -> f32 {
	f00 * (1.0 - x) * (1.0 - y) + f10 * x * (1.0 - y) + f01 * (1.0 - x) * y + f11 * x * y
}

fn nearest(p: Pointf, dim: (u32, u32)) -> (u32, u32) {
	let (u, v) = (warp(p[0]), warp(p[1]));
	let (w, h) = (dim.0, dim.1);
	let i = (u * (w as f32 - 1.0) + 0.5) as u32;
	let j = (v * (h as f32 - 1.0) + 0.5) as u32;
	(i, j)
}

fn warp(x: f32) -> f32 {
	let mut x = x % 1.0;
	if x < 0.0 {
		x = 1.0 + x;
	}
	if x == 1.0 {
		x = 0.0
	}
	x
}

pub struct Transformed<T: Texture> {
	scale: (f32, f32),
	pan: (f32, f32),
	inner: T,
}

impl<T: Texture> Transformed<T> {
	pub fn pan(inner: T, pan: (f32, f32)) -> Self {
		Self {
			inner,
			pan,
			scale: (1.0, 1.0),
		}
	}

	pub fn scale(inner: T, (scale_u, scale_v): (f32, f32)) -> Self {
		Self {
			inner,
			pan: (0.0, 0.0),
			scale: (1.0 / scale_u, 1.0 / scale_v),
		}
	}
}

impl<T: Texture> Texture for Transformed<T> {
	fn color_at(&self, p: Pointf) -> Color {
		let scale = Pointf(self.scale.0, self.scale.1, 1.0);
		let pan = Pointf(self.pan.0, self.pan.1, 0.0);
		self.inner.color_at(p * scale - pan)
	}
}
