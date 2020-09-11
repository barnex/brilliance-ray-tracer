use super::*;
use std::sync::Arc;

pub trait Texture: Send + Sync + 'static {
	fn color_at(&self, p: Pointf) -> Color;
}

impl Texture for Color {
	fn color_at(&self, _: Pointf) -> Color {
		self.clone()
	}
}

impl Texture for Image<Color> {
	fn color_at(&self, p: Pointf) -> Color {
		self.at(nearest(p, self.dimensions()))
	}
}

impl Texture for Image<RGB> {
	fn color_at(&self, p: Pointf) -> Color {
		self.at(nearest(p, self.dimensions())).into()
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
