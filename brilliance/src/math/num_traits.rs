pub trait One {
	fn one() -> Self;
}

impl One for f32 {
	#[inline]
	fn one() -> Self {
		1.0
	}
}

impl One for f64 {
	#[inline]
	fn one() -> Self {
		1.0
	}
}

pub trait Recip {
	fn recip(self) -> Self;
}

impl Recip for f32 {
	#[inline]
	fn recip(self) -> Self {
		self.recip()
	}
}

impl Recip for f64 {
	#[inline]
	fn recip(self) -> Self {
		self.recip()
	}
}
